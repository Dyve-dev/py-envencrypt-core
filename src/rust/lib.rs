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
pub fn protect(data: &[u8], optional_entropy: Option<&[u8]>, machine_scope: bool) -> windows::core::Result<Vec<u8>> {
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
pub fn unprotect(cipher: &[u8], optional_entropy: Option<&[u8]>) -> windows::core::Result<Vec<u8>> {
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
    fn roundtrip_protect() {
        let data = b"secret";
        let ent = Some(b"entropy".as_ref());
        let enc = protect(data, ent, false).unwrap();
        let dec = unprotect(&enc, ent).unwrap();
        assert_eq!(dec, data);
    }

    #[test]
    fn roundtrip_no_entropy() {
        let data = b"secret without entropy";
        let enc = protect(data, None, false).unwrap();
        let dec = unprotect(&enc, None).unwrap();
        assert_eq!(dec, data);
    }

    #[test]
    fn roundtrip_machine_scope() {
        let data = b"machine secret";
        let enc = protect(data, None, true).unwrap();
        let dec = unprotect(&enc, None).unwrap();
        assert_eq!(dec, data);
    }
}


#[pyfunction]
#[pyo3(signature = (data, optional_entropy=None, machine_scope=false))]
fn dpapi_protect(py: Python, data: &[u8], optional_entropy: Option<&[u8]>, machine_scope: bool) -> PyResult<Py<pyo3::types::PyBytes>> {
    let protected = protect(data, optional_entropy, machine_scope)
        .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(format!("DPAPI error: {:?}", e)))?;
    Ok(pyo3::types::PyBytes::new_bound(py, &protected).into())
}

#[pyfunction]
#[pyo3(signature = (data, optional_entropy=None))]
fn dpapi_unprotect(py: Python, data: &[u8], optional_entropy: Option<&[u8]>) -> PyResult<Py<pyo3::types::PyBytes>> {
    let unprotected = unprotect(data, optional_entropy)
        .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(format!("DPAPI error: {:?}", e)))?;
    Ok(pyo3::types::PyBytes::new_bound(py, &unprotected).into())
}

/// A Python module implemented in Rust.
#[pymodule]
fn core(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(dpapi_protect, m)?)?;
    m.add_function(wrap_pyfunction!(dpapi_unprotect, m)?)?;
    Ok(())
}
