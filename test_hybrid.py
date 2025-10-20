#!/usr/bin/env python3
"""
Test script to demonstrate the hybrid packaging approach for envencrypt-core.

This script shows how the package behaves with different installation configurations.
"""

import sys
import traceback

def test_core_functionality():
    """Test core package functionality."""
    print("=" * 50)
    print("Testing Core Functionality")
    print("=" * 50)
    
    try:
        import envencrypt_core
        print(f"✓ Core package imported successfully")
        print(f"  Version: {envencrypt_core.get_version()}")
        
        # Check available modules
        available = envencrypt_core.available_modules()
        print(f"  Available modules: {available}")
        
        return True
    except Exception as e:
        print(f"✗ Core package import failed: {e}")
        return False

def test_dpapi_functionality():
    """Test DPAPI functionality if available."""
    print("\n" + "=" * 50)
    print("Testing DPAPI Functionality")
    print("=" * 50)
    
    try:
        import envencrypt_core
        
        if envencrypt_core.available_modules().get('dpapi', False):
            print("✓ DPAPI module is available")
            
            # Test direct module access
            if envencrypt_core.dpapi:
                print("✓ DPAPI module accessible via envencrypt_core.dpapi")
            
            # Test convenience function
            test_data = b"Hello, DPAPI!"
            try:
                encrypted = envencrypt_core.encrypt_with_dpapi(test_data, "Test encryption")
                decrypted, description = envencrypt_core.decrypt_with_dpapi(encrypted)
                
                if decrypted == test_data:
                    print(f"✓ DPAPI encryption/decryption works")
                    print(f"  Original: {test_data}")
                    print(f"  Description: '{description}'")
                else:
                    print("✗ DPAPI encryption/decryption failed - data mismatch")
            except Exception as e:
                print(f"✗ DPAPI encryption/decryption failed: {e}")
        else:
            print("⚠ DPAPI module not available")
            
            # Test error handling
            try:
                envencrypt_core.encrypt_with_dpapi(b"test")
                print("✗ Expected ImportError not raised")
            except ImportError as e:
                print(f"✓ Proper error handling: {e}")
                
    except Exception as e:
        print(f"✗ DPAPI test failed: {e}")
        traceback.print_exc()

def test_keyring_functionality():
    """Test keyring functionality if available."""
    print("\n" + "=" * 50)
    print("Testing Keyring Functionality")
    print("=" * 50)
    
    try:
        import envencrypt_core
        
        if envencrypt_core.available_modules().get('keyring', False):
            print("✓ Keyring module is available")
            
            if envencrypt_core.keyring:
                print("✓ Keyring module accessible via envencrypt_core.keyring")
                # Add keyring-specific tests here when functions are implemented
            
        else:
            print("⚠ Keyring module not available")
            print("  Install with: pip install envencrypt-core[keyring]")
                
    except Exception as e:
        print(f"✗ Keyring test failed: {e}")
        traceback.print_exc()

def test_require_module():
    """Test the require_module function."""
    print("\n" + "=" * 50)
    print("Testing Module Requirements")
    print("=" * 50)
    
    try:
        import envencrypt_core
        
        # Test requiring available modules
        available = envencrypt_core.available_modules()
        for module_name, is_available in available.items():
            try:
                envencrypt_core.require_module(module_name)
                if is_available:
                    print(f"✓ require_module('{module_name}') passed (module available)")
                else:
                    print(f"✗ require_module('{module_name}') should have failed")
            except ImportError as e:
                if not is_available:
                    print(f"✓ require_module('{module_name}') properly failed: {e}")
                else:
                    print(f"✗ require_module('{module_name}') unexpectedly failed: {e}")
        
        # Test requiring non-existent module
        try:
            envencrypt_core.require_module('nonexistent')
            print("✗ require_module('nonexistent') should have failed")
        except ImportError as e:
            print(f"✓ require_module('nonexistent') properly failed: {e}")
            
    except Exception as e:
        print(f"✗ require_module test failed: {e}")
        traceback.print_exc()

def main():
    """Run all tests."""
    print("EnvEncrypt Core - Hybrid Packaging Test")
    print(f"Python version: {sys.version}")
    print(f"Platform: {sys.platform}")
    
    # Run tests
    core_ok = test_core_functionality()
    
    if core_ok:
        test_dpapi_functionality()
        test_keyring_functionality() 
        test_require_module()
    
    print("\n" + "=" * 50)
    print("Test Summary")
    print("=" * 50)
    
    if core_ok:
        print("✓ Core functionality tests completed")
        print("\nTo install optional modules:")
        print("  pip install envencrypt-core[dpapi]    # For DPAPI support")
        print("  pip install envencrypt-core[keyring]  # For keyring support")
        print("  pip install envencrypt-core[all]      # For all modules")
    else:
        print("✗ Core functionality tests failed")
        return 1
    
    return 0

if __name__ == "__main__":
    sys.exit(main())