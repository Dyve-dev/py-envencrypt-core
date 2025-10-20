# Development build script (faster for testing)
# This installs the modules directly into your Python environment

Write-Host "Building DPAPI module (development)..."
Set-Location "crates\dpapi"
uv run maturin develop
if ($LASTEXITCODE -ne 0) {
    Write-Error "DPAPI build failed"
    exit 1
}

Write-Host "Building Keyring module (development)..."
Set-Location "..\keyring"
uv run maturin develop
if ($LASTEXITCODE -ne 0) {
    Write-Error "Keyring build failed" 
    exit 1
}

Set-Location "..\..\"
Write-Host "Both modules built and installed successfully!"
Write-Host "You can now import:"
Write-Host "  import envencrypt_core.dpapi"
Write-Host "  import envencrypt_core.keyring"