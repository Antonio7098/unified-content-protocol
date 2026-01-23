# Intermediate Examples

This document provides intermediate examples demonstrating more complex UCP usage patterns.

## Example 1: Document Transformation with UCL

=== "Rust"
    ```rust
    use ucp_api::UcpClient;
    use ucm_core::Document;

    fn main() -> Result<(), Box<dyn std::error::Error>> {
        let client = UcpClient::new();
        let mut doc = client.create_document();
        
        // Build initial structure
        client.execute_ucl(&mut doc, r#"
            APPEND blk_ff00000000000000000000 text WITH role="heading1" label="title" :: "Technical Guide"
            APPEND blk_ff00000000000000000000 text WITH role="heading2" label="intro" :: "Introduction"
            APPEND blk_ff00000000000000000000 text WITH role="heading2" label="setup" :: "Setup"
            APPEND blk_ff00000000000000000000 text WITH role="heading2" label="usage" :: "Usage"
        "#)?;
        
        // Add content under sections
        if let Some(intro_id) = doc.indices.find_by_label("intro") {
            client.execute_ucl(&mut doc, &format!(r#"
                APPEND {} text WITH role="paragraph" :: "Welcome to this technical guide."
                APPEND {} text WITH role="paragraph" :: "This guide covers installation and usage."
            "#, intro_id, intro_id))?;
        }
        
        if let Some(setup_id) = doc.indices.find_by_label("setup") {
            client.execute_ucl(&mut doc, &format!(r#"
                APPEND {} text WITH role="paragraph" :: "Follow these steps to set up:"
                APPEND {} code WITH lang="bash" :: "cargo add my-crate"
            "#, setup_id, setup_id))?;
        }
        
        // Transform: Add tags to all paragraphs
        for block_id in doc.indices.find_by_type("text").clone() {
            if let Some(block) = doc.get_block(&block_id) {
                if block.metadata.semantic_role
                    .as_ref()
                    .map(|r| r.category.as_str() == "paragraph")
                    .unwrap_or(false)
                {
                    client.execute_ucl(&mut doc, &format!(
                        r#"EDIT {} SET metadata.tags += ["content"]"#,
                        block_id
                    ))?;
                }
            }
        }
        
        println!("Document has {} blocks", doc.block_count());
        println!("Content-tagged blocks: {}", doc.indices.find_by_tag("content").len());
        
        Ok(())
    }
    ```

=== "Python"
    ```python
    from ucp_content import Document, execute_ucl

    doc = Document.create()
    root = doc.root_id

    # Build initial structure
    execute_ucl(doc, f"""
        APPEND {root} text WITH role="heading1" label="title" :: "Technical Guide"
        APPEND {root} text WITH role="heading2" label="intro" :: "Introduction"
        APPEND {root} text WITH role="heading2" label="setup" :: "Setup"
        APPEND {root} text WITH role="heading2" label="usage" :: "Usage"
    """)

    # Add content under sections
    intro_id = doc.find_by_label("intro")
    if intro_id:
        execute_ucl(doc, f"""
            APPEND {intro_id} text WITH role="paragraph" :: "Welcome to this technical guide."
            APPEND {intro_id} text WITH role="paragraph" :: "This guide covers installation and usage."
        """)

    setup_id = doc.find_by_label("setup")
    if setup_id:
        execute_ucl(doc, f"""
            APPEND {setup_id} text WITH role="paragraph" :: "Follow these steps to set up:"
            APPEND {setup_id} code WITH lang="bash" :: "pip install my-package"
        """)

    # Transform: Add tags to all paragraphs
    # Note: find_by_type and find_by_label return lists of IDs
    for block_id in doc.find_by_type("text"):
        block = doc.get_block(block_id)
        if block.role == "paragraph":
            execute_ucl(doc, f'EDIT {block_id} SET metadata.tags += ["content"]')

    print(f"Document has {doc.block_count()} blocks")
    print(f"Content-tagged blocks: {len(doc.find_by_tag('content'))}")
    ```

