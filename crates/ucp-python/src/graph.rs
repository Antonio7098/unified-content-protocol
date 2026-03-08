use std::fs;

use pyo3::prelude::*;

use crate::{document::PyDocument, json::to_python_json, types::PyBlockId};

#[pyclass(unsendable, name = "Graph")]
#[derive(Clone)]
pub struct PyGraph {
    inner: ucp_api::GraphNavigator,
}

#[pyclass(unsendable, name = "GraphSession")]
#[derive(Clone)]
pub struct PyGraphSession {
    inner: ucp_api::GraphSession,
}

#[pymethods]
impl PyGraph {
    #[staticmethod]
    fn from_document(doc: &PyDocument) -> Self {
        Self {
            inner: ucp_api::GraphNavigator::from_document(doc.inner().clone()),
        }
    }

    #[staticmethod]
    fn from_json(payload: &str) -> PyResult<Self> {
        Ok(Self {
            inner: ucp_api::GraphNavigator::from_json(payload).map_err(to_runtime_error)?,
        })
    }

    #[staticmethod]
    fn load(path: &str) -> PyResult<Self> {
        Ok(Self {
            inner: ucp_api::GraphNavigator::load(path).map_err(to_runtime_error)?,
        })
    }

    #[staticmethod]
    fn from_sqlite(path: &str, graph_key: &str) -> PyResult<Self> {
        Ok(Self {
            inner: ucp_api::GraphNavigator::open_sqlite(path, graph_key)
                .map_err(to_runtime_error)?,
        })
    }

    fn persist_sqlite(&self, path: &str, graph_key: &str) -> PyResult<Self> {
        Ok(Self {
            inner: self
                .inner
                .persist_sqlite(path, graph_key)
                .map_err(to_runtime_error)?,
        })
    }

    fn save(&self, path: &str) -> PyResult<()> {
        fs::write(path, self.to_json()?).map_err(to_runtime_error)
    }

    fn to_json(&self) -> PyResult<String> {
        self.inner.to_json().map_err(to_runtime_error)
    }

    fn to_document(&self) -> PyResult<PyDocument> {
        Ok(PyDocument::new(
            self.inner.to_document().map_err(to_runtime_error)?,
        ))
    }

    fn session(&self) -> PyGraphSession {
        PyGraphSession {
            inner: self.inner.session(),
        }
    }

    fn root_id(&self) -> PyBlockId {
        PyBlockId::from(self.inner.root_id())
    }

    fn resolve(&self, selector: &str) -> Option<PyBlockId> {
        self.inner.resolve_selector(selector).map(PyBlockId::from)
    }

    fn store_stats(&self, py: Python<'_>) -> PyResult<PyObject> {
        to_python_json(py, &self.inner.store_stats())
    }

    fn observability(&self, py: Python<'_>) -> PyResult<PyObject> {
        to_python_json(py, &self.inner.observability())
    }

    fn describe(&self, py: Python<'_>, selector: &str) -> PyResult<Option<PyObject>> {
        Ok(self
            .inner
            .describe(selector)
            .map(|node| to_python_json(py, &node))
            .transpose()?)
    }

    #[pyo3(signature = (label_regex=None, content_type=None, semantic_role_regex=None, tag_regex=None, case_sensitive=false, limit=None))]
    fn find_nodes(
        &self,
        py: Python<'_>,
        label_regex: Option<String>,
        content_type: Option<String>,
        semantic_role_regex: Option<String>,
        tag_regex: Option<String>,
        case_sensitive: bool,
        limit: Option<usize>,
    ) -> PyResult<PyObject> {
        let matches = self
            .inner
            .find_nodes(&ucp_api::GraphFindQuery {
                label_regex,
                content_type,
                semantic_role_regex,
                tag_regex,
                case_sensitive,
                limit,
            })
            .map_err(to_runtime_error)?;
        to_python_json(py, &matches)
    }

    #[pyo3(signature = (start, end, max_hops=6))]
    fn path_between(
        &self,
        py: Python<'_>,
        start: &str,
        end: &str,
        max_hops: usize,
    ) -> PyResult<Option<PyObject>> {
        let Some(start_id) = self.inner.resolve_selector(start) else {
            return Ok(None);
        };
        let Some(end_id) = self.inner.resolve_selector(end) else {
            return Ok(None);
        };
        Ok(self
            .inner
            .path_between(start_id, end_id, max_hops)
            .map(|path| to_python_json(py, &path))
            .transpose()?)
    }
}

#[pymethods]
impl PyGraphSession {
    fn fork(&self) -> Self {
        self.clone()
    }

