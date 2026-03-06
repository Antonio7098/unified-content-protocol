mod analyze;
mod build;
mod canonical;
mod extract;
mod filesystem;
mod resolve;
mod validate;
mod languages {
    pub(super) mod python;
    pub(super) mod rust;
    pub(super) mod ts_js;
}

pub use build::build_code_graph;
pub use canonical::{canonical_codegraph_json, canonical_fingerprint};
pub use validate::validate_code_graph_profile;

use analyze::{analyze_file, is_python_package_init};
use canonical::*;
use extract::*;
use filesystem::*;
use resolve::*;

#[cfg(test)]
mod tests;
