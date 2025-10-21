# DPAPI Module

Windows Data Protection API (DPAPI) module for secure data encryption and decryption.

## Features

- Encrypt and decrypt data using Windows DPAPI
- Support for user and machine scope
- Optional entropy for additional security

## Usage

```python
from envencrypt_core.dpapi import dpapi_protect, dpapi_unprotect

# Encrypt data
data = b"Hello, World!"
entropy = b"my_entropy"
encrypted =dpapi_protect(data, entropy, False)  # False = user scope

# Decrypt data
decrypted = dpapi_unprotect(encrypted, entropy)
assert decrypted == data
```
