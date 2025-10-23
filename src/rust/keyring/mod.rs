use pyo3::prelude::*;

#[cfg(target_os = "windows")]
use windows::{
    Security::Credentials::{PasswordCredential, PasswordVault},
    Win32::Security::Cryptography::{
        BCryptOpenAlgorithmProvider, BCryptGenerateSymmetricKey, BCryptCloseAlgorithmProvider,
        BCRYPT_ALG_HANDLE, BCRYPT_KEY_HANDLE, BCRYPT_AES_ALGORITHM, BCRYPT_ALG_HANDLE_HMAC_FLAG
    },
};

// #[cfg(target_os = "windows")]
// pub fn generate_random_key(key_length: usize) -> windows::core::Result<Vec<u8>> {
//     let mut hprov: HCRYPTPROV = HCRYPTPROV::default();
    
//     unsafe {
//         // Acquire cryptographic context
//         CryptAcquireContextW(
//             &mut hprov,
//             None,  // No container name
//             None,  // Use default provider
//             PROV_RSA_FULL,
//             CRYPT_VERIFYCONTEXT, // Don't need persistent key container
//         )?;
        
//         // Generate random bytes for the key
//         let mut key_buffer = vec![0u8; key_length];
//         CryptGenRandom(hprov, key_buffer.len() as u32, key_buffer.as_mut_ptr())?;
        
//         // Release context
//         CryptReleaseContext(hprov, 0)?;
        
//         Ok(key_buffer)
//     }
// }
#[cfg(target_os = "windows")]
pub fn generate_aes_key() -> windows::core::Result<Vec<u8>> {
    let mut alg_handle = BCRYPT_ALG_HANDLE::default();
    let mut key_handle = BCRYPT_KEY_HANDLE::default();

    unsafe {
        // Open AES algorithm provider
        BCryptOpenAlgorithmProvider(
            &mut alg_handle,
            BCRYPT_AES_ALGORITHM,
            None,
            BCRYPT_ALG_HANDLE_HMAC_FLAG,
        );
        
        // Generate 256-bit (32 bytes) AES key
        let key_data = vec![0u8; 32];
        BCryptGenerateSymmetricKey(
            alg_handle,
            &mut key_handle,
            None,
            Some(&key_data),
            0,
        );
        
        // Close algorithm provider
        BCryptCloseAlgorithmProvider(alg_handle, 0);
        
        Ok(key_data)
    }
}
    // unsafe {
    //     // Acquire AES cryptographic context
    //     CryptAcquireContextW(
    //         &mut hprov,
    //         None,
    //         None,
    //         PROV_RSA_AES,
    //         CRYPT_VERIFYCONTEXT,
    //     )?;
        
    //     // Generate AES-256 key
    //     CryptGenKey(
    //         hprov,
    //         CALG_AES_256,
    //         CRYPT_EXPORTABLE,
    //         &mut hkey,
    //     )?;
        
    //     // Get key size
    //     let mut key_size = 0u32;
    //     let mut size_len = std::mem::size_of::<u32>() as u32;
    //     CryptGetKeyParam(
    //         hkey,
    //         KP_KEYLEN,
    //         Some(&mut key_size as *mut u32 as *mut u8),
    //         &mut size_len,
    //         0,
    //     )?;
        
    //     // Export key to get raw key material
    //     let mut key_blob_size = 0u32;
    //     CryptExportKey(hkey, HCRYPTKEY::default(), PLAINTEXTKEYBLOB, 0, None, &mut key_blob_size)?;
        
    //     let mut key_blob = vec![0u8; key_blob_size as usize];
    //     CryptExportKey(hkey, HCRYPTKEY::default(), PLAINTEXTKEYBLOB, 0, Some(key_blob.as_mut_ptr()), &mut key_blob_size)?;
        
    //     // Clean up
    //     CryptDestroyKey(hkey)?;
    //     CryptReleaseContext(hprov, 0)?;
        
    //     // Extract actual key from blob (skip blob header)
    //     // PLAINTEXTKEYBLOB format: BLOBHEADER + DWORD key_size + key_data
    //     let header_size = std::mem::size_of::<BLOBHEADER>() + std::mem::size_of::<u32>();
    //     Ok(key_blob[header_size..].to_vec())
    // }
