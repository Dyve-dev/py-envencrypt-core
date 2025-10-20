# Build script for separate .pyd files
# This will create dpapi.pyd and keyring.pyd

Write-Host "Building DPAPI module..."
Set-Location "crates\dpapi"
uv build
if ($LASTEXITCODE -ne 0) {
    Write-Error "DPAPI build failed"
    exit 1
}

Write-Host "Building Keyring module..."
Set-Location "..\keyring"
uv build
if ($LASTEXITCODE -ne 0) {
    Write-Error "Keyring build failed" 
    exit 1
}

Set-Location "..\..\"
Write-Host "Both modules built successfully!"
Write-Host "You should now have:"
Write-Host "  - envencrypt_core.dpapi (dpapi.pyd)"
Write-Host "  - envencrypt_core.keyring (keyring.pyd)"
Write-Host ""
Write-Host "Install with:"
Write-Host "  pip install crates/dpapi/dist/*.whl"
Write-Host "  pip install crates/keyring/dist/*.whl"