# Usage Examples

## Basic Navigation

Navigate through a document's structure:

=== "Rust"
    ```rust
    use ucp_agent::{AgentTraversal, SessionConfig, ExpandDirection, ExpandOptions};
    use ucm_core::Document;

    let doc = Document::create();
    let traversal = AgentTraversal::new(doc);
    let session = traversal.create_session(SessionConfig::default())?;

    // Navigate to root
    let nav = traversal.navigate_to(&session, doc.root)?;
    println!("Position: {}", nav.position);

    // Expand down to see children
    let expansion = traversal.expand(
        &session,
        nav.position,
        ExpandDirection::Down,
        ExpandOptions::new().with_depth(2),
    )?;

    println!("Found {} blocks in {} levels",
        expansion.total_blocks,
        expansion.levels.len()
    );

    // Navigate back to root
    traversal.navigate_to(&session, doc.root)?;
    let back_result = traversal.go_back(&session, 1)?;

    traversal.close_session(&session)?;
    ```

=== "Python"
    ```python
    from ucp import Document, AgentTraversal, SessionConfig, ExpandDirection

    doc = Document.create()
    traversal = AgentTraversal(doc)
    session = traversal.create_session(SessionConfig())

    # Navigate to root
    nav = traversal.navigate_to(session, doc.root)
    print(f"Position: {nav.position}")

    # Expand down
    expansion = traversal.expand(session, nav.position, "down", depth=2)
    print(f"Found {expansion.total_blocks} blocks")

    # Go back
    back = traversal.go_back(session)

    traversal.close_session(session)
    ```

=== "JavaScript"
    ```javascript
    const { Document, AgentTraversal, SessionConfig, ExpandDirection } = require('ucp');

    const doc = Document.create();
    const traversal = new AgentTraversal(doc);
    const session = traversal.createSession(new SessionConfig());

    // Navigate to root
    const nav = traversal.navigateTo(session, doc.root);
    console.log(`Position: ${nav.position}`);

    // Expand down
    const expansion = traversal.expand(
        session,
        nav.position,
        ExpandDirection.Down,
        null,
        new ExpandOptions().withDepth(2)
    );
    console.log(`Found ${expansion.totalBlocks} blocks`);

    // Go back
    const back = traversal.goBack(session);

    traversal.closeSession(session);
    ```

## Finding Blocks

Find blocks by role, tag, or pattern:

=== "Rust"
    ```rust
    // Find all paragraphs
    let paragraphs = traversal.find_by_pattern(
        &session,
        Some("paragraph"),
        None,
        None,
        None,
    )?;

    println!("Found {} paragraphs", paragraphs.matches.len());

    // Find blocks with "important" tag
    let important = traversal.find_by_pattern(
        &session,
        None,
        Some("important"),
        None,
        None,
    )?;

    // Find blocks containing "authentication"
    let auth_blocks = traversal.find_by_pattern(
        &session,
        None,
        None,
        None,
        Some("authentication"),
    )?;

    println!("Found {} blocks mentioning authentication", auth_blocks.matches.len());
    ```

=== "Python"
    ```python
    # Find all paragraphs
    paragraphs = traversal.find(session, role="paragraph")
    print(f"Found {len(paragraphs.matches)} paragraphs")

    # Find blocks with "important" tag
    important = traversal.find(session, tag="important")

    # Find blocks with multiple tags (use tags parameter)
    tagged = traversal.find(session, tags=["important", "urgent"])

    # Find blocks containing "authentication"
    auth_blocks = traversal.find(session, pattern="authentication")
    print(f"Found {len(auth_blocks.matches)} blocks mentioning authentication")
    ```

=== "JavaScript"
    ```javascript
    // Find all paragraphs
    const paragraphs = traversal.findByPattern(session, "paragraph");
    console.log(`Found ${paragraphs.matches.length} paragraphs`);

    // Find blocks with "important" tag
    const important = traversal.findByPattern(session, null, "important");

    // Find blocks with multiple tags (comma-separated)
    const tagged = traversal.findByPattern(session, null, null, "important,urgent");

    // Find blocks containing "authentication"
    const authBlocks = traversal.findByPattern(
        session,
        null,
        null,
        null,
        "authentication"
    );
    console.log(`Found ${authBlocks.matches.length} blocks mentioning authentication`);
    ```

## Syncing Document Changes

When you add blocks to the document after creating a traversal, you must call `update_document()` to sync the changes:

