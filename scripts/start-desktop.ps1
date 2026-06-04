<#
SQL Admin 桌面应用本地调试脚本
==============================

功能：
1. 检查工具链依赖（cargo-tauri, trunk, wasm32 target）
2. 编译并启动 Tauri 2 桌面应用（含前端热重载）

说明：
- 桌面模式不需要后端服务，所有 API 通过 Tauri IPC 调用
- 前端通过 --features tauri 编译，启用 Tauri 双模式 API
- SQLite 数据库存储在系统 AppData 目录（由 Tauri 自动管理）
- 首次启动 WASM 编译较慢（5-10 分钟），后续增量编译较快

使用方法：
.\scripts\start-desktop.ps1
.\scripts\start-desktop.ps1 -Release
#>

param(
    [switch]$Release
)

Write-Host "==========================================" -ForegroundColor Cyan
Write-Host "  SQL Admin - Desktop Dev Mode" -ForegroundColor Cyan
Write-Host "==========================================" -ForegroundColor Cyan
Write-Host ""

if (-not (Test-Path "Cargo.toml")) {
    Write-Host "[ERROR] Please run from project root" -ForegroundColor Red
    exit 1
}

# ---- Toolchain Check ----
Write-Host "[CHECK] Checking toolchain..." -ForegroundColor Yellow

$allOk = $true

try {
    $tauriVersion = cargo tauri --version 2>&1
    Write-Host "  cargo-tauri: $tauriVersion" -ForegroundColor Gray
} catch {
    Write-Host "  [FAIL] cargo-tauri not found. Run: cargo install tauri-cli --version `"^2`"" -ForegroundColor Red
    $allOk = $false
}

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

# ---- CSS Build ----
Write-Host ""
Write-Host "[CSS] Building Tailwind CSS..." -ForegroundColor Yellow
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

# ---- Build Check ----
Write-Host ""
Write-Host "[BUILD] Pre-checking desktop crate..." -ForegroundColor Yellow
cargo check -p sql-admin-desktop
if ($LASTEXITCODE -ne 0) {
    Write-Host "[FAIL] Desktop crate check failed" -ForegroundColor Red
    exit $LASTEXITCODE
}
Write-Host "  Desktop crate OK" -ForegroundColor Green

Write-Host ""
Write-Host "[BUILD] Pre-checking frontend (tauri feature)..." -ForegroundColor Yellow
cargo check -p sql-admin-frontend --features tauri --target wasm32-unknown-unknown
if ($LASTEXITCODE -ne 0) {
    Write-Host "[FAIL] Frontend check failed" -ForegroundColor Red
    exit $LASTEXITCODE
}
Write-Host "  Frontend crate OK" -ForegroundColor Green

# ---- Launch ----
Write-Host ""
Write-Host "==========================================" -ForegroundColor Cyan
Write-Host "  Mode:    Desktop (Tauri 2)" -ForegroundColor Green
Write-Host "  Feature: tauri (IPC mode)" -ForegroundColor Green
Write-Host "  DB:      SQLite (AppData auto-managed)" -ForegroundColor Green
if ($Release) {
    Write-Host "  Profile: release" -ForegroundColor Green
}
Write-Host "  Press Ctrl+C to stop" -ForegroundColor Gray
Write-Host "==========================================" -ForegroundColor Cyan
Write-Host ""

try {
    Push-Location crates/desktop
    if ($Release) {
        cargo tauri dev --release
    } else {
        cargo tauri dev
    }
} finally {
    Pop-Location
}

Write-Host ""
Write-Host "Desktop app stopped." -ForegroundColor Cyan
