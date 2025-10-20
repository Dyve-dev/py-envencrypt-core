# Keyring Module

Cross-platform keyring module for secure password storage and retrieval.

## Features

- Store and retrieve passwords securely
- Delete stored passwords
- List available services
- Thread-safe implementation

## Usage

```python
import envencrypt_core.keyring as keyring

# Set a password
keyring.set_password("myapp", "username", "secret123")

# Get a password
password = keyring.get_password("myapp", "username")
print(password)  # "secret123"

# Delete a password
deleted = keyring.delete_password("myapp", "username")
print(deleted)  # True

# List services
services = keyring.list_services()
print(services)  # ["myapp", ...]
```