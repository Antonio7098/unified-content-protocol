use std::{fs, path::PathBuf};

use pyo3::prelude::*;

use crate::{document::PyDocument, json::to_python_json, types::PyBlockId};

#[pyclass(unsendable, name = "CodeGraph")]
#[derive(Clone)]
pub struct PyCodeGraph {
    inner: ucp_api::CodeGraphNavigator,
}

#[pyclass(unsendable, name = "CodeGraphSession")]
#[derive(Clone)]
pub struct PyCodeGraphSession {
    inner: ucp_api::CodeGraphNavigatorSession,
}

#[pymethods]
impl PyCodeGraph {
    #[staticmethod]
    #[pyo3(signature = (repo_path, commit_hash=None, include_hidden=false, continue_on_parse_error=true, max_file_bytes=None, emit_export_edges=true, include_extensions=None, exclude_dirs=None))]
    fn build(
        repo_path: &str,
        commit_hash: Option<String>,
        include_hidden: bool,
        continue_on_parse_error: bool,
        max_file_bytes: Option<usize>,
        emit_export_edges: bool,
        include_extensions: Option<Vec<String>>,
        exclude_dirs: Option<Vec<String>>,
    ) -> PyResult<Self> {
        let mut config = ucp_api::CodeGraphExtractorConfig::default();
        config.include_hidden = include_hidden;
        config.continue_on_parse_error = continue_on_parse_error;
        config.emit_export_edges = emit_export_edges;
        if let Some(value) = max_file_bytes {
            config.max_file_bytes = value;
        }
        if let Some(value) = include_extensions {
            config.include_extensions = value;
        }
        if let Some(value) = exclude_dirs {
            config.exclude_dirs = value;
        }

        let graph = ucp_api::CodeGraphNavigator::build(&ucp_api::CodeGraphBuildInput {
            repository_path: PathBuf::from(repo_path),
            commit_hash: commit_hash.unwrap_or_else(|| "HEAD".to_string()),
            config,
        })
        .map_err(to_runtime_error)?;
        Ok(Self { inner: graph })
    }

    #[staticmethod]
    fn from_document(doc: &PyDocument) -> PyResult<Self> {
        if !ucp_api::is_codegraph_document(doc.inner()) {
            return Err(pyo3::exceptions::PyValueError::new_err(
                "Document is not a codegraph document",
            ));
        }
        Ok(Self {
            inner: ucp_api::CodeGraphNavigator::new(doc.inner().clone()),
        })
    }

    #[staticmethod]
    fn from_json(payload: &str) -> PyResult<Self> {
        let portable: ucp_api::PortableDocument =
            serde_json::from_str(payload).map_err(to_value_error)?;
        let document = portable.to_document().map_err(to_runtime_error)?;
        Ok(Self {
            inner: ucp_api::CodeGraphNavigator::new(document),
        })
    }

    #[staticmethod]
    fn load(path: &str) -> PyResult<Self> {
        Self::from_json(&fs::read_to_string(path).map_err(to_runtime_error)?)
    }

    fn save(&self, path: &str) -> PyResult<()> {
        fs::write(path, self.to_json()?).map_err(to_runtime_error)
    }

    fn to_json(&self) -> PyResult<String> {
        serde_json::to_string_pretty(&ucp_api::PortableDocument::from_document(
            self.inner.document(),
        ))
        .map_err(to_value_error)
    }

    fn to_document(&self) -> PyDocument {
        PyDocument::new(self.inner.document().clone())
    }

    fn session(&self) -> PyCodeGraphSession {
        PyCodeGraphSession {
            inner: self.inner.session(),
        }
    }

    fn resolve(&self, selector: &str) -> Option<PyBlockId> {
        self.inner.resolve_selector(selector).map(PyBlockId::from)
    }

    fn describe(&self, py: Python<'_>, selector: &str) -> PyResult<Option<PyObject>> {
        Ok(self
            .inner
            .resolve_selector(selector)
            .and_then(|id| self.inner.describe_node(id))
            .map(|node| to_python_json(py, &node))
            .transpose()?)
    }

    #[pyo3(signature = (node_class=None, name_regex=None, path_regex=None, logical_key_regex=None, exported=None, case_sensitive=false, limit=None))]
    fn find_nodes(
        &self,
        py: Python<'_>,
        node_class: Option<String>,
        name_regex: Option<String>,
        path_regex: Option<String>,
        logical_key_regex: Option<String>,
        exported: Option<bool>,
        case_sensitive: bool,
        limit: Option<usize>,
    ) -> PyResult<PyObject> {
        let query = build_find_query(
            node_class,
            name_regex,
            path_regex,
            logical_key_regex,
            exported,
            case_sensitive,
            limit,
        );
        to_python_json(
            py,
            &self.inner.find_nodes(&query).map_err(to_runtime_error)?,
        )
    }