=== "JavaScript"
    ```javascript
    import { Document, executeUcl } from 'ucp-content';

    const doc = Document.create();
    const root = doc.rootId;

    // Build initial structure
    executeUcl(doc, `
        APPEND ${root} text WITH role="heading1" label="title" :: "Technical Guide"
        APPEND ${root} text WITH role="heading2" label="intro" :: "Introduction"
        APPEND ${root} text WITH role="heading2" label="setup" :: "Setup"
        APPEND ${root} text WITH role="heading2" label="usage" :: "Usage"
    `);

    // Add content under sections
    const introId = doc.findByLabel("intro");
    if (introId) {
        executeUcl(doc, `
            APPEND ${introId} text WITH role="paragraph" :: "Welcome to this technical guide."
            APPEND ${introId} text WITH role="paragraph" :: "This guide covers installation and usage."
        `);
    }

    const setupId = doc.findByLabel("setup");
    if (setupId) {
        executeUcl(doc, `
            APPEND ${setupId} text WITH role="paragraph" :: "Follow these steps to set up:"
            APPEND ${setupId} code WITH lang="bash" :: "npm install my-package"
        `);
    }

    // Transform: Add tags to all paragraphs
    // Note: findByType returns array of IDs
    const textBlocks = doc.findByType("text");
    for (const blockId of textBlocks) {
        const block = doc.getBlock(blockId);
        // Assuming role is exposed as a string property
        if (block.role === "paragraph") {
            executeUcl(doc, `EDIT ${blockId} SET metadata.tags += ["content"]`);
        }
    }

    console.log(`Document has ${doc.blockCount()} blocks`);
    console.log(`Content-tagged blocks: ${doc.findByTag("content").length}`);
    ```

## Example 2: Working with Edges and Relationships

=== "Rust"
    ```rust
    use ucm_core::{Block, Content, Document, Edge, EdgeType};

    fn main() -> Result<(), Box<dyn std::error::Error>> {
        let mut doc = Document::create();
        let root = doc.root.clone();
        
        // Create a claim
        let claim = Block::new(
            Content::text("Rust provides memory safety without garbage collection."),
            Some("body.argument")
        ).with_label("main-claim");
        let claim_id = doc.add_block(claim, &root)?;
        
        // Create supporting evidence
        let evidence1 = Block::new(
            Content::text("The borrow checker enforces ownership rules at compile time."),
            Some("body.evidence")
        ).with_label("evidence-1");
        let evidence1_id = doc.add_block(evidence1, &root)?;
        
        let evidence2 = Block::new(
            Content::text("Lifetimes ensure references are always valid."),
            Some("body.evidence")
        ).with_label("evidence-2");
        let evidence2_id = doc.add_block(evidence2, &root)?;
        
        // Create counterargument
        let counter = Block::new(
            Content::text("Unsafe blocks can bypass safety guarantees."),
            Some("body.counterargument")
        ).with_label("counter-1");
        let counter_id = doc.add_block(counter, &root)?;
        
        // Create rebuttal
        let rebuttal = Block::new(
            Content::text("Unsafe is explicit and auditable, unlike implicit unsafety."),
            Some("body.rebuttal")
        ).with_label("rebuttal-1");
        let rebuttal_id = doc.add_block(rebuttal, &root)?;
        
        // Add relationships
        // Evidence supports claim
        let edge1 = Edge::new(EdgeType::Supports, claim_id.clone())
            .with_confidence(0.95);
        doc.get_block_mut(&evidence1_id).unwrap().add_edge(edge1.clone());
        doc.edge_index.add_edge(&evidence1_id, &edge1);
        
        let edge2 = Edge::new(EdgeType::Supports, claim_id.clone())
            .with_confidence(0.90);
        doc.get_block_mut(&evidence2_id).unwrap().add_edge(edge2.clone());
        doc.edge_index.add_edge(&evidence2_id, &edge2);
        
        // Counter contradicts claim
        let edge3 = Edge::new(EdgeType::Contradicts, claim_id.clone())
            .with_confidence(0.7);
        doc.get_block_mut(&counter_id).unwrap().add_edge(edge3.clone());
        doc.edge_index.add_edge(&counter_id, &edge3);
        
        // Rebuttal contradicts counter
        let edge4 = Edge::new(EdgeType::Contradicts, counter_id.clone())
            .with_confidence(0.85);
        doc.get_block_mut(&rebuttal_id).unwrap().add_edge(edge4.clone());
        doc.edge_index.add_edge(&rebuttal_id, &edge4);
        
        // Query relationships
        println!("Blocks supporting the claim:");
        for source_id in doc.edge_index.incoming_of_type(&claim_id, &EdgeType::Supports) {
            if let Some(block) = doc.get_block(&source_id) {
                if let Content::Text(text) = &block.content {
                    println!("  - {}", &text.text[..50.min(text.text.len())]);
                }
            }
        }
        
        println!("\nBlocks contradicting the claim:");
        for source_id in doc.edge_index.incoming_of_type(&claim_id, &EdgeType::Contradicts) {
            if let Some(block) = doc.get_block(&source_id) {
                if let Content::Text(text) = &block.content {
                    println!("  - {}", &text.text[..50.min(text.text.len())]);
                }
            }
        }
        
        Ok(())
    }
    ```

