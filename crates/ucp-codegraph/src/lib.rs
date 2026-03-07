mod context;
mod legacy;
mod model;
mod projection;

pub use context::{
    approximate_prompt_tokens, export_codegraph_context, export_codegraph_context_with_config,
    is_codegraph_document, render_codegraph_context_prompt, resolve_codegraph_selector,
    CodeGraphCoderef, CodeGraphContextEdgeExport, CodeGraphContextExport,
    CodeGraphContextFrontierAction, CodeGraphContextHeuristics, CodeGraphContextNodeExport,
    CodeGraphContextSession, CodeGraphContextSummary, CodeGraphContextUpdate, CodeGraphDetailLevel,
    CodeGraphExportConfig, CodeGraphExportMode, CodeGraphHiddenLevelSummary, CodeGraphPrunePolicy,
    CodeGraphRenderConfig, CodeGraphSelectionOrigin, CodeGraphSelectionOriginKind,
    CodeGraphTraversalConfig, HydratedSourceExcerpt,
};
pub use legacy::{
    build_code_graph, build_code_graph_incremental, canonical_codegraph_json,
    canonical_fingerprint, validate_code_graph_profile,
};
pub use model::{
    CodeGraphBuildInput, CodeGraphBuildResult, CodeGraphBuildStatus, CodeGraphDiagnostic,
    CodeGraphExtractorConfig, CodeGraphIncrementalBuildInput, CodeGraphIncrementalStats,
    CodeGraphSeverity, CodeGraphStats, CodeGraphValidationResult, PortableDocument,
    CODEGRAPH_EXTRACTOR_VERSION, CODEGRAPH_PROFILE_MARKER, CODEGRAPH_PROFILE_VERSION,
};
pub use projection::{
    codegraph_prompt_projection, codegraph_prompt_projection_with_config,
    CodeGraphPromptProjectionConfig,
};
