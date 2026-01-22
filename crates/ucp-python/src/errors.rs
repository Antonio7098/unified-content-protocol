//! Python exception types for UCP errors.

use pyo3::exceptions::PyException;
use pyo3::prelude::*;
use ucm_core::Error;

// Define exception hierarchy
pyo3::create_exception!(_core, PyUcpError, PyException, "Base UCP error");
pyo3::create_exception!(_core, PyBlockNotFoundError, PyUcpError, "Block not found");
pyo3::create_exception!(
    _core,
    PyInvalidBlockIdError,
    PyUcpError,
    "Invalid block ID format"
);
pyo3::create_exception!(
    _core,
    PyCycleDetectedError,
    PyUcpError,
    "Cycle detected in document structure"
);
pyo3::create_exception!(_core, PyValidationError, PyUcpError, "Validation error");
pyo3::create_exception!(_core, PyParseError, PyUcpError, "Parse error");

/// Convert a Rust UCM error to a Python exception.
pub fn convert_error(err: Error) -> PyErr {
    match err {
        Error::BlockNotFound(id) => {
            PyBlockNotFoundError::new_err(format!("Block not found: {}", id))
        }
        Error::InvalidBlockId(id) => {
            PyInvalidBlockIdError::new_err(format!("Invalid block ID: {}", id))
        }
        Error::CycleDetected(id) => {
            PyCycleDetectedError::new_err(format!("Cycle detected at block: {}", id))
        }
        Error::Validation(msg) => PyValidationError::new_err(msg),
        Error::Parse {
            message,
            line,
            column,
        } => PyParseError::new_err(format!(
            "Parse error at line {}, column {}: {}",
            line, column, message
        )),
        Error::VersionConflict { expected, actual } => PyUcpError::new_err(format!(
            "Version conflict: expected {}, found {}",
            expected, actual
        )),
        other => PyUcpError::new_err(other.to_string()),
    }
}

/// Helper trait for ergonomic error conversion.
pub trait IntoPyResult<T> {
    fn into_py_result(self) -> PyResult<T>;
}

impl<T> IntoPyResult<T> for ucm_core::Result<T> {
    fn into_py_result(self) -> PyResult<T> {
        self.map_err(convert_error)
    }
}