    #[pyo3(signature = (start, end, max_hops=8))]
    fn path_between(
        &self,
        py: Python<'_>,
        start: &str,
        end: &str,
        max_hops: usize,
    ) -> PyResult<Option<PyObject>> {
        let start_id = self.inner.resolve_selector(start).ok_or_else(|| {
            pyo3::exceptions::PyValueError::new_err(format!("Unknown selector: {}", start))
        })?;
        let end_id = self.inner.resolve_selector(end).ok_or_else(|| {
            pyo3::exceptions::PyValueError::new_err(format!("Unknown selector: {}", end))
        })?;
        self.inner
            .path_between(start_id, end_id, max_hops)
            .map(|path| to_python_json(py, &path))
            .transpose()
    }

    fn __repr__(&self) -> String {
        format!("CodeGraph(nodes={})", self.inner.document().blocks.len())
    }
}

#[pymethods]
impl PyCodeGraphSession {
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

    #[pyo3(signature = (target=None))]
    fn focus(&mut self, py: Python<'_>, target: Option<&str>) -> PyResult<PyObject> {
        to_python_json(py, &self.inner.focus(target).map_err(to_runtime_error)?)
    }

    #[pyo3(signature = (target, detail_level="symbol_card"))]
    fn select(&mut self, py: Python<'_>, target: &str, detail_level: &str) -> PyResult<PyObject> {
        let detail_level = parse_detail_level(detail_level)?;
        to_python_json(
            py,
            &self
                .inner
                .select(target, detail_level)
                .map_err(to_runtime_error)?,
        )
    }

    #[pyo3(signature = (target, mode="dependencies", relation=None, relations=None, depth=1, max_add=None, priority_threshold=None))]
    fn expand(
        &mut self,
        py: Python<'_>,
        target: &str,
        mode: &str,
        relation: Option<String>,
        relations: Option<Vec<String>>,
        depth: usize,
        max_add: Option<usize>,
        priority_threshold: Option<u16>,
    ) -> PyResult<PyObject> {
        let mode = parse_expand_mode(mode)?;
        let traversal = build_traversal(relation, relations, depth, max_add, priority_threshold);
        to_python_json(
            py,
            &self
                .inner
                .expand(target, mode, &traversal)
                .map_err(to_runtime_error)?,
        )
    }

    #[pyo3(signature = (target, padding=2))]
    fn hydrate(&mut self, py: Python<'_>, target: &str, padding: usize) -> PyResult<PyObject> {
        to_python_json(
            py,
            &self
                .inner
                .hydrate_source(target, padding)
                .map_err(to_runtime_error)?,
        )
    }

    #[pyo3(signature = (target, include_descendants=false))]
    fn collapse(
        &mut self,
        py: Python<'_>,
        target: &str,
        include_descendants: bool,
    ) -> PyResult<PyObject> {
        to_python_json(
            py,
            &self
                .inner
                .collapse(target, include_descendants)
                .map_err(to_runtime_error)?,
        )
    }

    #[pyo3(signature = (target, pinned=true))]
    fn pin(&mut self, py: Python<'_>, target: &str, pinned: bool) -> PyResult<PyObject> {
        to_python_json(
            py,
            &self.inner.pin(target, pinned).map_err(to_runtime_error)?,
        )
    }

    #[pyo3(signature = (max_selected=None))]
    fn prune(&mut self, py: Python<'_>, max_selected: Option<usize>) -> PyResult<PyObject> {
        to_python_json(py, &self.inner.prune(max_selected))
    }

    #[pyo3(signature = (max_tokens=None, compact=false, include_rendered=None, visible_levels=None, only_node_classes=None, exclude_node_classes=None, max_frontier_actions=None))]
    fn export(
        &self,
        py: Python<'_>,
        max_tokens: Option<usize>,
        compact: bool,
        include_rendered: Option<bool>,
        visible_levels: Option<usize>,
        only_node_classes: Option<Vec<String>>,
        exclude_node_classes: Option<Vec<String>>,
        max_frontier_actions: Option<usize>,
    ) -> PyResult<PyObject> {
        let render = render_config(max_tokens);
        let export = export_config(
            compact,
            include_rendered,
            visible_levels,
            only_node_classes,
            exclude_node_classes,
            max_frontier_actions,
        );
        to_python_json(py, &self.inner.export(&render, &export))
    }

    #[pyo3(signature = (max_tokens=None))]
    fn render_prompt(&self, max_tokens: Option<usize>) -> String {
        self.inner.render_prompt(&render_config(max_tokens))
    }

    #[pyo3(signature = (top=1, padding=2, depth=None, max_add=None, priority_threshold=None))]
    fn apply_recommended(
        &mut self,
        py: Python<'_>,
        top: usize,
        padding: usize,
        depth: Option<usize>,
        max_add: Option<usize>,
        priority_threshold: Option<u16>,
    ) -> PyResult<PyObject> {
        to_python_json(
            py,
            &self
                .inner
                .apply_recommended_actions(top, padding, depth, max_add, priority_threshold)
                .map_err(to_runtime_error)?,
        )
    }

    fn why_selected(&self, py: Python<'_>, target: &str) -> PyResult<PyObject> {
        to_python_json(
            py,
            &self.inner.why_selected(target).map_err(to_runtime_error)?,
        )
    }

