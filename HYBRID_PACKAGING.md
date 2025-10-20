# EnvEncrypt Core - Hybrid Packaging

This document explains the hybrid packaging approach for the EnvEncrypt Core project, which allows users to install only the functionality they need.

## Package Structure

### Core Package: `envencrypt-core`
- **Type**: Pure Python package
- **Dependencies**: None by default
- **Purpose**: Provides main interface and optional dependency management
- **Always Available**: Core utilities, module detection, helpful error messages

### Optional Packages

#### `envencrypt-core-dpapi`
- **Type**: Python extension module (Rust-based)
- **Platform**: Windows only
- **Purpose**: Windows Data Protection API functionality
- **Install**: `pip install envencrypt-core[dpapi]`

#### `envencrypt-core-keyring`  
- **Type**: Python extension module (Rust-based)
- **Platform**: Cross-platform
- **Purpose**: System keyring/password management
- **Install**: `pip install envencrypt-core[keyring]`

## Installation Options

### Basic Installation
```bash
pip install envencrypt-core
```
Installs only the core package with no platform-specific functionality.

### With DPAPI Support (Windows)
```bash
pip install envencrypt-core[dpapi]
```
Installs core package + DPAPI module for Windows encryption.

### With Keyring Support
```bash
pip install envencrypt-core[keyring]
```
Installs core package + keyring module for password management.

### Complete Installation
```bash
pip install envencrypt-core[all]
```
Installs core package + all available optional modules.

## Usage Examples

### Basic Usage with Module Detection
```python
import envencrypt_core

# Check what's available
available = envencrypt_core.available_modules()
print(f"Available modules: {available}")

# Get version
print(f"Version: {envencrypt_core.get_version()}")
```

### Using DPAPI (Windows only)
```python
import envencrypt_core

# Method 1: Direct module access
if envencrypt_core.dpapi:
    encrypted = envencrypt_core.dpapi.encrypt(b"secret data", "My secret")
    decrypted, description = envencrypt_core.dpapi.decrypt(encrypted)

# Method 2: Convenience functions with error handling
try:
    encrypted = envencrypt_core.encrypt_with_dpapi(b"secret data", "My secret")
    decrypted, description = envencrypt_core.decrypt_with_dpapi(encrypted)
except ImportError as e:
    print(f"DPAPI not available: {e}")
```

### Graceful Degradation
```python
import envencrypt_core

def encrypt_data(data: bytes) -> bytes:
    """Encrypt data using the best available method."""
    
    # Prefer DPAPI on Windows
    if envencrypt_core.available_modules().get('dpapi', False):
        return envencrypt_core.encrypt_with_dpapi(data)
    
    # Fall back to keyring if available
    elif envencrypt_core.available_modules().get('keyring', False):
        # Use keyring-based encryption
        pass
    
    # Fall back to basic encryption
    else:
        # Use basic encryption method
        pass
```

## Building the Project

### Build All Packages
```bash
# PowerShell
.\build_hybrid.ps1

# Or manually:
# Build individual crates
cd crates\dpapi
maturin build --release --out ..\..\target\wheels
cd ..\keyring  
maturin build --release --out ..\..\target\wheels
cd ..\..

# Build main package
python -m build --wheel --outdir target\wheels
```

### Testing
```bash
python test_hybrid.py
```

## Advantages of This Approach

1. **Minimal Dependencies**: Users only install what they need
2. **Platform Flexibility**: Windows-specific code only on Windows
3. **Clear Error Messages**: Helpful guidance when modules are missing
4. **Backward Compatibility**: Core functionality works everywhere
5. **Modular Development**: Each crate can be developed independently
6. **Easy Discovery**: Clear naming shows what extras provide

## Package Relationships

```
envencrypt-core (pure Python)
├── [dpapi] → envencrypt-core-dpapi (Rust extension)
├── [keyring] → envencrypt-core-keyring (Rust extension)  
└── [all] → both dpapi and keyring
```

## Development Workflow

1. **Develop crates independently** in `crates/` directories
2. **Test crates individually** using their own test suites
3. **Update core package** to expose new functionality
4. **Build all packages** using the build script
5. **Test integration** using the hybrid test script
6. **Publish packages** to PyPI (core + optional packages)

## Troubleshooting

### Module Not Available Errors
If you see errors like:
```
ImportError: Module 'dpapi' is not available. Install with: pip install envencrypt-core[dpapi]
```

This means you're trying to use functionality that requires an optional package. Install the suggested extra to resolve the issue.

### Platform-Specific Issues
- DPAPI functionality only works on Windows
- If you install `envencrypt-core[dpapi]` on non-Windows systems, the package will install but the module won't be available

### Build Issues
- Ensure Rust toolchain is installed for building extension modules
- Use `maturin` for building Rust-based Python extensions
- Check that all dependencies are properly specified in `pyproject.toml` files