//! Generic graph runtime for UCP documents.

mod navigator;
mod query;
mod session;
mod store;
mod types;

pub use navigator::GraphNavigator;
pub use query::{GraphFindQuery, GraphNeighborMode};
pub use session::{
    GraphExport, GraphExportEdge, GraphExportNode, GraphSelectionExplanation, GraphSelectionOrigin,
    GraphSelectionOriginKind, GraphSession, GraphSessionDiff, GraphSessionNode,
    GraphSessionSummary, GraphSessionUpdate,
};
#[cfg(not(target_arch = "wasm32"))]
pub use store::SqliteGraphStore;
pub use store::{
    GraphNodeRecord, GraphStore, GraphStoreError, GraphStoreObservability, GraphStoreStats,
    InMemoryGraphStore,
};
pub use types::{
    GraphDetailLevel, GraphEdgeSummary, GraphNodeSummary, GraphPathHop, GraphPathResult,
};
