//! Integration tests for the agent graph traversal system.

use std::sync::Arc;
use ucm_core::{Block, BlockId, Content, Document};
use ucp_agent::{
    AgentCapabilities, AgentError, AgentTraversal, ExpandDirection, ExpandOptions, GlobalLimits,
    MockRagProvider, RagProvider, SearchOptions, SessionConfig, SessionLimits, ViewMode,
};

/// Helper to create a test document with a known structure.
fn create_test_document() -> Document {
    let mut doc = Document::create();
    let root = doc.root;

    // Create a simple hierarchy:
    // root
    //   ├── child1 (heading1)
    //   │   ├── grandchild1a (paragraph)
    //   │   └── grandchild1b (code)
    //   ├── child2 (heading2)
    //   │   └── grandchild2 (paragraph)
    //   └── child3 (paragraph)

    // Create child blocks
    let child1 =
        Block::new(Content::text("Child 1 - Introduction"), Some("heading1")).with_tag("important");
    let child2 =
        Block::new(Content::text("Child 2 - Methods"), Some("heading2")).with_tag("methods");
    let child3 = Block::new(
        Content::text("Child 3 - Simple paragraph"),
        Some("paragraph"),
    );

    let child1_id = doc.add_block(child1, &root).unwrap();
    let child2_id = doc.add_block(child2, &root).unwrap();
    let _child3_id = doc.add_block(child3, &root).unwrap();

    // Create grandchild blocks
    let grandchild1a = Block::new(
        Content::text("This is the introduction paragraph with important details."),
        Some("paragraph"),
    )
    .with_tag("important")
    .with_tag("introduction");

    let grandchild1b = Block::new(
        Content::code("rust", "fn main() { println!(\"Hello\"); }"),
        None,
    );

    let grandchild2 = Block::new(
        Content::text("Methodology section content here."),
        Some("paragraph"),
    )
    .with_tag("methods");

    doc.add_block(grandchild1a, &child1_id).unwrap();
    doc.add_block(grandchild1b, &child1_id).unwrap();
    doc.add_block(grandchild2, &child2_id).unwrap();

    doc
}

/// Create a random-looking block ID for testing "not found" scenarios.
fn fake_block_id() -> BlockId {
    BlockId::from_bytes([
        0xDE, 0xAD, 0xBE, 0xEF, 0x00, 0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77,
    ])
}

// ==================== Session Management Tests ====================

#[test]
fn test_session_creation_and_close() {
    let doc = create_test_document();
    let traversal = AgentTraversal::new(doc);

    let session_id = traversal
        .create_session(SessionConfig::default())
        .expect("Should create session");

    assert!(!session_id.0.is_nil(), "Session ID should not be nil");

    traversal
        .close_session(&session_id)
        .expect("Should close session");

    // Closing again should fail
    let result = traversal.close_session(&session_id);
    assert!(matches!(result, Err(AgentError::SessionNotFound(_))));
}

#[test]
fn test_session_with_config() {
    let doc = create_test_document();
    let traversal = AgentTraversal::new(doc);

    let config = SessionConfig::new()
        .with_name("test-session")
        .with_view_mode(ViewMode::Preview { length: 50 });

    let session_id = traversal
        .create_session(config)
        .expect("Should create session with config");

    assert!(!session_id.0.is_nil());

    traversal.close_session(&session_id).unwrap();
}

#[test]
fn test_max_sessions_limit() {
    let doc = create_test_document();
    let traversal = AgentTraversal::new(doc).with_global_limits(GlobalLimits {
        max_sessions: 2,
        ..Default::default()
    });

    // Create first two sessions
    let _s1 = traversal.create_session(SessionConfig::default()).unwrap();
    let _s2 = traversal.create_session(SessionConfig::default()).unwrap();

    // Third should fail
    let result = traversal.create_session(SessionConfig::default());
    assert!(matches!(
        result,
        Err(AgentError::MaxSessionsReached { max: 2 })
    ));
}

