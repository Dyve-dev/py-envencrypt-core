#!/usr/bin/env python3
"""Test script to verify both modules work independently."""

def test_dpapi():
    """Test DPAPI module."""
    try:
        import envencrypt_core.dpapi as dpapi
        
        # Test data
        data = b"Hello, DPAPI!"
        entropy = b"test_entropy"
        
        # Test protection
        encrypted = dpapi.protect(data, entropy, False)
        print(f"DPAPI: Encrypted {len(data)} bytes to {len(encrypted)} bytes")
        
        # Test unprotection
        decrypted = dpapi.unprotect(encrypted, entropy)
        print(f"DPAPI: Decrypted back to {len(decrypted)} bytes")
        
        assert decrypted == data, "DPAPI roundtrip failed!"
        print("✓ DPAPI module working correctly")
        return True
        
    except ImportError as e:
        print(f"✗ DPAPI module not available: {e}")
        return False
    except Exception as e:
        print(f"✗ DPAPI test failed: {e}")
        return False

def test_keyring():
    """Test Keyring module."""
    try:
        import envencrypt_core.keyring as keyring
        
        # Test set/get password
        keyring.set_password("test_service", "test_user", "secret123")
        password = keyring.get_password("test_service", "test_user")
        
        assert password == "secret123", "Keyring get/set failed!"
        print("✓ Keyring set/get working")
        
        # Test delete password
        deleted = keyring.delete_password("test_service", "test_user")
        assert deleted, "Delete should return True"
        
        password = keyring.get_password("test_service", "test_user")
        assert password is None, "Password should be deleted"
        print("✓ Keyring delete working")
        
        # Test list services
        keyring.set_password("service1", "user1", "pass1")
        keyring.set_password("service2", "user2", "pass2")
        services = keyring.list_services()
        print(f"✓ Keyring services: {services}")
        
        print("✓ Keyring module working correctly")
        return True
        
    except ImportError as e:
        print(f"✗ Keyring module not available: {e}")
        return False
    except Exception as e:
        print(f"✗ Keyring test failed: {e}")
        return False

def main():
    """Run all tests."""
    print("Testing separate .pyd modules...\n")
    
    dpapi_ok = test_dpapi()
    print()
    keyring_ok = test_keyring()
    print()
    
    if dpapi_ok and keyring_ok:
        print("🎉 All modules working correctly!")
        print("\nYou now have separate .pyd files:")
        print("  - dpapi.pyd (DPAPI functions)")
        print("  - keyring.pyd (Keyring functions)")
    else:
        print("❌ Some modules failed. Check the build process.")

if __name__ == "__main__":
    main()