=== "Python"
    ```python
    from ucp_content import Document, EdgeType

    doc = Document.create()
    root = doc.root_id

    # Create content
    claim_id = doc.add_block(root, "Rust provides memory safety without garbage collection.", role="body.argument", label="main-claim")
    evidence1_id = doc.add_block(root, "The borrow checker enforces ownership rules at compile time.", role="body.evidence", label="evidence-1")
    evidence2_id = doc.add_block(root, "Lifetimes ensure references are always valid.", role="body.evidence", label="evidence-2")
    counter_id = doc.add_block(root, "Unsafe blocks can bypass safety guarantees.", role="body.counterargument", label="counter-1")
    rebuttal_id = doc.add_block(root, "Unsafe is explicit and auditable, unlike implicit unsafety.", role="body.rebuttal", label="rebuttal-1")

    # Add relationships
    doc.add_edge(evidence1_id, EdgeType.Supports, claim_id)
    doc.add_edge(evidence2_id, EdgeType.Supports, claim_id)
    doc.add_edge(counter_id, EdgeType.Contradicts, claim_id)
    doc.add_edge(rebuttal_id, EdgeType.Contradicts, counter_id)

    # Query relationships
    print("Blocks supporting the claim:")
    supporters = doc.incoming_edges(claim_id)
    for edge_type, source_id in supporters:
        if edge_type == EdgeType.Supports:
            block = doc.get_block(source_id)
            print(f"  - {block.content.as_text()}")

    print("\nBlocks contradicting the claim:")
    contradictors = doc.incoming_edges(claim_id)
    for edge_type, source_id in contradictors:
        if edge_type == EdgeType.Contradicts:
            block = doc.get_block(source_id)
            print(f"  - {block.content.as_text()}")
    ```

=== "JavaScript"
    ```javascript
    import { Document, EdgeType } from 'ucp-content';

    const doc = Document.create();
    const root = doc.rootId;

    // Create content
    const claimId = doc.addBlock(root, "Rust provides memory safety without garbage collection.", "body.argument", "main-claim");
    const evidence1Id = doc.addBlock(root, "The borrow checker enforces ownership rules at compile time.", "body.evidence", "evidence-1");
    const evidence2Id = doc.addBlock(root, "Lifetimes ensure references are always valid.", "body.evidence", "evidence-2");
    const counterId = doc.addBlock(root, "Unsafe blocks can bypass safety guarantees.", "body.counterargument", "counter-1");
    const rebuttalId = doc.addBlock(root, "Unsafe is explicit and auditable, unlike implicit unsafety.", "body.rebuttal", "rebuttal-1");

    // Add relationships
    doc.addEdge(evidence1Id, EdgeType.Supports, claimId);
    doc.addEdge(evidence2Id, EdgeType.Supports, claimId);
    doc.addEdge(counterId, EdgeType.Contradicts, claimId);
    doc.addEdge(rebuttalId, EdgeType.Contradicts, counterId);

    // Query relationships
    console.log("Blocks supporting the claim:");
    const incoming = doc.incomingEdges(claimId);
    for (const edge of incoming) {
        if (edge.edgeType === "Supports") {
            const block = doc.getBlock(edge.source);
            console.log(`  - ${block.content.text}`);
        }
    }

    console.log("\nBlocks contradicting the claim:");
    for (const edge of incoming) {
        if (edge.edgeType === "Contradicts") {
            const block = doc.getBlock(edge.source);
            console.log(`  - ${block.content.text}`);
        }
    }
    ```

