#!/usr/bin/env pwsh
# Build script for hybrid packaging approach

Write-Host "Building EnvEncrypt Core with Hybrid Packaging..." -ForegroundColor Green

# Clean previous builds
Write-Host "Cleaning previous builds..." -ForegroundColor Yellow
# if (Test-Path "target/wheels") {
#     Remove-Item -Recurse -Force "target/wheels"
# }
if (Test-Path "dist") {
    Remove-Item -Recurse -Force "dist"
}

# Create output directories
# New-Item -ItemType Directory -Force -Path "target/wheels" | Out-Null
New-Item -ItemType Directory -Force -Path "dist" | Out-Null

# Build individual crate packages
Write-Host "Building DPAPI crate..." -ForegroundColor Yellow
Set-Location "crates/dpapi"
uv build --out-dir "../../dist"
if ($LASTEXITCODE -ne 0) {
    Write-Error "Failed to build DPAPI crate"
    exit 1
}
Set-Location "../.."

Write-Host "Building Keyring crate..." -ForegroundColor Yellow
Set-Location "crates/keyring"
uv build --out-dir "../../dist"
if ($LASTEXITCODE -ne 0) {
    Write-Error "Failed to build Keyring crate"
    exit 1
}
Set-Location "../.."

# Build main package (pure Python with optional dependencies)
Write-Host "Building main package..." -ForegroundColor Yellow
uv build --out-dir "dist"
if ($LASTEXITCODE -ne 0) {
    Write-Error "Failed to build main package"
    exit 1
}

# Copy all wheels to dist for easy access
# Write-Host "Copying wheels to dist..." -ForegroundColor Yellow
# Copy-Item "target/wheels/*.whl" "dist/"

Write-Host "Build completed successfully!" -ForegroundColor Green
Write-Host "Packages built:" -ForegroundColor Cyan
Get-ChildItem "dist/*.whl" | ForEach-Object { Write-Host "  - $($_.Name)" -ForegroundColor White }

Write-Host ""
Write-Host "Installation examples:" -ForegroundColor Cyan
Write-Host "  # Install core package only:" -ForegroundColor White
Write-Host "  pip install dist/envencrypt_core-*.whl" -ForegroundColor Gray
Write-Host ""
Write-Host "  # Install with DPAPI support:" -ForegroundColor White
Write-Host "  pip install dist/envencrypt_core-*.whl[dpapi] dist/envencrypt_core_dpapi-*.whl" -ForegroundColor Gray
Write-Host ""
Write-Host "  # Install with all modules:" -ForegroundColor White
Write-Host "  pip install dist/envencrypt_core-*.whl[all] dist/envencrypt_core_dpapi-*.whl dist/envencrypt_core_keyring-*.whl" -ForegroundColor Gray