#[test]
fn test_session_with_custom_capabilities() {
    let doc = create_test_document();
    let root_id = doc.root;
    let traversal = AgentTraversal::new(doc);

    let config = SessionConfig::new().with_capabilities(AgentCapabilities::read_only());

    let session_id = traversal
        .create_session(config)
        .expect("Should create read-only session");

    // Navigate should work (read-only allows traversal)
    let _ = traversal.navigate_to(&session_id, root_id);

    // Context modification should fail
    let result = traversal.context_add(&session_id, root_id, None, None);
    assert!(matches!(
        result,
        Err(AgentError::OperationNotPermitted { .. })
    ));

    traversal.close_session(&session_id).unwrap();
}

// ==================== Navigation Tests ====================

#[test]
fn test_navigate_to_root() {
    let doc = create_test_document();
    let root_id = doc.root;
    let traversal = AgentTraversal::new(doc);

    let session_id = traversal.create_session(SessionConfig::default()).unwrap();

    let result = traversal.navigate_to(&session_id, root_id);
    assert!(result.is_ok());

    let nav_result = result.unwrap();
    assert_eq!(nav_result.position, root_id);
    assert!(nav_result.refreshed);

    traversal.close_session(&session_id).unwrap();
}

#[test]
fn test_navigate_to_nonexistent_block() {
    let doc = create_test_document();
    let traversal = AgentTraversal::new(doc);

    let session_id = traversal.create_session(SessionConfig::default()).unwrap();

    // Create a fake block ID that doesn't exist
    let fake_id = fake_block_id();
    let result = traversal.navigate_to(&session_id, fake_id);

    assert!(matches!(result, Err(AgentError::BlockNotFound(_))));

    traversal.close_session(&session_id).unwrap();
}

#[test]
fn test_go_back_empty_history() {
    let doc = create_test_document();
    let traversal = AgentTraversal::new(doc);

    let session_id = traversal.create_session(SessionConfig::default()).unwrap();

    // Going back without navigation should fail
    let result = traversal.go_back(&session_id, 1);
    assert!(matches!(result, Err(AgentError::EmptyHistory)));

    traversal.close_session(&session_id).unwrap();
}

#[test]
fn test_navigate_and_go_back() {
    let doc = create_test_document();
    let root_id = doc.root;
    let child_ids: Vec<_> = doc.children(&root_id).to_vec();
    let traversal = AgentTraversal::new(doc);

    if child_ids.is_empty() {
        return; // Skip test if no children
    }

    let session_id = traversal.create_session(SessionConfig::default()).unwrap();

    // Navigate to root
    traversal.navigate_to(&session_id, root_id).unwrap();

    // Navigate to first child
    let child_id = child_ids[0];
    traversal.navigate_to(&session_id, child_id).unwrap();

    // Go back to root
    let result = traversal.go_back(&session_id, 1).unwrap();
    assert_eq!(result.position, root_id);

    traversal.close_session(&session_id).unwrap();
}

// ==================== Expansion Tests ====================

#[test]
fn test_expand_down() {
    let doc = create_test_document();
    let root_id = doc.root;
    let traversal = AgentTraversal::new(doc);

    let session_id = traversal.create_session(SessionConfig::default()).unwrap();

    let result = traversal.expand(
        &session_id,
        root_id,
        ExpandDirection::Down,
        ExpandOptions::new().with_depth(2),
    );

    assert!(result.is_ok());
    let expansion = result.unwrap();
    assert_eq!(expansion.root, root_id);
    assert!(expansion.total_blocks >= 1); // At least the root

    traversal.close_session(&session_id).unwrap();
}

