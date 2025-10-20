use pyo3::prelude::*;
// works only on Windows
use std::{ffi::c_void, ptr::null_mut, slice};

use windows::Win32::{
    Foundation::{LocalFree, HLOCAL},
    Security::Cryptography::{
        CryptProtectData, CryptUnprotectData,
        CRYPTPROTECT_LOCAL_MACHINE, CRYPTPROTECT_UI_FORBIDDEN,
        CRYPT_INTEGER_BLOB as DATA_BLOB,
    },
    
};

fn to_blob(bytes: &[u8]) -> DATA_BLOB {
    DATA_BLOB {
        cbData: bytes.len() as u32,
        pbData: bytes.as_ptr() as *mut u8,
    }
}

fn take_blob(blob: &mut DATA_BLOB) -> Vec<u8> {
    unsafe {
        let data = slice::from_raw_parts(blob.pbData, blob.cbData as usize).to_vec();
        LocalFree(Some(HLOCAL(blob.pbData as *mut c_void)));
        data
    }
}

/// DPAPI protect. `machine_scope = true` to use machine store; false = current user.
pub fn dpapi_protect(data: &[u8], optional_entropy: Option<&[u8]>, machine_scope: bool) -> windows::core::Result<Vec<u8>> {
    let in_blob = to_blob(data);
    let entropy_blob = optional_entropy.map(to_blob);
    let entropy_ptr = entropy_blob.as_ref().map(|b| b as *const DATA_BLOB);

    let mut out_blob = DATA_BLOB { cbData: 0, pbData: null_mut() };
    let mut flags = CRYPTPROTECT_UI_FORBIDDEN;
    if machine_scope { flags |= CRYPTPROTECT_LOCAL_MACHINE; }
    
    unsafe {
        CryptProtectData(
            &in_blob, 
            None, 
            entropy_ptr, 
            None, 
            None, 
            flags, 
            &mut out_blob,
        )?;
    }
    Ok(take_blob(&mut out_blob))
}

/// DPAPI unprotect. Use the same `optional_entropy` and scope you used to protect.
pub fn dpapi_unprotect(cipher: &[u8], optional_entropy: Option<&[u8]>) -> windows::core::Result<Vec<u8>> {
    let mut in_blob = to_blob(cipher);
    let entropy_blob = optional_entropy.map(to_blob);
    let entropy_ptr = entropy_blob.as_ref().map(|b| b as *const DATA_BLOB);

    let mut out_blob = DATA_BLOB { cbData: 0, pbData: null_mut() };
    unsafe {
        CryptUnprotectData(
            &mut in_blob,
            None,                                   // ignore description
            entropy_ptr,
            None,
            None,
            CRYPTPROTECT_UI_FORBIDDEN,
            &mut out_blob,
        )?;
    }
    Ok(take_blob(&mut out_blob))
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn roundtrip_user_scope() {
        let data = b"secret";
        let ent = Some(b"entropy".as_ref());
        let enc = dpapi_protect(data, ent, false).unwrap();
        let dec = dpapi_unprotect(&enc, ent).unwrap();
        assert_eq!(dec, data);
    }
}
/// Formats the sum of two numbers as string.
#[pyfunction]
fn sum_as_string(a: usize, b: usize) -> PyResult<String> {
    Ok((a + b).to_string())
}

#[pyfunction]
fn hello_from_bin() -> String {
    "Hello from example-ext!".to_string()
}

#[pyfunction]
fn protect(data: &[u8]) -> PyResult<Vec<u8>> {
    let protected = dpapi_protect(data, None, false)
        .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(format!("DPAPI error: {:?}", e)))?;
    Ok(protected)
}

/// A Python module implemented in Rust.
#[pymodule]
fn dpapi(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(sum_as_string, m)?)?;
    m.add_function(wrap_pyfunction!(hello_from_bin, m)?)?;
    m.add_function(wrap_pyfunction!(protect, m)?)?;
    Ok(())
}
