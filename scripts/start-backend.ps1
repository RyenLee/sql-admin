<#
SQL Admin 后端本地调试脚本
==========================

功能：
1. 加载 .env 环境变量
2. 创建数据目录
3. 编译并启动后端服务（Axum）

使用方法：
.\scripts\start-backend.ps1
.\scripts\start-backend.ps1 -ServerAddr "127.0.0.1:3000"
#>

param(
    [string]$ServerAddr = "0.0.0.0:3000"
)

Write-Host "==========================================" -ForegroundColor Cyan
Write-Host "  SQL Admin - Backend Dev Server" -ForegroundColor Cyan
Write-Host "==========================================" -ForegroundColor Cyan
Write-Host ""

if (-not (Test-Path "Cargo.toml")) {
    Write-Host "[ERROR] Please run from project root" -ForegroundColor Red
    exit 1
}

Write-Host "[ENV] Loading environment..." -ForegroundColor Yellow

$defaultEncryptionKey = "LiteAdmin2026DevEncryptionKey!!32b"

if (Test-Path ".env") {
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
    Write-Host "  Loaded .env file" -ForegroundColor Gray
} else {
    Write-Host "  .env not found, using defaults" -ForegroundColor Gray
    $env:DATABASE_URL = "sqlite:data/sql_admin.sqlite3?mode=rwc"
    $env:APP_ENV = "dev"
    $env:LOG_LEVEL = "debug"
    $env:RUST_LOG = "sql_admin=debug,axum=info"
}

if (-not (Test-Path env:ENCRYPTION_KEY)) {
    $env:ENCRYPTION_KEY = $defaultEncryptionKey
}
if (-not (Test-Path env:SERVER_ADDR)) {
    $env:SERVER_ADDR = $ServerAddr
}
if (-not (Test-Path env:DATABASE_URL)) {
    $env:DATABASE_URL = "sqlite:data/sql_admin.sqlite3?mode=rwc"
}

Write-Host "  DATABASE_URL: $env:DATABASE_URL" -ForegroundColor Gray
Write-Host "  SERVER_ADDR:  $env:SERVER_ADDR" -ForegroundColor Gray
Write-Host "  RUST_LOG:     $env:RUST_LOG" -ForegroundColor Gray

Write-Host "[CHECK] Checking toolchain..." -ForegroundColor Yellow

$allOk = $true

try {
    $cargoWatchVersion = cargo watch --version 2>&1
    Write-Host "  cargo-watch: $cargoWatchVersion" -ForegroundColor Gray
} catch {
    Write-Host "  [FAIL] cargo-watch not found. Run: cargo install cargo-watch" -ForegroundColor Red
    $allOk = $false
}

if (-not $allOk) {
    Write-Host ""
    Write-Host "[ERROR] Toolchain check failed" -ForegroundColor Red
    exit 1
}

$port = $env:SERVER_ADDR.Split(':')[1]
$portCheck = netstat -ano | Select-String ":$port\s"
if ($portCheck) {
    $processId = ($portCheck -split '\s+')[-1]
    Write-Host "  [WARN] Port $port is already in use by process ID $processId" -ForegroundColor Yellow
    Write-Host "  Attempting to stop the conflicting process..." -ForegroundColor Yellow
    try {
        taskkill /f /pid $processId 2>&1 | Out-Null
        # Wait longer for port to be fully released (Windows TCP port release can be slow)
        Start-Sleep -Seconds 2
        
        # Verify port is actually free now
        $verifyCheck = netstat -ano | Select-String ":$port\s"
        if ($verifyCheck) {
            Write-Host "  [WARN] Port $port is still in use after termination attempt" -ForegroundColor Yellow
            Write-Host "  Please manually stop the process using port $port" -ForegroundColor Red
            exit 1
        }
        Write-Host "  [OK] Port $port is now available" -ForegroundColor Green
    } catch {
        Write-Host "  [FAIL] Failed to terminate process $processId" -ForegroundColor Red
        Write-Host "  Please manually stop the process using port $port" -ForegroundColor Red
        exit 1
    }
} else {
    Write-Host "  Port $port is available" -ForegroundColor Gray
}

# Second check right before launching (prevents race condition during cargo build)
Write-Host ""
Write-Host "[CHECK] Verifying port before launch..." -ForegroundColor Yellow
$preLaunchCheck = netstat -ano | Select-String ":$port\s"
if ($preLaunchCheck) {
    $blockingProcessId = ($preLaunchCheck -split '\s+')[-1]
    Write-Host "  [WARN] Port $port was occupied during build by process ID $blockingProcessId" -ForegroundColor Yellow
    taskkill /f /pid $blockingProcessId 2>&1 | Out-Null
    Start-Sleep -Seconds 2
}

$dataDir = "./data"
if (-not (Test-Path $dataDir)) {
    Write-Host ""
    Write-Host "[INIT] Creating data directory: $dataDir" -ForegroundColor Yellow
    New-Item -ItemType Directory -Path $dataDir | Out-Null
}

Write-Host ""
Write-Host "[BUILD] Compiling backend..." -ForegroundColor Yellow
cargo build -p sql-admin-interfaces
if ($LASTEXITCODE -ne 0) {
    Write-Host "[FAIL] Build failed" -ForegroundColor Red
    exit $LASTEXITCODE
}
Write-Host "  Build completed" -ForegroundColor Green

# Final port check right before launch (after cargo build time, port may have been taken)
Write-Host ""
Write-Host "[CHECK] Final port verification before launch..." -ForegroundColor Yellow
$finalCheck = netstat -ano | Select-String ":$port\s"
if ($finalCheck) {
    $blockingPid = ($finalCheck -split '\s+')[-1]
    Write-Host "  [WARN] Port $port was taken during build by process $blockingPid" -ForegroundColor Yellow
    Write-Host "  Stopping blocking process..." -ForegroundColor Yellow
    taskkill /f /pid $blockingPid 2>&1 | Out-Null
    Start-Sleep -Seconds 3
}

Write-Host ""
Write-Host "==========================================" -ForegroundColor Cyan
Write-Host "  Backend: http://$($env:SERVER_ADDR)" -ForegroundColor Green
Write-Host "  Health:  http://$($env:SERVER_ADDR)/health" -ForegroundColor Green
Write-Host "  Hot-reload: enabled (cargo-watch)" -ForegroundColor Green
Write-Host "  Press Ctrl+C to stop" -ForegroundColor Gray
Write-Host "==========================================" -ForegroundColor Cyan
Write-Host ""

cargo watch -x 'run -p sql-admin-interfaces'

Write-Host ""
Write-Host "Backend stopped." -ForegroundColor Cyan