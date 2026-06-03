<#
SQL Admin 前端本地调试脚本
==========================

功能：
1. 检查工具链依赖（trunk, wasm32 target）
2. 启动前端开发服务器（Trunk + Leptos）

说明：
- 前端通过 Trunk.toml 中配置的 proxy 将 /api 请求转发到后端
- 确保后端已启动（默认 http://localhost:3000）

使用方法：
.\scripts\start-frontend.ps1
.\scripts\start-frontend.ps1 -Port 8080
#>

param(
    [string]$Port = "8080"
)

Write-Host "==========================================" -ForegroundColor Cyan
Write-Host "  SQL Admin - Frontend Dev Server" -ForegroundColor Cyan
Write-Host "==========================================" -ForegroundColor Cyan
Write-Host ""

if (-not (Test-Path "Cargo.toml")) {
    Write-Host "[ERROR] Please run from project root" -ForegroundColor Red
    exit 1
}

Write-Host "[CHECK] Checking toolchain..." -ForegroundColor Yellow

$allOk = $true

try {
    $trunkVersion = trunk --version 2>&1
    Write-Host "  trunk: $trunkVersion" -ForegroundColor Gray
} catch {
    Write-Host "  [FAIL] trunk not found. Run: cargo install trunk" -ForegroundColor Red
    $allOk = $false
}

$wasmTarget = rustup target list --installed 2>&1 | Select-String "wasm32-unknown-unknown"
if ($wasmTarget) {
    Write-Host "  wasm32-unknown-unknown: installed" -ForegroundColor Gray
} else {
    Write-Host "  [FAIL] wasm32-unknown-unknown not installed. Run: rustup target add wasm32-unknown-unknown" -ForegroundColor Red
    $allOk = $false
}

if (-not $allOk) {
    Write-Host ""
    Write-Host "[ERROR] Toolchain check failed" -ForegroundColor Red
    exit 1
}

Write-Host ""
Write-Host "[CHECK] Checking port availability..." -ForegroundColor Yellow

$portCheck = netstat -ano | Select-String ":$Port\s"
if ($portCheck) {
    $processId = ($portCheck -split '\s+')[-1]
    Write-Host "  [WARN] Port $Port is already in use by process ID $processId" -ForegroundColor Yellow
    Write-Host "  Attempting to stop the conflicting process..." -ForegroundColor Yellow
    try {
        taskkill /f /pid $processId | Out-Null
        Start-Sleep -Milliseconds 1000
        Write-Host "  [OK] Process $processId terminated" -ForegroundColor Green
    } catch {
        Write-Host "  [FAIL] Failed to terminate process $processId" -ForegroundColor Red
        Write-Host "  Please manually stop the process using port $Port" -ForegroundColor Red
        exit 1
    }
} else {
    Write-Host "  Port $Port is available" -ForegroundColor Gray
}

Write-Host ""
Write-Host "[CSS] Building Tailwind CSS (initial)..." -ForegroundColor Yellow
Push-Location crates/frontend
try {
    npm run build:css 2>&1 | Out-Null
    if ($LASTEXITCODE -ne 0) {
        Write-Host "  [WARN] CSS build failed, continuing without styles" -ForegroundColor Yellow
    } else {
        Write-Host "  CSS build completed" -ForegroundColor Gray
    }
} catch {
    Write-Host "  [WARN] npm not available, skipping CSS build" -ForegroundColor Yellow
}
Pop-Location

Write-Host ""
Write-Host "[CSS] Starting Tailwind CSS watcher..." -ForegroundColor Yellow
Push-Location crates/frontend
try {
    $cssProcess = Start-Process -FilePath "npm" -ArgumentList "run", "watch:css" -PassThru -NoNewWindow -RedirectStandardOutput "$env:TEMP\sql-admin-css-watch.log" -RedirectStandardError "$env:TEMP\sql-admin-css-watch-err.log"
    Write-Host "  CSS watcher started (PID: $($cssProcess.Id))" -ForegroundColor Gray
} catch {
    Write-Host "  [WARN] npm not available, CSS will not auto-rebuild" -ForegroundColor Yellow
    $cssProcess = $null
}
Pop-Location

Write-Host ""
Write-Host "==========================================" -ForegroundColor Cyan
Write-Host "  Frontend: http://localhost:$Port" -ForegroundColor Green
Write-Host "  Hot-reload: enabled (trunk + CSS watch)" -ForegroundColor Green
Write-Host "  API proxy: /api    -> http://localhost:3000/api" -ForegroundColor Gray
Write-Host "  Health:    /health -> http://localhost:3000/health" -ForegroundColor Gray
Write-Host "  Press Ctrl+C to stop" -ForegroundColor Gray
Write-Host "==========================================" -ForegroundColor Cyan
Write-Host ""

try {
    $env:RUST_LOG = "warn"
    Push-Location crates/frontend
    trunk serve --port $Port
} finally {
    Pop-Location
    if ($cssProcess -and -not $cssProcess.HasExited) {
        Write-Host "  Stopping CSS watcher..." -ForegroundColor Yellow
        Stop-Process -Id $cssProcess.Id -Force -ErrorAction SilentlyContinue
    }
}

Write-Host ""
Write-Host "Frontend stopped." -ForegroundColor Cyan