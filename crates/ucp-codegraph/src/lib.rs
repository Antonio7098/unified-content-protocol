mod context;
mod legacy;
mod model;
mod programmatic;
mod projection;

pub use context::{
    approximate_prompt_tokens, export_codegraph_context, export_codegraph_context_with_config,
    is_codegraph_document, render_codegraph_context_prompt, resolve_codegraph_selector,
    CodeGraphCoderef, CodeGraphContextEdgeExport, CodeGraphContextExport,
    CodeGraphContextFrontierAction, CodeGraphContextHeuristics, CodeGraphContextNodeExport,
    CodeGraphContextSession, CodeGraphContextSummary, CodeGraphContextUpdate, CodeGraphDetailLevel,
    CodeGraphExportConfig, CodeGraphExportMode, CodeGraphExportOmissionDetail,
    CodeGraphExportOmissionReason, CodeGraphExportOmissionReport, CodeGraphHiddenLevelSummary,
    CodeGraphOperationBudget, CodeGraphPersistedSession, CodeGraphPrunePolicy,
    CodeGraphRecommendation, CodeGraphRenderConfig, CodeGraphSelectionOrigin,
    CodeGraphSelectionOriginKind, CodeGraphSessionEvent, CodeGraphSessionMutation,
    CodeGraphSessionMutationKind, CodeGraphSessionPersistenceMetadata, CodeGraphTraversalConfig,
    HydratedSourceExcerpt,
};
pub use legacy::{
    build_code_graph, build_code_graph_incremental, canonical_codegraph_json,
    canonical_fingerprint, validate_code_graph_profile,
};
pub use model::{
    CodeGraphBuildInput, CodeGraphBuildResult, CodeGraphBuildStatus, CodeGraphDiagnostic,
    CodeGraphExtractorConfig, CodeGraphIncrementalBuildInput, CodeGraphIncrementalStats,
    CodeGraphSeverity, CodeGraphStats, CodeGraphValidationResult, CODEGRAPH_EXTRACTOR_VERSION,
    CODEGRAPH_PROFILE_MARKER, CODEGRAPH_PROFILE_VERSION,
};
pub use programmatic::{
    CodeGraphExpandMode, CodeGraphExportOmissionExplanation, CodeGraphFindQuery,
    CodeGraphMutationEstimate, CodeGraphNavigator, CodeGraphNavigatorSession, CodeGraphNodeSummary,
    CodeGraphPathHop, CodeGraphPathResult, CodeGraphProvenanceStep, CodeGraphPruneExplanation,
    CodeGraphRecommendedActionsResult, CodeGraphSelectionExplanation,
    CodeGraphSelectorResolutionExplanation, CodeGraphSessionDiff,
};
pub use projection::{
    codegraph_prompt_projection, codegraph_prompt_projection_with_config,
    CodeGraphPromptProjectionConfig,
};
pub use ucm_core::PortableDocument;
