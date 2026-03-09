use pyo3::prelude::*;
use serde::Serialize;
use serde_json::Value;

const BLOCK_ID_FIELDS: &[&str] = &[
    "block_id",
    "root_block_id",
    "parent",
    "focus",
    "focus_before",
    "focus_after",
    "source",
    "target",
    "from",
    "to",
];

const BLOCK_ID_LIST_FIELDS: &[&str] = &["children", "added", "removed", "changed"];

pub fn to_python_json<T: Serialize>(py: Python<'_>, value: &T) -> PyResult<PyObject> {
    let mut json_value = serde_json::to_value(value)
        .map_err(|err| pyo3::exceptions::PyValueError::new_err(err.to_string()))?;
    normalize_block_ids(&mut json_value, None);
    let json = serde_json::to_string(&json_value)
        .map_err(|err| pyo3::exceptions::PyValueError::new_err(err.to_string()))?;
    Ok(py
        .import_bound("json")?
        .call_method1("loads", (json,))?
        .into())
}

fn normalize_block_ids(value: &mut Value, key: Option<&str>) {
    match value {
        Value::Object(map) => {
            for (child_key, child_value) in map.iter_mut() {
                normalize_block_ids(child_value, Some(child_key));
            }
        }
        Value::Array(values) => {
            if key
                .map(|name| BLOCK_ID_LIST_FIELDS.contains(&name))
                .unwrap_or(false)
            {
                for value in values.iter_mut() {
                    if let Value::String(block_id) = value {
                        normalize_block_id_string(block_id);
                    } else {
                        normalize_block_ids(value, None);
                    }
                }
            } else {
                for value in values.iter_mut() {
                    normalize_block_ids(value, None);
                }
            }
        }
        Value::String(block_id)
            if key
                .map(|name| BLOCK_ID_FIELDS.contains(&name))
                .unwrap_or(false) =>
        {
            normalize_block_id_string(block_id);
        }
        _ => {}
    }
}

fn normalize_block_id_string(block_id: &mut String) {
    if block_id.starts_with("blk_") {
        return;
    }
    if block_id.len() == 24 && block_id.chars().all(|ch| ch.is_ascii_hexdigit()) {
        *block_id = format!("blk_{block_id}");
    }
}