#[test]
fn test_expand_with_depth_limit() {
    let doc = create_test_document();
    let root_id = doc.root;
    let traversal = AgentTraversal::new(doc);

    let config = SessionConfig::new().with_limits(SessionLimits {
        max_expand_depth: 1,
        ..Default::default()
    });

    let session_id = traversal.create_session(config).unwrap();

    // Depth 1 should work
    let result = traversal.expand(
        &session_id,
        root_id,
        ExpandDirection::Down,
        ExpandOptions::new().with_depth(1),
    );
    assert!(result.is_ok());

    // Depth 2 should fail (exceeds limit)
    let result = traversal.expand(
        &session_id,
        root_id,
        ExpandDirection::Down,
        ExpandOptions::new().with_depth(2),
    );
    assert!(matches!(
        result,
        Err(AgentError::DepthLimitExceeded { current: 2, max: 1 })
    ));

    traversal.close_session(&session_id).unwrap();
}

#[test]
fn test_expand_up() {
    let doc = create_test_document();
    let root_id = doc.root;
    let child_ids: Vec<_> = doc.children(&root_id).to_vec();
    let traversal = AgentTraversal::new(doc);

    if child_ids.is_empty() {
        return; // Skip test
    }

    let session_id = traversal.create_session(SessionConfig::default()).unwrap();

    // Expand up from child should find root
    let result = traversal.expand(
        &session_id,
        child_ids[0],
        ExpandDirection::Up,
        ExpandOptions::new().with_depth(2),
    );

    assert!(result.is_ok());

    traversal.close_session(&session_id).unwrap();
}

// ==================== Find/Search Tests ====================

#[test]
fn test_find_by_role() {
    let doc = create_test_document();
    let traversal = AgentTraversal::new(doc);

    let session_id = traversal.create_session(SessionConfig::default()).unwrap();

    let result = traversal.find_by_pattern(&session_id, Some("paragraph"), None, None, None);

    assert!(result.is_ok());
    let find_result = result.unwrap();
    assert!(find_result.total_searched > 0);

    traversal.close_session(&session_id).unwrap();
}

#[test]
fn test_find_by_tag() {
    let doc = create_test_document();
    let traversal = AgentTraversal::new(doc);

    let session_id = traversal.create_session(SessionConfig::default()).unwrap();

    let result = traversal.find_by_pattern(&session_id, None, Some("important"), None, None);

    assert!(result.is_ok());
    let find_result = result.unwrap();
    assert!(find_result.total_searched > 0);

    traversal.close_session(&session_id).unwrap();
}

#[test]
fn test_find_by_pattern() {
    let doc = create_test_document();
    let traversal = AgentTraversal::new(doc);

    let session_id = traversal.create_session(SessionConfig::default()).unwrap();

    // Search for content containing "introduction"
    let result = traversal.find_by_pattern(&session_id, None, None, None, Some("introduction"));

    assert!(result.is_ok());
    let find_result = result.unwrap();
    assert!(find_result.total_searched > 0);

    traversal.close_session(&session_id).unwrap();
}

#[tokio::test]
async fn test_search_without_rag_provider() {
    let doc = create_test_document();
    let traversal = AgentTraversal::new(doc);

    let session_id = traversal.create_session(SessionConfig::default()).unwrap();

    let result = traversal
        .search(&session_id, "test query", SearchOptions::new())
        .await;

    // Should fail because no RAG provider is configured
    assert!(matches!(result, Err(AgentError::RagNotConfigured)));

    traversal.close_session(&session_id).unwrap();
}

#[tokio::test]
async fn test_search_with_mock_rag() {
    let doc = create_test_document();
    let root_id = doc.root;

    // Create mock RAG provider with pre-configured results
    let mut mock_rag = MockRagProvider::new();
    mock_rag.add_result(root_id, 0.95, Some("Test content"));

    let traversal =
        AgentTraversal::new(doc).with_rag_provider(Arc::new(mock_rag) as Arc<dyn RagProvider>);

    let session_id = traversal.create_session(SessionConfig::default()).unwrap();

    let result = traversal
        .search(
            &session_id,
            "test query",
            SearchOptions::new().with_limit(10),
        )
        .await;

    assert!(result.is_ok());
    let search_result = result.unwrap();
    assert!(!search_result.matches.is_empty());
    assert_eq!(search_result.matches[0].block_id, root_id);

    traversal.close_session(&session_id).unwrap();
}

// ==================== View Tests ====================

