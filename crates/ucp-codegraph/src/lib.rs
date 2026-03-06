mod legacy;
mod model;
mod projection;

pub use legacy::{
    build_code_graph, canonical_codegraph_json, canonical_fingerprint, validate_code_graph_profile,
};
pub use model::{
    CodeGraphBuildInput, CodeGraphBuildResult, CodeGraphBuildStatus, CodeGraphDiagnostic,
    CodeGraphExtractorConfig, CodeGraphSeverity, CodeGraphStats, CodeGraphValidationResult,
    PortableDocument, CODEGRAPH_EXTRACTOR_VERSION, CODEGRAPH_PROFILE_MARKER,
    CODEGRAPH_PROFILE_VERSION,
};
pub use projection::{
    codegraph_prompt_projection, codegraph_prompt_projection_with_config,
    CodeGraphPromptProjectionConfig,
};