## Example 3: Transactions and Snapshots

=== "Rust"
    ```rust
    use ucm_engine::{Engine, Operation};
    use ucm_core::{Content, Document};

    fn main() -> Result<(), Box<dyn std::error::Error>> {
        let mut engine = Engine::new();
        let mut doc = Document::create();
        let root = doc.root.clone();
        
        // Create initial snapshot
        engine.create_snapshot("initial", &doc, Some("Empty document".into()))?;
        
        // Transaction 1: Add structure
        let txn1 = engine.begin_named_transaction("add-chapters");
        
        engine.add_to_transaction(&txn1, Operation::Append {
            parent_id: root.clone(),
            content: Content::text("Chapter 1"),
            label: Some("ch1".into()),
            tags: vec!["chapter".into()],
            semantic_role: Some("heading1".into()),
            index: None,
        })?;
        
        engine.add_to_transaction(&txn1, Operation::Append {
            parent_id: root.clone(),
            content: Content::text("Chapter 2"),
            label: Some("ch2".into()),
            tags: vec!["chapter".into()],
            semantic_role: Some("heading1".into()),
            index: None,
        })?;
        
        let results = engine.commit_transaction(&txn1, &mut doc)?;
        println!("Transaction 1: Added {} blocks", results.len());
        
        // Snapshot after chapters
        engine.create_snapshot("with-chapters", &doc, Some("Added chapters".into()))?;
        
        // Transaction 2: Add content (will be rolled back)
        let txn2 = engine.begin_named_transaction("add-content");
        
        let ch1_id = doc.indices.find_by_label("ch1").unwrap();
        engine.add_to_transaction(&txn2, Operation::Append {
            parent_id: ch1_id,
            content: Content::text("Wrong content - will be rolled back"),
            label: None,
            tags: vec![],
            semantic_role: Some("paragraph".into()),
            index: None,
        })?;
        
        // Oops, wrong content - rollback
        engine.rollback_transaction(&txn2)?;
        println!("Transaction 2: Rolled back");
        
        // Transaction 3: Add correct content
        let txn3 = engine.begin_named_transaction("add-correct-content");
        
        engine.add_to_transaction(&txn3, Operation::Append {
            parent_id: ch1_id,
            content: Content::text("This is the correct introduction to Chapter 1."),
            label: None,
            tags: vec![],
            semantic_role: Some("paragraph".into()),
            index: None,
        })?;
        
        engine.commit_transaction(&txn3, &mut doc)?;
        println!("Transaction 3: Committed");
        
        // Final snapshot
        engine.create_snapshot("final", &doc, Some("Complete document".into()))?;
        
        // List snapshots
        println!("\nSnapshots:");
        for name in engine.list_snapshots() {
            println!("  - {}", name);
        }
        
        // Demonstrate restore
        println!("\nCurrent block count: {}", doc.block_count());
        
        let restored = engine.restore_snapshot("with-chapters")?;
        println!("After restore to 'with-chapters': {} blocks", restored.block_count());
        
        Ok(())
    }
    ```