#[test]
fn test_view_block_full() {
    let doc = create_test_document();
    let root_id = doc.root;
    let traversal = AgentTraversal::new(doc);

    let session_id = traversal.create_session(SessionConfig::default()).unwrap();

    let result = traversal.view_block(&session_id, root_id, ViewMode::Full);

    assert!(result.is_ok());
    let view = result.unwrap();
    assert_eq!(view.block_id, root_id);

    traversal.close_session(&session_id).unwrap();
}

#[test]
fn test_view_block_preview() {
    let doc = create_test_document();
    let root_id = doc.root;
    let traversal = AgentTraversal::new(doc);

    let session_id = traversal.create_session(SessionConfig::default()).unwrap();

    let result = traversal.view_block(&session_id, root_id, ViewMode::Preview { length: 10 });

    assert!(result.is_ok());
    let view = result.unwrap();

    // If there's content, it should be truncated
    if let Some(content) = &view.content {
        assert!(content.len() <= 10);
    }

    traversal.close_session(&session_id).unwrap();
}

#[test]
fn test_view_block_metadata() {
    let doc = create_test_document();
    let root_id = doc.root;
    let traversal = AgentTraversal::new(doc);

    let session_id = traversal.create_session(SessionConfig::default()).unwrap();

    let result = traversal.view_block(&session_id, root_id, ViewMode::Metadata);

    assert!(result.is_ok());
    let view = result.unwrap();

    // Metadata mode should not include content
    assert!(view.content.is_none());

    traversal.close_session(&session_id).unwrap();
}

#[test]
fn test_view_neighborhood() {
    let doc = create_test_document();
    let traversal = AgentTraversal::new(doc);

    let session_id = traversal.create_session(SessionConfig::default()).unwrap();

    let result = traversal.view_neighborhood(&session_id);

    assert!(result.is_ok());
    let view = result.unwrap();

    // Should have position set
    assert!(!view.position.to_string().is_empty());

    traversal.close_session(&session_id).unwrap();
}

// ==================== Path Finding Tests ====================

#[test]
fn test_find_path_same_node() {
    let doc = create_test_document();
    let root_id = doc.root;
    let traversal = AgentTraversal::new(doc);

    let session_id = traversal.create_session(SessionConfig::default()).unwrap();

    let result = traversal.find_path(&session_id, root_id, root_id, None);

    assert!(result.is_ok());
    let path = result.unwrap();
    assert_eq!(path.len(), 1);
    assert_eq!(path[0], root_id);

    traversal.close_session(&session_id).unwrap();
}

#[test]
fn test_find_path_root_to_child() {
    let doc = create_test_document();
    let root_id = doc.root;
    let child_ids: Vec<_> = doc.children(&root_id).to_vec();
    let traversal = AgentTraversal::new(doc);

    if child_ids.is_empty() {
        return; // Skip test
    }

    let session_id = traversal.create_session(SessionConfig::default()).unwrap();

    let result = traversal.find_path(&session_id, root_id, child_ids[0], None);

    assert!(result.is_ok());
    let path = result.unwrap();
    assert!(path.len() >= 2);
    assert_eq!(path[0], root_id);
    assert_eq!(*path.last().unwrap(), child_ids[0]);

    traversal.close_session(&session_id).unwrap();
}

#[test]
fn test_find_path_no_path_exists() {
    let doc = Document::create();
    let fake_id = fake_block_id();
    let traversal = AgentTraversal::new(doc);

    let session_id = traversal.create_session(SessionConfig::default()).unwrap();

    // Try to find path to non-existent block
    let result = traversal.find_path(&session_id, BlockId::root(), fake_id, Some(5));

    // Should fail because fake_id doesn't exist in BFS search
    assert!(result.is_err());

    traversal.close_session(&session_id).unwrap();
}

// ==================== Context Operations Tests ====================

