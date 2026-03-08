use pyo3::prelude::*;
use serde::Serialize;

pub fn to_python_json<T: Serialize>(py: Python<'_>, value: &T) -> PyResult<PyObject> {
    let json = serde_json::to_string(value)
        .map_err(|err| pyo3::exceptions::PyValueError::new_err(err.to_string()))?;
    Ok(py
        .import_bound("json")?
        .call_method1("loads", (json,))?
        .into())
}