    fn selected_block_ids(&self) -> Vec<PyBlockId> {
        self.inner
            .selected_block_ids()
            .into_iter()
            .map(PyBlockId::from)
            .collect()
    }

    fn summary(&self, py: Python<'_>) -> PyResult<PyObject> {
        to_python_json(py, &self.inner.summary())
    }

    #[pyo3(signature = (max_depth=None))]
    fn seed_overview(&mut self, py: Python<'_>, max_depth: Option<usize>) -> PyResult<PyObject> {
        to_python_json(py, &self.inner.seed_overview(max_depth))
    }

    #[pyo3(signature = (selector=None))]
    fn focus(&mut self, py: Python<'_>, selector: Option<&str>) -> PyResult<PyObject> {
        to_python_json(py, &self.inner.focus(selector).map_err(to_runtime_error)?)
    }

    #[pyo3(signature = (selector, detail_level="summary"))]
    fn select(&mut self, py: Python<'_>, selector: &str, detail_level: &str) -> PyResult<PyObject> {
        let detail_level = parse_detail_level(detail_level)?;
        to_python_json(
            py,
            &self
                .inner
                .select(selector, detail_level)
                .map_err(to_runtime_error)?,
        )
    }

    #[pyo3(signature = (selector, mode="children", depth=1, max_add=None))]
    fn expand(
        &mut self,
        py: Python<'_>,
        selector: &str,
        mode: &str,
        depth: usize,
        max_add: Option<usize>,
    ) -> PyResult<PyObject> {
        let mode = parse_neighbor_mode(mode)?;
        to_python_json(
            py,
            &self
                .inner
                .expand(selector, mode, depth, max_add)
                .map_err(to_runtime_error)?,
        )
    }

    #[pyo3(signature = (selector, include_descendants=false))]
    fn collapse(
        &mut self,
        py: Python<'_>,
        selector: &str,
        include_descendants: bool,
    ) -> PyResult<PyObject> {
        to_python_json(
            py,
            &self
                .inner
                .collapse(selector, include_descendants)
                .map_err(to_runtime_error)?,
        )
    }

    #[pyo3(signature = (selector, pinned=true))]
    fn pin(&mut self, py: Python<'_>, selector: &str, pinned: bool) -> PyResult<PyObject> {
        to_python_json(
            py,
            &self.inner.pin(selector, pinned).map_err(to_runtime_error)?,
        )
    }

    #[pyo3(signature = (max_selected=None))]
    fn prune(&mut self, py: Python<'_>, max_selected: Option<usize>) -> PyResult<PyObject> {
        to_python_json(py, &self.inner.prune(max_selected))
    }

    fn export(&self, py: Python<'_>) -> PyResult<PyObject> {
        to_python_json(py, &self.inner.export())
    }

    fn why_selected(&self, py: Python<'_>, selector: &str) -> PyResult<PyObject> {
        to_python_json(
            py,
            &self
                .inner
                .why_selected(selector)
                .map_err(to_runtime_error)?,
        )
    }

    fn diff(&self, py: Python<'_>, other: &PyGraphSession) -> PyResult<PyObject> {
        to_python_json(py, &self.inner.diff(&other.inner))
    }
}

fn parse_detail_level(value: &str) -> PyResult<ucp_api::GraphDetailLevel> {
    match value {
        "stub" => Ok(ucp_api::GraphDetailLevel::Stub),
        "summary" => Ok(ucp_api::GraphDetailLevel::Summary),
        "full" => Ok(ucp_api::GraphDetailLevel::Full),
        _ => Err(pyo3::exceptions::PyValueError::new_err(format!(
            "Unsupported detail level: {}",
            value
        ))),
    }
}

fn parse_neighbor_mode(value: &str) -> PyResult<ucp_api::GraphNeighborMode> {
    match value {
        "children" => Ok(ucp_api::GraphNeighborMode::Children),
        "parents" => Ok(ucp_api::GraphNeighborMode::Parents),
        "outgoing" => Ok(ucp_api::GraphNeighborMode::Outgoing),
        "incoming" => Ok(ucp_api::GraphNeighborMode::Incoming),
        "neighborhood" => Ok(ucp_api::GraphNeighborMode::Neighborhood),
        _ => Err(pyo3::exceptions::PyValueError::new_err(format!(
            "Unsupported neighbor mode: {}",
            value
        ))),
    }
}

fn to_runtime_error(err: impl std::fmt::Display) -> PyErr {
    pyo3::exceptions::PyRuntimeError::new_err(err.to_string())
}