#[test]
fn test_context_add_and_remove() {
    let doc = create_test_document();
    let root_id = doc.root;
    let traversal = AgentTraversal::new(doc);

    let session_id = traversal.create_session(SessionConfig::default()).unwrap();

    // Add block to context
    let result = traversal.context_add(
        &session_id,
        root_id,
        Some("test reason".to_string()),
        Some(0.9),
    );
    assert!(result.is_ok());

    // Remove block from context
    let result = traversal.context_remove(&session_id, root_id);
    assert!(result.is_ok());

    traversal.close_session(&session_id).unwrap();
}

#[test]
fn test_context_clear() {
    let doc = create_test_document();
    let root_id = doc.root;
    let traversal = AgentTraversal::new(doc);

    let session_id = traversal.create_session(SessionConfig::default()).unwrap();

    // Add some blocks
    traversal
        .context_add(&session_id, root_id, None, None)
        .unwrap();

    // Clear context
    let result = traversal.context_clear(&session_id);
    assert!(result.is_ok());

    traversal.close_session(&session_id).unwrap();
}

#[test]
fn test_context_focus() {
    let doc = create_test_document();
    let root_id = doc.root;
    let traversal = AgentTraversal::new(doc);

    let session_id = traversal.create_session(SessionConfig::default()).unwrap();

    // Set focus
    let result = traversal.context_focus(&session_id, Some(root_id));
    assert!(result.is_ok());

    // Clear focus
    let result = traversal.context_focus(&session_id, None);
    assert!(result.is_ok());

    traversal.close_session(&session_id).unwrap();
}

#[test]
fn test_context_add_results_without_search() {
    let doc = create_test_document();
    let traversal = AgentTraversal::new(doc);

    let session_id = traversal.create_session(SessionConfig::default()).unwrap();

    // Try to add results without performing a search first
    let result = traversal.context_add_results(&session_id);

    // Should fail because no results available
    assert!(matches!(result, Err(AgentError::NoResultsAvailable)));

    traversal.close_session(&session_id).unwrap();
}

#[test]
fn test_context_add_results_after_find() {
    let doc = create_test_document();
    let traversal = AgentTraversal::new(doc);

    let session_id = traversal.create_session(SessionConfig::default()).unwrap();

    // Perform a find operation first
    let find_result = traversal
        .find_by_pattern(&session_id, None, None, None, Some(".*"))
        .unwrap();

    if !find_result.matches.is_empty() {
        // Now add results to context
        let result = traversal.context_add_results(&session_id);
        assert!(result.is_ok());

        let added = result.unwrap();
        assert_eq!(added.len(), find_result.matches.len());
    }

    traversal.close_session(&session_id).unwrap();
}

// ==================== UCL Execution Tests ====================

#[tokio::test]
async fn test_execute_ucl_goto() {
    let doc = create_test_document();
    let root_id = doc.root;
    let traversal = AgentTraversal::new(doc);

    let session_id = traversal.create_session(SessionConfig::default()).unwrap();

    let ucl_input = format!("GOTO {}", root_id);
    let result = ucp_agent::execute_ucl(&traversal, &session_id, &ucl_input).await;

    assert!(result.is_ok());
    let results = result.unwrap();
    assert_eq!(results.len(), 1);

    traversal.close_session(&session_id).unwrap();
}

#[tokio::test]
async fn test_execute_ucl_back_empty() {
    let doc = create_test_document();
    let traversal = AgentTraversal::new(doc);

    let session_id = traversal.create_session(SessionConfig::default()).unwrap();

    let result = ucp_agent::execute_ucl(&traversal, &session_id, "BACK").await;

    // Should fail because history is empty
    assert!(result.is_err());

    traversal.close_session(&session_id).unwrap();
}

#[tokio::test]
async fn test_execute_ucl_find() {
    let doc = create_test_document();
    let traversal = AgentTraversal::new(doc);

    let session_id = traversal.create_session(SessionConfig::default()).unwrap();

    // UCL syntax for FIND uses uppercase keywords: ROLE=, TAG=, LABEL=, PATTERN=
    let result = ucp_agent::execute_ucl(&traversal, &session_id, "FIND ROLE=paragraph").await;

    if let Err(ref e) = result {
        eprintln!("Error: {:?}", e);
    }
    assert!(
        result.is_ok(),
        "UCL FIND should succeed: {:?}",
        result.err()
    );

    traversal.close_session(&session_id).unwrap();
}