=== "Rust"
    ```rust
    let doc = Document::create();
    let traversal = AgentTraversal::new(doc.clone());
    let session = traversal.create_session(SessionConfig::default())?;

    // Add blocks to the document after creating traversal
    let block_id = doc.add_block(doc.root, "New content")?;

    // IMPORTANT: Update traversal to see the new block
    traversal.update_document(doc.clone())?;

    // Now you can navigate to the new block
    let nav = traversal.navigate_to(&session, block_id)?;
    println!("Navigated to: {}", nav.position);

    traversal.close_session(&session)?;
    ```

=== "Python"
    ```python
    doc = Document.create()
    traversal = AgentTraversal(doc)
    session = traversal.create_session()

    # Add blocks after creating traversal
    block_id = doc.add_block(doc.root_id, "New content")

    # Update traversal to sync changes
    traversal.update_document(doc)

    # Now navigation works
    nav = traversal.navigate_to(session, block_id)
    print(f"Navigated to: {nav.position}")

    traversal.close_session(session)
    ```

=== "JavaScript"
    ```javascript
    const doc = Document.create();
    const traversal = new WasmAgentTraversal(doc);
    const session = traversal.createSession();

    // Add blocks after creating traversal
    const blockId = doc.addBlock(doc.rootId, "New content");

    // Update traversal to sync changes
    traversal.updateDocument(doc);

    // Now you can navigate
    const nav = traversal.navigateTo(session, blockId);
    console.log(`Navigated to: ${nav.position}`);

    traversal.closeSession(session);
    ```

## Viewing Neighborhood

View the context around the current position (ancestors, children, siblings, connections):

=== "Rust"
    ```rust
    // Navigate to a block first
    traversal.navigate_to(&session, some_block_id)?;

    // View neighborhood around current position
    let neighborhood = traversal.view_neighborhood(&session)?;

    println!("Position: {}", neighborhood.position);
    println!("Ancestors: {}", neighborhood.ancestors.len());
    println!("Children: {}", neighborhood.children.len());
    println!("Siblings: {}", neighborhood.siblings.len());
    println!("Connections: {}", neighborhood.connections.len());

    // Process ancestors (parents)
    for ancestor in &neighborhood.ancestors {
        println!("Parent: {} ({})", ancestor.block_id, ancestor.role.as_deref().unwrap_or("unknown"));
    }

    // Process children
    for child in &neighborhood.children {
        println!("Child: {} ({} children)", child.block_id, child.children_count);
    }
    ```

=== "Python"
    ```python
    # Navigate first
    traversal.navigate_to(session, some_block_id)

    # View neighborhood
    neighborhood = traversal.view_neighborhood(session)

    print(f"Position: {neighborhood.position}")
    print(f"Ancestors: {len(neighborhood.ancestors)}")
    print(f"Children: {len(neighborhood.children)}")
    print(f"Siblings: {len(neighborhood.siblings)}")
    print(f"Connections: {len(neighborhood.connections)}")

    # Process ancestors
    for ancestor in neighborhood.ancestors:
        print(f"Parent: {ancestor.block_id} ({ancestor.role})")

    # Process children
    for child in neighborhood.children:
        print(f"Child: {child.block_id} ({child.children_count} children)")

    # Process semantic connections
    for conn in neighborhood.connections:
        print(f"Connected: {conn.block.block_id} via {conn.edge_type}")
    ```

=== "JavaScript"
    ```javascript
    // Navigate first
    traversal.navigateTo(session, someBlockId);

    // View neighborhood
    const neighborhood = traversal.viewNeighborhood(session);

    console.log(`Position: ${neighborhood.position}`);
    console.log(`Ancestors: ${neighborhood.ancestors.length}`);
    console.log(`Children: ${neighborhood.children.length}`);
    console.log(`Siblings: ${neighborhood.siblings.length}`);
    console.log(`Connections: ${neighborhood.connections.length}`);

    // Process ancestors
    for (const ancestor of neighborhood.ancestors) {
        console.log(`Parent: ${ancestor.blockId} (${ancestor.role})`);
    }

    // Process children
    for (const child of neighborhood.children) {
        console.log(`Child: ${child.blockId} (${child.childrenCount} children)`);
    }

    // Process semantic connections
    for (const conn of neighborhood.connections) {
        console.log(`Connected: ${conn.block.blockId} via ${conn.edgeType}`);
    }
    ```

## Context Window Management

The traversal crate emits CTX events rather than mutating an internal context
buffer, so these examples show how to trigger those events while your host
application listens for `ExecutionResult::Context` (Rust) or the equivalent in
Python/WASM to persist blocks, apply pruning, or render prompts.