=== "Python"
    ```python
    from ucp_content import Document, SnapshotManager, execute_ucl

    doc = Document.create()
    root = doc.root_id
    snapshots = SnapshotManager()

    # Create initial snapshot
    snapshots.create("initial", doc, "Empty document")

    # Transaction 1: Add structure (using atomic UCL command block if available or separate calls)
    # Python/JS bindings typically use the high-level API which auto-commits unless using a specific transaction API.
    # Here we demonstrate snapshot/restore for state management.
    
    doc.add_block(root, "Chapter 1", label="ch1", role="heading1")
    doc.add_block(root, "Chapter 2", label="ch2", role="heading1")
    print("Added structure")

    # Snapshot after chapters
    snapshots.create("with-chapters", doc, "Added chapters")

    # Simulating a "transaction" via snapshot rollback
    ch1_id = doc.find_by_label("ch1")
    doc.add_block(ch1_id, "Wrong content - will be rolled back")
    print("Added wrong content")

    # Rollback
    doc = snapshots.restore("with-chapters")
    print("Rolled back to 'with-chapters'")

    # Add correct content
    ch1_id = doc.find_by_label("ch1") # Need to find again in restored doc
    doc.add_block(ch1_id, "This is the correct introduction.")
    print("Added correct content")

    # Final snapshot
    snapshots.create("final", doc, "Complete document")

    # List snapshots
    print("\nSnapshots:")
    for snap in snapshots.list():
        print(f"  - {snap.name}")
    ```

=== "JavaScript"
    ```javascript
    import { Document, SnapshotManager } from 'ucp-content';

    let doc = Document.create();
    const root = doc.rootId;
    const snapshots = new SnapshotManager();

    // Create initial snapshot
    snapshots.create("initial", doc, "Empty document");

    // Add structure
    doc.addBlock(root, "Chapter 1", "heading1", "ch1");
    doc.addBlock(root, "Chapter 2", "heading1", "ch2");
    console.log("Added structure");

    // Snapshot after chapters
    snapshots.create("with-chapters", doc, "Added chapters");

    // Simulating a "transaction" via snapshot rollback
    let ch1Id = doc.findByLabel("ch1");
    doc.addBlock(ch1Id, "Wrong content - will be rolled back");
    console.log("Added wrong content");

    // Rollback
    doc = snapshots.restore("with-chapters");
    console.log("Rolled back to 'with-chapters'");

    // Add correct content
    ch1Id = doc.findByLabel("ch1"); // Need to find again in restored doc
    doc.addBlock(ch1Id, "This is the correct introduction.");
    console.log("Added correct content");

    // Final snapshot
    snapshots.create("final", doc, "Complete document");

    // List snapshots
    console.log("\nSnapshots:");
    for (const snap of snapshots.list()) {
        console.log(`  - ${snap.name}`);
    }
    ```

## Example 4: Custom Validation Rules

=== "Rust"
    ```rust
    use ucm_core::{Document, Block, Content, ValidationIssue, ValidationSeverity, ErrorCode};
    use ucm_engine::validate::{ValidationPipeline, ValidationResult};

    fn validate_document_rules(doc: &Document) -> ValidationResult {
        let mut issues = Vec::new();
        
        // Rule 1: Document must have a title
        let has_title = doc.blocks.values().any(|b| {
            b.metadata.semantic_role
                .as_ref()
                .map(|r| r.category.as_str() == "title")
                .unwrap_or(false)
        });
        
        if !has_title {
            issues.push(ValidationIssue::warning(
                ErrorCode::E202InvalidStructure,
                "Document should have a title block".to_string(),
            ));
        }
        
        // Rule 2: All code blocks should have a language
        for block in doc.blocks.values() {
            if let Content::Code(code) = &block.content {
                if code.language.is_empty() || code.language == "text" {
                    issues.push(ValidationIssue::warning(
                        ErrorCode::E200SchemaViolation,
                        format!("Code block {} has no language specified", block.id),
                    ));
                }
            }
        }
        
        ValidationResult::invalid(issues)
    }

    fn main() -> Result<(), Box<dyn std::error::Error>> {
        let mut doc = Document::create();
        let root = doc.root.clone();
        
        // Add blocks (intentionally missing title)
        let heading = Block::new(Content::text(""), Some("heading1")); // Empty heading
        doc.add_block(heading, &root)?;
        
        let code = Block::new(Content::code("", "print('hello')"), Some("code")); // No language
        doc.add_block(code, &root)?;
        
        // Standard validation
        let standard_validator = ValidationPipeline::new();
        let mut result = standard_validator.validate_document(&doc);
        
        // Custom validation
        result.merge(validate_document_rules(&doc));
        
        println!("Validation Results:");
        println!("  Valid: {}", result.valid);
        
        for issue in &result.issues {
            println!("  [{:?}] {}", issue.severity, issue.message);
        }
        
        Ok(())
    }
    ```

