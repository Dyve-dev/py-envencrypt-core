use pyo3::prelude::*;
mod dpapi;
use dpapi::dpapi::{dpapi_protect, dpapi_unprotect};

/// A Python module implemented in Rust.
#[pymodule]
fn core(m: &Bound<'_, PyModule>) -> PyResult<()> {
    #[cfg(target_os = "windows")]
    m.add_function(wrap_pyfunction!(dpapi_protect, m)?)?;
    #[cfg(target_os = "windows")]
    m.add_function(wrap_pyfunction!(dpapi_unprotect, m)?)?;
    Ok(())
}
