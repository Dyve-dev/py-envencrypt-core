use pyo3::prelude::*;
use std::collections::HashMap;
use std::sync::{LazyLock, Mutex};

// Simple in-memory keyring for demonstration
// In a real implementation, you'd use Windows Credential Manager
static KEYSTORE: LazyLock<Mutex<HashMap<String, String>>> = LazyLock::new(|| Mutex::new(HashMap::new()));

/// Get a password from the keyring
#[pyfunction]
fn get_password(service: &str, username: &str) -> PyResult<Option<String>> {
    let key = format!("{}:{}", service, username);
    let keystore = KEYSTORE.lock().map_err(|_| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>("Keystore lock failed"))?;
    Ok(keystore.get(&key).cloned())
}

/// Set a password in the keyring
#[pyfunction]
fn set_password(service: &str, username: &str, password: &str) -> PyResult<()> {
    let key = format!("{}:{}", service, username);
    let mut keystore = KEYSTORE.lock().map_err(|_| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>("Keystore lock failed"))?;
    keystore.insert(key, password.to_string());
    Ok(())
}

/// Delete a password from the keyring
#[pyfunction]
fn delete_password(service: &str, username: &str) -> PyResult<bool> {
    let key = format!("{}:{}", service, username);
    let mut keystore = KEYSTORE.lock().map_err(|_| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>("Keystore lock failed"))?;
    Ok(keystore.remove(&key).is_some())
}

/// List all services in the keyring
#[pyfunction]
fn list_services() -> PyResult<Vec<String>> {
    let keystore = KEYSTORE.lock().map_err(|_| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>("Keystore lock failed"))?;
    let services: std::collections::HashSet<String> = keystore
        .keys()
        .map(|key| key.split(':').next().unwrap_or("").to_string())
        .collect();
    Ok(services.into_iter().collect())
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_set_get_password() {
        set_password("test_service", "test_user", "test_password").unwrap();
        let result = get_password("test_service", "test_user").unwrap();
        assert_eq!(result, Some("test_password".to_string()));
    }
    
    #[test]
    fn test_delete_password() {
        set_password("test_service2", "test_user2", "test_password2").unwrap();
        let deleted = delete_password("test_service2", "test_user2").unwrap();
        assert!(deleted);
        
        let result = get_password("test_service2", "test_user2").unwrap();
        assert_eq!(result, None);
    }
}

/// Keyring module for password management
#[pymodule]
fn keyring(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(get_password, m)?)?;
    m.add_function(wrap_pyfunction!(set_password, m)?)?;
    m.add_function(wrap_pyfunction!(delete_password, m)?)?;
    m.add_function(wrap_pyfunction!(list_services, m)?)?;
    Ok(())
}