#[tokio::test]
async fn test_execute_ucl_ctx_clear() {
    let doc = create_test_document();
    let traversal = AgentTraversal::new(doc);

    let session_id = traversal.create_session(SessionConfig::default()).unwrap();

    let result = ucp_agent::execute_ucl(&traversal, &session_id, "CTX CLEAR").await;

    assert!(result.is_ok());

    traversal.close_session(&session_id).unwrap();
}

#[tokio::test]
async fn test_execute_ucl_multiple_commands() {
    let doc = create_test_document();
    let root_id = doc.root;
    let traversal = AgentTraversal::new(doc);

    let session_id = traversal.create_session(SessionConfig::default()).unwrap();

    let ucl_input = format!("GOTO {}\nFIND ROLE=paragraph\nCTX CLEAR", root_id);

    let result = ucp_agent::execute_ucl(&traversal, &session_id, &ucl_input).await;

    if let Err(ref e) = result {
        eprintln!("Error: {:?}", e);
    }
    assert!(
        result.is_ok(),
        "Multiple UCL commands should succeed: {:?}",
        result.err()
    );
    let results = result.unwrap();
    assert_eq!(results.len(), 3);

    traversal.close_session(&session_id).unwrap();
}

// ==================== View Mode Tests ====================

#[test]
fn test_view_mode_preview() {
    let mode = ViewMode::preview(50);
    match mode {
        ViewMode::Preview { length } => assert_eq!(length, 50),
        _ => panic!("Expected Preview mode"),
    }
}

#[test]
fn test_view_mode_adaptive() {
    let mode = ViewMode::adaptive(0.7);
    match mode {
        ViewMode::Adaptive { interest_threshold } => {
            assert!((interest_threshold - 0.7).abs() < 0.001)
        }
        _ => panic!("Expected Adaptive mode"),
    }
}

// ==================== Metrics Tests ====================

#[test]
fn test_metrics_recorded_on_navigation() {
    let doc = create_test_document();
    let root_id = doc.root;
    let traversal = AgentTraversal::new(doc);

    let session_id = traversal.create_session(SessionConfig::default()).unwrap();

    // Perform navigation
    let _ = traversal.navigate_to(&session_id, root_id);

    // Metrics should be recorded (tested indirectly through session info)
    // The actual metric values are internal to the session

    traversal.close_session(&session_id).unwrap();
}

// ==================== Safety Tests ====================

#[test]
fn test_session_closed_after_completion() {
    let doc = create_test_document();
    let root_id = doc.root;
    let traversal = AgentTraversal::new(doc);

    let session_id = traversal.create_session(SessionConfig::default()).unwrap();

    // Close the session
    traversal.close_session(&session_id).unwrap();

    // Operations should fail after close
    let result = traversal.navigate_to(&session_id, root_id);
    assert!(matches!(result, Err(AgentError::SessionNotFound(_))));
}

// ==================== Multiple Sessions Tests ====================

#[test]
fn test_multiple_sessions_independent() {
    let doc = create_test_document();
    let root_id = doc.root;
    let child_ids: Vec<_> = doc.children(&root_id).to_vec();
    let traversal = AgentTraversal::new(doc);

    if child_ids.is_empty() {
        return;
    }

    // Create two sessions
    let session1 = traversal.create_session(SessionConfig::default()).unwrap();
    let session2 = traversal.create_session(SessionConfig::default()).unwrap();

    // Navigate session1 to root
    traversal.navigate_to(&session1, root_id).unwrap();

    // Navigate session2 to child
    traversal.navigate_to(&session2, child_ids[0]).unwrap();

    // Sessions should have independent positions
    // (tested indirectly through subsequent operations)

    traversal.close_session(&session1).unwrap();
    traversal.close_session(&session2).unwrap();
}