    #[pyo3(signature = (other))]
    fn diff(&self, py: Python<'_>, other: &PyCodeGraphSession) -> PyResult<PyObject> {
        to_python_json(py, &self.inner.diff(&other.inner))
    }

    #[pyo3(signature = (start, end, max_hops=8))]
    fn path_between(
        &self,
        py: Python<'_>,
        start: &str,
        end: &str,
        max_hops: usize,
    ) -> PyResult<Option<PyObject>> {
        self.inner
            .path_between(start, end, max_hops)
            .map_err(to_runtime_error)?
            .map(|path| to_python_json(py, &path))
            .transpose()
    }

    #[pyo3(signature = (node_class=None, name_regex=None, path_regex=None, logical_key_regex=None, exported=None, case_sensitive=false, limit=None))]
    fn find_nodes(
        &self,
        py: Python<'_>,
        node_class: Option<String>,
        name_regex: Option<String>,
        path_regex: Option<String>,
        logical_key_regex: Option<String>,
        exported: Option<bool>,
        case_sensitive: bool,
        limit: Option<usize>,
    ) -> PyResult<PyObject> {
        let query = build_find_query(
            node_class,
            name_regex,
            path_regex,
            logical_key_regex,
            exported,
            case_sensitive,
            limit,
        );
        to_python_json(
            py,
            &self.inner.find_nodes(&query).map_err(to_runtime_error)?,
        )
    }

    fn __repr__(&self) -> String {
        format!(
            "CodeGraphSession(selected={})",
            self.inner.selected_block_ids().len()
        )
    }
}

fn build_find_query(
    node_class: Option<String>,
    name_regex: Option<String>,
    path_regex: Option<String>,
    logical_key_regex: Option<String>,
    exported: Option<bool>,
    case_sensitive: bool,
    limit: Option<usize>,
) -> ucp_api::CodeGraphFindQuery {
    ucp_api::CodeGraphFindQuery {
        node_class,
        name_regex,
        path_regex,
        logical_key_regex,
        case_sensitive,
        exported,
        limit,
    }
}

fn build_traversal(
    relation: Option<String>,
    relations: Option<Vec<String>>,
    depth: usize,
    max_add: Option<usize>,
    priority_threshold: Option<u16>,
) -> ucp_api::CodeGraphTraversalConfig {
    let mut relation_filters = relations.unwrap_or_default();
    if let Some(value) = relation {
        relation_filters.push(value);
    }
    ucp_api::CodeGraphTraversalConfig {
        depth: depth.max(1),
        relation_filters,
        max_add,
        priority_threshold,
    }
}

fn render_config(max_tokens: Option<usize>) -> ucp_api::CodeGraphRenderConfig {
    max_tokens
        .map(ucp_api::CodeGraphRenderConfig::for_max_tokens)
        .unwrap_or_default()
}

fn export_config(
    compact: bool,
    include_rendered: Option<bool>,
    visible_levels: Option<usize>,
    only_node_classes: Option<Vec<String>>,
    exclude_node_classes: Option<Vec<String>>,
    max_frontier_actions: Option<usize>,
) -> ucp_api::CodeGraphExportConfig {
    let mut config = if compact {
        ucp_api::CodeGraphExportConfig::compact()
    } else {
        ucp_api::CodeGraphExportConfig::default()
    };
    if let Some(value) = include_rendered {
        config.include_rendered = value;
    }
    config.visible_levels = visible_levels;
    config.only_node_classes = only_node_classes.unwrap_or_default();
    config.exclude_node_classes = exclude_node_classes.unwrap_or_default();
    if let Some(value) = max_frontier_actions {
        config.max_frontier_actions = value.max(1);
    }
    config
}

fn parse_detail_level(value: &str) -> PyResult<ucp_api::CodeGraphDetailLevel> {
    match value {
        "skeleton" => Ok(ucp_api::CodeGraphDetailLevel::Skeleton),
        "symbol_card" | "symbol-card" => Ok(ucp_api::CodeGraphDetailLevel::SymbolCard),
        "neighborhood" => Ok(ucp_api::CodeGraphDetailLevel::Neighborhood),
        "source" => Ok(ucp_api::CodeGraphDetailLevel::Source),
        _ => Err(pyo3::exceptions::PyValueError::new_err(format!(
            "Unsupported detail level: {}",
            value
        ))),
    }
}

fn parse_expand_mode(value: &str) -> PyResult<ucp_api::CodeGraphExpandMode> {
    match value {
        "file" => Ok(ucp_api::CodeGraphExpandMode::File),
        "dependencies" => Ok(ucp_api::CodeGraphExpandMode::Dependencies),
        "dependents" => Ok(ucp_api::CodeGraphExpandMode::Dependents),
        _ => Err(pyo3::exceptions::PyValueError::new_err(format!(
            "Unsupported expand mode: {}",
            value
        ))),
    }
}

fn to_runtime_error(err: impl std::fmt::Display) -> PyErr {
    pyo3::exceptions::PyRuntimeError::new_err(err.to_string())
}

fn to_value_error(err: impl std::fmt::Display) -> PyErr {
    pyo3::exceptions::PyValueError::new_err(err.to_string())
}
