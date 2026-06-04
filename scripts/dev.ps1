<#
SQL Admin 一键全栈调试脚本
==========================

功能：
根据模式选择启动 Web 全栈或桌面应用：
- web:    构建前端 + 启动后端 (Axum，内含前端静态文件服务)
- web-dev: 启动后端 (Axum) + 前端 (Trunk) 开发服务器（热重载）
- desktop: 启动 Tauri 2 桌面应用

使用方法：
.\scripts\dev.ps1                       (默认 desktop 模式)
.\scripts\dev.ps1 -Mode web             (Web 全栈模式，单进程)
.\scripts\dev.ps1 -Mode web-dev         (Web 开发模式，前后端热重载)
.\scripts\dev.ps1 -Mode desktop         (桌面模式)
#>

param(
    [ValidateSet("web", "web-dev", "desktop")]
    [string]$Mode = "desktop"
)

if (-not (Test-Path "Cargo.toml")) {
    Write-Host "[ERROR] Please run from project root" -ForegroundColor Red
    exit 1
}

if ($Mode -eq "desktop") {
    Write-Host ""
    Write-Host "Starting in DESKTOP mode..." -ForegroundColor Green
    Write-Host ""

    & .\scripts\start-desktop.ps1

} elseif ($Mode -eq "web-dev") {
    # -------------------------------------------------------
    # Web Dev Mode: Frontend (Trunk) + Backend (Axum) hot-reload
    # -------------------------------------------------------
    Write-Host ""
    Write-Host "Starting in WEB-DEV mode (hot-reload)..." -ForegroundColor Green
    Write-Host "  Backend  -> http://localhost:3000" -ForegroundColor Gray
    Write-Host "  Frontend -> http://localhost:8080" -ForegroundColor Gray
    Write-Host ""

    # Start backend in background
    $backendJob = Start-Job -ScriptBlock {
        param($root)
        Set-Location $root
        & powershell -File ".\scripts\start-backend.ps1"
    } -ArgumentList $PWD.Path

    # Wait for backend to be ready
    Write-Host "[WAIT] Waiting for backend to start..." -ForegroundColor Yellow
    $maxWait = 30
    $waited = 0
    while ($waited -lt $maxWait) {
        Start-Sleep -Seconds 1
        $waited++
        try {
            $response = Invoke-WebRequest -Uri "http://localhost:3000/health" -TimeoutSec 2 -ErrorAction Stop
            if ($response.StatusCode -eq 200) {
                Write-Host "  Backend is ready (${waited}s)" -ForegroundColor Green
                break
            }
        } catch {
            # Backend not ready yet
        }
    }
    if ($waited -ge $maxWait) {
        Write-Host "  [WARN] Backend did not respond within ${maxWait}s, starting frontend anyway" -ForegroundColor Yellow
    }

    # Start frontend (blocking)
    try {
        & .\scripts\start-frontend.ps1
    } finally {
        # Stop backend when frontend exits
        Write-Host ""
        Write-Host "[CLEANUP] Stopping backend..." -ForegroundColor Yellow
        Stop-Job $backendJob -ErrorAction SilentlyContinue
        Remove-Job $backendJob -Force -ErrorAction SilentlyContinue
    }

} else {
    # -------------------------------------------------------
    # Web Mode: Build frontend, then start Axum with static files
    # -------------------------------------------------------
    Write-Host ""
    Write-Host "Starting in WEB mode (single-process)..." -ForegroundColor Green
    Write-Host ""

    # Step 1: Build frontend WASM
    Write-Host "[BUILD] Building frontend..." -ForegroundColor Yellow
    Push-Location crates/frontend
    try {
        # Build CSS first
        try {
            npm run build:css 2>&1 | Out-Null
            Write-Host "  CSS build completed" -ForegroundColor Gray
        } catch {
            Write-Host "  [WARN] CSS build failed, continuing without styles" -ForegroundColor Yellow
        }

        # Build WASM with trunk
        trunk build 2>&1
        if ($LASTEXITCODE -ne 0) {
            Write-Host "[FAIL] Frontend build failed" -ForegroundColor Red
            Pop-Location
            exit $LASTEXITCODE
        }
        Write-Host "  Frontend build completed -> crates/frontend/dist/" -ForegroundColor Green
    } finally {
        Pop-Location
    }

    # Step 2: Set FRONTEND_DIST and start Axum (serves both API + static files)
    $env:FRONTEND_DIST = "crates/frontend/dist"

    if (-not (Test-Path ".env")) {
        $env:DATABASE_URL = "sqlite:data/sql_admin.sqlite3?mode=rwc"
        $env:APP_ENV = "dev"
        $env:LOG_LEVEL = "info"
        $env:RUST_LOG = "sql_admin=info,axum=info"
    } else {
        Get-Content ".env" | ForEach-Object {
            $line = $_.Trim()
            if ($line -and -not $line.StartsWith("#")) {
                $parts = $line -split "=", 2
                if ($parts.Count -eq 2) {
                    $key = $parts[0].Trim()
                    $value = $parts[1].Trim()
                    [Environment]::SetEnvironmentVariable($key, $value, "Process")
                }
            }
        }
    }

    if (-not (Test-Path env:ENCRYPTION_KEY)) {
        $env:ENCRYPTION_KEY = "LiteAdmin2026DevEncryptionKey!!32b"
    }
    if (-not (Test-Path env:SERVER_ADDR)) {
        $env:SERVER_ADDR = "0.0.0.0:3000"
    }

    $dataDir = "./data"
    if (-not (Test-Path $dataDir)) {
        New-Item -ItemType Directory -Path $dataDir | Out-Null
    }

    Write-Host ""
    Write-Host "==========================================" -ForegroundColor Cyan
    Write-Host "  SQL Admin - Web (single-process)" -ForegroundColor Cyan
    Write-Host "  http://$($env:SERVER_ADDR)" -ForegroundColor Green
    Write-Host "  API:  /api/*" -ForegroundColor Gray
    Write-Host "  Static files from: $env:FRONTEND_DIST" -ForegroundColor Gray
    Write-Host "  Press Ctrl+C to stop" -ForegroundColor Gray
    Write-Host "==========================================" -ForegroundColor Cyan
    Write-Host ""

    cargo run -p sql-admin-interfaces
}