=== "Python"
    ```python
    from ucp_content import Document

    def validate_custom_rules(doc):
        issues = []
        
        # Rule: Document must have a title
        has_title = any(
            block.role == "title" 
            for block in doc.blocks()
        )
        if not has_title:
            issues.append(("Warning", "Document should have a title block"))
            
        return issues

    doc = Document.create()
    doc.add_block(doc.root_id, "Content", role="paragraph")

    # Standard validation
    issues = doc.validate()
    
    # Custom validation
    custom_issues = validate_custom_rules(doc)
    
    print("Validation Results:")
    for severity, code, msg in issues:
        print(f"  [{severity}] {msg}")
    for severity, msg in custom_issues:
        print(f"  [{severity}] {msg}")
    ```

=== "JavaScript"
    ```javascript
    import { Document } from 'ucp-content';

    function validateCustomRules(doc) {
        const issues = [];
        
        // Rule: Document must have a title
        const blocks = doc.blocks();
        const hasTitle = blocks.some(b => b.role === "title");
        
        if (!hasTitle) {
            issues.push({ severity: "Warning", message: "Document should have a title block" });
        }
        
        return issues;
    }

    const doc = Document.create();
    doc.addBlock(doc.rootId, "Content", "paragraph");

    // Standard validation
    const issues = doc.validate() || [];
    
    // Custom validation
    const customIssues = validateCustomRules(doc);
    
    console.log("Validation Results:");
    for (const issue of [...issues, ...customIssues]) {
        console.log(`  [${issue.severity}] ${issue.message}`);
    }
    ```

## Example 5: Document Merging

=== "Rust"
    ```rust
    // See Rust example above for complex merging logic
    ```

=== "Python"
    ```python
    # Simplified merging example
    main_doc = Document.create(title="Main")
    merge_doc = Document.create(title="Merge")
    
    # Add content to merge
    merge_root = merge_doc.root_id
    merge_doc.add_block(merge_root, "Content to merge", role="paragraph")
    
    # Merge logic (simplified: copy content)
    for block in merge_doc.blocks():
        if block.id != merge_root:
            main_doc.add_block(
                main_doc.root_id,
                block.content.as_text(),
                role=block.role
            )
            
    print(f"Main doc has {main_doc.block_count()} blocks")
    ```

=== "JavaScript"
    ```javascript
    // Simplified merging example
    const mainDoc = Document.create("Main");
    const mergeDoc = Document.create("Merge");
    
    // Add content to merge
    mergeDoc.addBlock(mergeDoc.rootId, "Content to merge", "paragraph");
    
    // Merge logic (simplified: copy content)
    const blocks = mergeDoc.blocks();
    for (const block of blocks) {
        if (block.id !== mergeDoc.rootId) {
            mainDoc.addBlock(
                mainDoc.rootId,
                block.content.text,
                block.role
            );
        }
    }
    
    console.log(`Main doc has ${mainDoc.blockCount()} blocks`);
    ```

## Example 6: Token-Aware Document Processing

=== "Rust"
    ```rust
    use ucm_core::{Block, Content, Document};
    use ucm_core::metadata::{TokenEstimate, TokenModel};

    fn main() -> Result<(), Box<dyn std::error::Error>> {
        let mut doc = Document::create();
        let root = doc.root.clone();
        
        // Add blocks
        let short_id = doc.add_block(
            Block::new(Content::text("Short text."), Some("paragraph")), 
            &root
        )?;
        
        let long_id = doc.add_block(
            Block::new(Content::text(&"Long content. ".repeat(100)), Some("paragraph")),
            &root
        )?;
        
        // Estimate tokens
        println!("Token estimates (GPT-4):");
        for (name, id) in [("Short", &short_id), ("Long", &long_id)] {
            let block = doc.get_block(id).unwrap();
            let tokens = block.token_estimate().for_model(TokenModel::GPT4);
            println!("  {}: {} tokens", name, tokens);
        }
        
        let total = doc.total_tokens(TokenModel::GPT4);
        println!("\nTotal document: {} tokens", total);
        
        Ok(())
    }
    ```

