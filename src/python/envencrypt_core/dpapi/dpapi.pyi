def dpapi_protect(data: bytes, optional_entropy: bytes | None = None, machine_scope: bool = False) -> bytes:
    """
    Protect data using Windows DPAPI (Data Protection API).
    
    Args:
        data: The data to protect (encrypt)
        optional_entropy: Optional additional entropy for encryption (default: None)
        machine_scope: If True, use machine store; if False, use current user store (default: False)
    
    Returns:
        The encrypted data as bytes
        
    Raises:
        RuntimeError: If DPAPI encryption fails
    """
    ...

def dpapi_unprotect(data: bytes, optional_entropy: bytes | None = None) -> bytes:
    """
    Unprotect (decrypt) data using Windows DPAPI (Data Protection API).
    
    Args:
        data: The encrypted data to unprotect (decrypt)
        optional_entropy: Optional entropy used during protection (must match what was used for protection)
    
    Returns:
        The decrypted data as bytes
        
    Raises:
        RuntimeError: If DPAPI decryption fails
    """
    ...