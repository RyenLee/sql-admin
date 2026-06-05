<#
SQL Admin 桌面应用构建脚本
==========================

功能：
1. 编译前端 WASM（带 tauri feature）
2. 构建 Tauri 桌面安装包

输出：
- Windows: crates/desktop/target/release/bundle/nsis/ 下的 .exe 安装包
- Windows: crates/desktop/target/release/bundle/msi/ 下的 .msi 安装包

使用方法：
.\scripts\build-desktop.ps1
.\scripts\build-desktop.ps1 -Target nsis       (仅构建 NSIS 安装包)
.\scripts\build-desktop.ps1 -Target msi        (仅构建 MSI 安装包)
.\scripts\build-desktop.ps1 -SkipBundle        (仅编译，不打包)
#>

param(
    [ValidateSet("nsis", "msi", "all")]
    [string]$Target = "all",
    [switch]$SkipBundle
)

Write-Host "==========================================" -ForegroundColor Cyan
Write-Host "  SQL Admin - Desktop Build" -ForegroundColor Cyan
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
        Write-Host "  [WARN] CSS build failed" -ForegroundColor Yellow
    } else {
        Write-Host "  CSS build completed" -ForegroundColor Gray
    }
} catch {
    Write-Host "  [WARN] npm not available, skipping CSS build" -ForegroundColor Yellow
}
Pop-Location

# ---- Build ----
Write-Host ""

# Set release log level to ERROR
$env:RUST_LOG = "error"
Write-Host "  Log level: ERROR (release build)" -ForegroundColor Gray

if ($SkipBundle) {
    Write-Host "[BUILD] Compiling desktop app (no bundle)..." -ForegroundColor Yellow
    cargo build -p sql-admin-desktop --release
    if ($LASTEXITCODE -ne 0) {
        Write-Host "[FAIL] Build failed" -ForegroundColor Red
        exit $LASTEXITCODE
    }
    $exePath = "crates/desktop/target/release/sql-admin-desktop.exe"
    if (Test-Path $exePath) {
        Write-Host "  Output: $exePath" -ForegroundColor Green
    }
} else {
    $buildArgs = @("build")
    if ($Target -eq "nsis") {
        $buildArgs += "--bundles", "nsis"
    } elseif ($Target -eq "msi") {
        $buildArgs += "--bundles", "msi"
    }

    Write-Host "[BUILD] Building Tauri app (target: $Target)..." -ForegroundColor Yellow
    Push-Location crates/desktop
    try {
        cargo tauri $buildArgs
        if ($LASTEXITCODE -ne 0) {
            Write-Host "[FAIL] Build failed" -ForegroundColor Red
            exit $LASTEXITCODE
        }
    } finally {
        Pop-Location
    }

    Write-Host ""
    Write-Host "==========================================" -ForegroundColor Green
    Write-Host "  Build completed!" -ForegroundColor Green
    Write-Host "==========================================" -ForegroundColor Green

    # Show output files
    $bundleDir = "crates/desktop/target/release/bundle"
    if (Test-Path $bundleDir) {
        Write-Host ""
        Write-Host "Output files:" -ForegroundColor Yellow
        Get-ChildItem -Path $bundleDir -Recurse -Include "*.exe", "*.msi" | ForEach-Object {
            $sizeMB = [math]::Round($_.Length / 1MB, 2)
            Write-Host "  $($_.FullName) ($sizeMB MB)" -ForegroundColor Gray
        }
    }
}

Write-Host ""
Write-Host "Done." -ForegroundColor Cyan