=== "Python"
    ```python
    from ucp_content import Document, IdMapper

    doc = Document.create()
    mapper = IdMapper() # Used for estimation logic
    
    # Add blocks
    short_id = doc.add_block(doc.root_id, "Short text.")
    long_id = doc.add_block(doc.root_id, "Long content. " * 100)
    
    # Get estimates
    short_block = doc.get_block(short_id)
    short_est = mapper.estimate_token_savings(short_block.content.as_text())
    print(f"Short block: ~{short_est[0]} tokens")
    
    long_block = doc.get_block(long_id)
    long_est = mapper.estimate_token_savings(long_block.content.as_text())
    print(f"Long block: ~{long_est[0]} tokens")
    ```

=== "JavaScript"
    ```javascript
    import { Document, IdMapper } from 'ucp-content';

    const doc = Document.create();
    const mapper = new IdMapper();
    
    const shortId = doc.addBlock(doc.rootId, "Short text.");
    const longId = doc.addBlock(doc.rootId, "Long content. ".repeat(100));
    
    const shortBlock = doc.getBlock(shortId);
    const shortEst = mapper.estimateTokenSavings(shortBlock.content.text);
    console.log(`Short block: ~${shortEst.originalTokens} tokens`);
    
    const longBlock = doc.getBlock(longId);
    const longEst = mapper.estimateTokenSavings(longBlock.content.text);
    console.log(`Long block: ~${longEst.originalTokens} tokens`);
    ```

## Example 7: Batch Document Processing

=== "Rust"
    ```rust
    use ucp_api::UcpClient;
    use ucp_translator_markdown::{parse_markdown, render_markdown};
    use std::collections::HashMap;

    fn process_markdown_files(files: &[(&str, &str)]) -> Result<HashMap<String, String>, Box<dyn std::error::Error>> {
        let client = UcpClient::new();
        let mut results = HashMap::new();
        
        for (name, content) in files {
            // Parse markdown
            let mut doc = parse_markdown(content)?;
            
            // Add processing metadata via UCL
            client.execute_ucl(&mut doc, &format!(r#"
                EDIT blk_ff00000000000000000000 SET metadata.processed = true
                EDIT blk_ff00000000000000000000 SET metadata.source = "{}"
            "#, name))?;
            
            // Render back
            let rendered = render_markdown(&doc)?;
            results.insert(name.to_string(), rendered);
        }
        
        Ok(results)
    }
    ```

=== "Python"
    ```python
    from ucp_content import parse, render, execute_ucl

    def process_files(files):
        results = {}
        for name, content in files.items():
            doc = parse(content)
            
            # Add metadata
            execute_ucl(doc, f"""
                EDIT {doc.root_id} SET metadata.processed = true
                EDIT {doc.root_id} SET metadata.source = "{name}"
            """)
            
            results[name] = render(doc)
        return results
    ```

=== "JavaScript"
    ```javascript
    import { parseMarkdown, renderMarkdown, executeUcl } from 'ucp-content';

    function processFiles(files) {
        const results = {};
        for (const [name, content] of Object.entries(files)) {
            const doc = parseMarkdown(content);
            
            // Add metadata
            executeUcl(doc, `
                EDIT ${doc.rootId} SET metadata.processed = true
                EDIT ${doc.rootId} SET metadata.source = "${name}"
            `);
            
            results[name] = renderMarkdown(doc);
        }
        return results;
    }
    ```

## Next Steps

- [Advanced Examples](./advanced.md) - Complex patterns and integrations
- [UCL Commands](../ucl-parser/commands.md) - Complete UCL reference
- [UCM Engine](../ucm-engine/README.md) - Engine documentation