=== "Rust"
    ```rust
    use ucp_agent::ViewMode;

    // Find relevant content
    let results = traversal.find_by_pattern(
        &session,
        None,
        Some("important"),
        None,
        None,
    )?;

    // Add search results to context
    for block_id in &results.matches {
        traversal.context_add(&session, *block_id, None, Some(0.9))?;
    }

    // Also add root for structure
    traversal.context_add(&session, doc.root, Some("structure".to_string()), Some(0.7))?;

    // Set focus to most relevant block (won't be pruned)
    if !results.matches.is_empty() {
        traversal.context_focus(&session, Some(results.matches[0]))?;
    }

    // View specific block with preview
    let view = traversal.view_block(
        &session,
        results.matches[0],
        ViewMode::Preview { length: 200 },
    )?;

    println!("Block preview: {}", view.content.unwrap_or_default());

    // When done, clear context
    traversal.context_clear(&session)?;
    ```

=== "Python"
    ```python
    from ucp import ViewMode

    # Find and add results to context
    results = traversal.find(session, tag="important")

    for block_id in results.matches:
        traversal.context_add(session, block_id, relevance=0.9)

    # Add root for structure
    traversal.context_add(session, doc.root, reason="structure", relevance=0.7)

    # Set focus
    if results.matches:
        traversal.context_focus(session, results.matches[0])

    # View with preview mode
    view = traversal.view_block(
        session,
        results.matches[0],
        ViewMode.preview(200)
    )
    print(f"Block preview: {view.content}")

    # Clear when done
    traversal.context_clear(session)
    ```

=== "JavaScript"
    ```javascript
    // Find and add results
    const results = traversal.findByPattern(session, null, "important");

    for (const blockId of results.matches) {
        traversal.contextAdd(session, blockId, null, 0.9);
    }

    // Add root for structure
    traversal.contextAdd(session, doc.root, "structure", 0.7);

    // Set focus
    if (results.matches.length > 0) {
        traversal.contextFocus(session, results.matches[0]);
    }

    // View with preview mode
    const view = traversal.viewBlock(session, results.matches[0], "preview");
    console.log(`Block preview: ${view.content}`);

    // Clear context
    traversal.contextClear(session);
    ```

## Semantic Search with RAG

Search using a semantic search provider:

=== "Rust"
    ```rust
    use std::sync::Arc;
    use ucp_agent::{MockRagProvider, RagProvider, SearchOptions};

    // Set up mock RAG provider (in real use, connect to actual RAG service)
    let mut rag = MockRagProvider::new();
    // Configure with results...

    let traversal = AgentTraversal::new(doc)
        .with_rag_provider(Arc::new(rag) as Arc<dyn RagProvider>);

    let session = traversal.create_session(SessionConfig::default())?;

    // Perform semantic search
    let results = traversal.search(
        &session,
        "How does authentication work?",
        SearchOptions::new()
            .with_limit(5)
            .with_min_similarity(0.7),
    ).await?;

    println!("Found {} relevant blocks", results.matches.len());

    // Add top results to context
    for match_ in &results.matches[..results.matches.len().min(3)] {
        traversal.context_add(
            &session,
            match_.block_id,
            None,
            Some(match_.similarity),
        )?;
    }

    // Add all results at once
    traversal.context_add_results(&session)?;

    traversal.close_session(&session)?;
    ```

=== "Python"
    ```python
    import asyncio
    from ucp import MockRagProvider, SearchOptions

    # Set up mock RAG provider
    rag = MockRagProvider()

    traversal = AgentTraversal(doc)
    session = traversal.create_session(SessionConfig())

    # Search semantically
    results = asyncio.run(traversal.search(
        session,
        "How does authentication work?",
        SearchOptions().with_limit(5)
    ))

    print(f"Found {len(results.matches)} relevant blocks")

    # Add top results to context
    for match in results.matches[:3]:
        traversal.context_add(
            session,
            match.block_id,
            relevance=match.similarity
        )

    # Add all results
    traversal.context_add_results(session)

    traversal.close_session(session)
    ```

=== "JavaScript"
    ```javascript
    // Set up mock RAG provider
    const rag = new MockRagProvider();

    const traversal = new AgentTraversal(doc);
    const session = traversal.createSession(new SessionConfig());

    // Search semantically (async)
    const results = await traversal.search(
        session,
        "How does authentication work?",
        new SearchOptions().withLimit(5)
    );

    console.log(`Found ${results.matches.length} relevant blocks`);

    // Add top results to context
    for (const match of results.matches.slice(0, 3)) {
        traversal.contextAdd(session, match.blockId, null, match.similarity);
    }

    // Add all results
    traversal.contextAddResults(session);

    traversal.closeSession(session);
    ```

## Path Finding

Find the connection path between two blocks:

=== "Rust"
    ```rust
    // Find path between two blocks
    let path = traversal.find_path(
        &session,
        source_block,
        target_block,
        Some(10),  // max 10 blocks
    )?;

    println!("Path: {} hops", path.len());

    // Add all blocks in path to context
    for block_id in &path {
        traversal.context_add(&session, *block_id, None, Some(0.8))?;
    }
    ```

=== "Python"
    ```python
    # Find path
    path = traversal.find_path(session, source_block, target_block, max_length=10)

    print(f"Path: {len(path)} hops")

    # Add all to context
    for block_id in path:
        traversal.context_add(session, block_id, relevance=0.8)
    ```

=== "JavaScript"
    ```javascript
    // Find path
    const path = traversal.findPath(session, sourceBlock, targetBlock, 10);

    console.log(`Path: ${path.length} hops`);

    // Add all to context
    for (const blockId of path) {
        traversal.contextAdd(session, blockId, null, 0.8);
    }
    ```

## Multiple Sessions (Parallel Agents)

Run multiple agents in parallel:

=== "Rust"
    ```rust
    // Create sessions for multiple agents
    let agent1_session = traversal.create_session(
        SessionConfig::new().with_name("analyzer")
    )?;
    let agent2_session = traversal.create_session(
        SessionConfig::new().with_name("summarizer")
    )?;

    // Agent 1: Analyze structure
    let expansion = traversal.expand(
        &agent1_session,
        doc.root,
        ExpandDirection::Down,
        ExpandOptions::new().with_depth(3),
    )?;

    // Agent 2: Find key terms
    let findings = traversal.find_by_pattern(
        &agent2_session,
        None,
        Some("important"),
        None,
        None,
    )?;

    // Each agent maintains independent state
    traversal.context_add(&agent1_session, doc.root, None, Some(0.9))?;
    traversal.context_add(&agent2_session, findings.matches[0], None, Some(0.9))?;

    // Close sessions
    traversal.close_session(&agent1_session)?;
    traversal.close_session(&agent2_session)?;
    ```

=== "Python"
    ```python
    # Create sessions for multiple agents
    analyzer = traversal.create_session(SessionConfig().with_name("analyzer"))
    summarizer = traversal.create_session(SessionConfig().with_name("summarizer"))

    # Agent 1: Analyze
    expansion = traversal.expand(analyzer, doc.root, "down", depth=3)

    # Agent 2: Summarize
    findings = traversal.find(summarizer, tag="important")

    # Add to contexts independently
    traversal.context_add(analyzer, doc.root, relevance=0.9)
    traversal.context_add(summarizer, findings.matches[0], relevance=0.9)

    # Clean up
    traversal.close_session(analyzer)
    traversal.close_session(summarizer)
    ```

=== "JavaScript"
    ```javascript
    // Create sessions for multiple agents
    const analyzer = traversal.createSession(new SessionConfig().withName("analyzer"));
    const summarizer = traversal.createSession(new SessionConfig().withName("summarizer"));

    // Agent 1
    const expansion = traversal.expand(
        analyzer,
        doc.root,
        ExpandDirection.Down,
        null,
        new ExpandOptions().withDepth(3)
    );

    // Agent 2
    const findings = traversal.findByPattern(summarizer, null, "important");

    // Independent contexts
    traversal.contextAdd(analyzer, doc.root, null, 0.9);
    traversal.contextAdd(summarizer, findings.matches[0], null, 0.9);

    // Clean up
    traversal.closeSession(analyzer);
    traversal.closeSession(summarizer);
    ```

## UCL Command Execution

Execute UCL commands directly:

=== "Rust"
    ```rust
    let ucl = format!(
        "GOTO {}\nEXPAND {} DOWN DEPTH=2\nFIND ROLE=paragraph\nCTX CLEAR",
        doc.root, doc.root
    );

    let results = ucp_agent::execute_ucl(&traversal, &session, &ucl).await?;

    for (i, result) in results.iter().enumerate() {
        println!("Command {}: {:?}", i + 1, result);
    }
    ```

=== "Python"
    ```python
    import asyncio

    ucl = f"""
    GOTO {doc.root}
    EXPAND {doc.root} DOWN DEPTH=2
    FIND ROLE=paragraph
    CTX CLEAR
    """

    results = asyncio.run(traversal.execute_ucl(session, ucl))

    for i, result in enumerate(results, 1):
        print(f"Command {i}: {result}")
    ```

=== "JavaScript"
    ```javascript
    const ucl = `
    GOTO ${doc.root}
    EXPAND ${doc.root} DOWN DEPTH=2
    FIND ROLE=paragraph
    CTX CLEAR
    `;

    const results = await traversal.executeUcl(session, ucl);

    results.forEach((result, i) => {
        console.log(`Command ${i + 1}: ${result}`);
    });
    ```
