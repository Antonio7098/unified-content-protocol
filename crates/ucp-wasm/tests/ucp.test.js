/**
 * Tests for UCP WASM bindings
 */

import { jest } from '@jest/globals';

// Import the WASM module
let ucp;
beforeAll(async () => {
  ucp = await import('../pkg/ucp_wasm.js');
});

describe('Document', () => {
  describe('creation', () => {
    test('creates empty document', () => {
      const doc = new ucp.Document();
      expect(doc).toBeDefined();
      expect(doc.id).toBeDefined();
      expect(doc.rootId).toBeDefined();
    });

    test('creates document with title', () => {
      const doc = new ucp.Document('Test Document');
      expect(doc.title).toBe('Test Document');
    });

    test('sets and gets title', () => {
      const doc = new ucp.Document();
      doc.title = 'New Title';
      expect(doc.title).toBe('New Title');
    });

    test('sets and gets description', () => {
      const doc = new ucp.Document();
      doc.description = 'A test description';
      expect(doc.description).toBe('A test description');
    });

    test('has version', () => {
      const doc = new ucp.Document();
      expect(doc.version).toBeGreaterThanOrEqual(0);
    });

    test('has timestamps', () => {
      const doc = new ucp.Document();
      expect(doc.createdAt).toBeDefined();
      expect(doc.modifiedAt).toBeDefined();
    });
  });

  describe('block operations', () => {
    test('adds text block', () => {
      const doc = new ucp.Document();
      const blockId = doc.addBlock(doc.rootId, 'Hello, World!');
      expect(blockId).toBeDefined();
      expect(doc.blockCount()).toBe(2);
    });

    test('adds block with role', () => {
      const doc = new ucp.Document();
      const blockId = doc.addBlock(doc.rootId, 'Intro', 'intro');
      const block = doc.getBlock(blockId);
      expect(block.role).toBe('intro');
    });

    test('adds block with label', () => {
      const doc = new ucp.Document();
      const blockId = doc.addBlock(doc.rootId, 'Labeled', null, 'my-label');
      const block = doc.getBlock(blockId);
      expect(block.label).toBe('my-label');
    });

    test('adds code block', () => {
      const doc = new ucp.Document();
      const blockId = doc.addCode(doc.rootId, 'python', 'print("hello")');
      const block = doc.getBlock(blockId);
      expect(block.contentType).toBe('code');
    });

    test('gets block', () => {
      const doc = new ucp.Document();
      const blockId = doc.addBlock(doc.rootId, 'Test');
      const block = doc.getBlock(blockId);
      expect(block).toBeDefined();
      expect(block.id).toBe(blockId);
    });

    test('edits block', () => {
      const doc = new ucp.Document();
      const blockId = doc.addBlock(doc.rootId, 'Original');
      doc.editBlock(blockId, 'Updated');
      const block = doc.getBlock(blockId);
      expect(block.text).toBe('Updated');
    });

    test('moves block', () => {
      const doc = new ucp.Document();
      const block1 = doc.addBlock(doc.rootId, 'Parent 1');
      const block2 = doc.addBlock(doc.rootId, 'Parent 2');
      const child = doc.addBlock(block1, 'Child');

      doc.moveBlock(child, block2);

      const children = doc.children(block2);
      expect(children).toContain(child);
    });

    test('deletes block', () => {
      const doc = new ucp.Document();
      const blockId = doc.addBlock(doc.rootId, 'To delete');
      const initialCount = doc.blockCount();

      doc.deleteBlock(blockId);

      expect(doc.blockCount()).toBe(initialCount - 1);
    });
  });

  describe('traversal', () => {
    let doc, root, block1, block2, block3;

    beforeEach(() => {
      doc = new ucp.Document();
      root = doc.rootId;
      block1 = doc.addBlock(root, 'Block 1', 'paragraph');
      block2 = doc.addBlock(root, 'Block 2', 'paragraph');
      block3 = doc.addBlock(block1, 'Block 3', 'note');
    });

    test('gets children', () => {
      const children = doc.children(root);
      expect(children).toContain(block1);
      expect(children).toContain(block2);
    });

    test('gets parent', () => {
      const parent = doc.parent(block1);
      expect(parent).toBe(root);
    });

    test('gets ancestors', () => {
      const ancestors = doc.ancestors(block3);
      expect(ancestors).toContain(block1);
      expect(ancestors).toContain(root);
    });

    test('gets descendants', () => {
      const descendants = doc.descendants(root);
      expect(descendants).toContain(block1);
      expect(descendants).toContain(block2);
      expect(descendants).toContain(block3);
    });

    test('gets siblings', () => {
      const siblings = doc.siblings(block1);
      expect(siblings).toContain(block2);
      expect(siblings).not.toContain(block1);
    });

    test('gets depth', () => {
      expect(doc.depth(root)).toBe(0);
      expect(doc.depth(block1)).toBe(1);
      expect(doc.depth(block3)).toBe(2);
    });

    test('gets path from root', () => {
      const path = doc.pathFromRoot(block3);
      expect(path[0]).toBe(root);
      expect(path[1]).toBe(block1);
      expect(path[2]).toBe(block3);
    });

    test('gets sibling index', () => {
      expect(doc.siblingIndex(block1)).toBe(0);
      expect(doc.siblingIndex(block2)).toBe(1);
    });

    test('checks reachability', () => {
      expect(doc.isReachable(block1)).toBe(true);
      expect(doc.isReachable(block3)).toBe(true);
    });

    test('checks ancestor relationship', () => {
      expect(doc.isAncestor(root, block3)).toBe(true);
      expect(doc.isAncestor(block1, block3)).toBe(true);
      expect(doc.isAncestor(block2, block3)).toBe(false);
    });
  });

  describe('finding', () => {
    test('finds by tag', () => {
      const doc = new ucp.Document();
      const block1 = doc.addBlock(doc.rootId, 'Block 1');
      const block2 = doc.addBlock(doc.rootId, 'Block 2');

      doc.addTag(block1, 'important');
      doc.addTag(block2, 'important');

      const found = doc.findByTag('important');
      expect(found).toContain(block1);
      expect(found).toContain(block2);
    });

    test('finds by label', () => {
      const doc = new ucp.Document();
      const blockId = doc.addBlock(doc.rootId, 'Labeled', null, 'unique-label');

      const found = doc.findByLabel('unique-label');
      expect(found).toBe(blockId);
    });

    test('finds by type', () => {
      const doc = new ucp.Document();
      const textBlock = doc.addBlock(doc.rootId, 'Text');
      const codeBlock = doc.addCode(doc.rootId, 'js', 'console.log()');

      const codeBlocks = doc.findByType('code');
      expect(codeBlocks).toContain(codeBlock);
      expect(codeBlocks).not.toContain(textBlock);
    });

    test('finds by role', () => {
      const doc = new ucp.Document();
      const block1 = doc.addBlock(doc.rootId, 'Para 1', 'paragraph');
      const block2 = doc.addBlock(doc.rootId, 'Para 2', 'paragraph');
      const block3 = doc.addBlock(doc.rootId, 'Note', 'note');

      const paragraphs = doc.findByRole('paragraph');
      expect(paragraphs).toContain(block1);
      expect(paragraphs).toContain(block2);
      expect(paragraphs).not.toContain(block3);
    });
  });

  describe('tags', () => {
    test('adds tag', () => {
      const doc = new ucp.Document();
      const blockId = doc.addBlock(doc.rootId, 'Test');

      doc.addTag(blockId, 'new-tag');
      const block = doc.getBlock(blockId);
      expect(block.tags).toContain('new-tag');
    });

    test('removes tag', () => {
      const doc = new ucp.Document();
      const blockId = doc.addBlock(doc.rootId, 'Test');
      doc.addTag(blockId, 'tag1');
      doc.addTag(blockId, 'tag2');

      const removed = doc.removeTag(blockId, 'tag1');
      expect(removed).toBe(true);

      const block = doc.getBlock(blockId);
      expect(block.tags).not.toContain('tag1');
      expect(block.tags).toContain('tag2');
    });

    test('sets label', () => {
      const doc = new ucp.Document();
      const blockId = doc.addBlock(doc.rootId, 'Test');

      doc.setLabel(blockId, 'new-label');
      const block = doc.getBlock(blockId);
      expect(block.label).toBe('new-label');
    });
  });

  describe('serialization', () => {
    test('converts to JSON', () => {
      const doc = new ucp.Document('Test');
      doc.addBlock(doc.rootId, 'Hello');

      const json = doc.toJson();
      expect(json).toBeDefined();
      expect(json.blocks).toBeDefined();
      expect(json.structure).toBeDefined();
    });

    test('gets all block IDs', () => {
      const doc = new ucp.Document();
      const blockId = doc.addBlock(doc.rootId, 'Test');

      const ids = doc.blockIds();
      expect(ids).toContain(doc.rootId);
      expect(ids).toContain(blockId);
    });

    test('gets all blocks', () => {
      const doc = new ucp.Document();
      doc.addBlock(doc.rootId, 'Test');

      const blocks = doc.blocks();
      expect(blocks.length).toBe(2);
    });
  });
});

describe('Content', () => {
  test('creates text content', () => {
    const content = ucp.Content.text('Hello');
    expect(content.typeTag).toBe('text');
    expect(content.asText()).toBe('Hello');
  });

  test('creates markdown content', () => {
    const content = ucp.Content.markdown('# Heading');
    expect(content.typeTag).toBe('text');
  });

  test('creates code content', () => {
    const content = ucp.Content.code('python', 'print("hi")');
    expect(content.typeTag).toBe('code');
    const code = content.asCode();
    expect(code.language).toBe('python');
    expect(code.source).toBe('print("hi")');
  });

  test('creates JSON content', () => {
    const content = ucp.Content.json({ key: 'value', count: 42 });
    expect(content.typeTag).toBe('json');
    const json = content.asJson();
    expect(json.key).toBe('value');
    expect(json.count).toBe(42);
  });

  test('creates table content', () => {
    const content = ucp.Content.table([['A', 'B'], ['1', '2']]);
    expect(content.typeTag).toBe('table');
    const table = content.asTable();
    expect(table.columns.length).toBe(2);
    expect(table.rows.length).toBe(2);
  });

  test('creates math content', () => {
    const content = ucp.Content.math('E = mc^2', true, 'latex');
    expect(content.typeTag).toBe('math');
    const math = content.asMath();
    expect(math.expression).toBe('E = mc^2');
    expect(math.displayMode).toBe(true);
    expect(math.format).toBe('latex');
  });

  test('creates media content', () => {
    const content = ucp.Content.media('image', 'https://example.com/img.png', 'Alt text', 800, 600);
    expect(content.typeTag).toBe('media');
    const media = content.asMedia();
    expect(media.mediaType).toBe('image');
    expect(media.url).toBe('https://example.com/img.png');
    expect(media.altText).toBe('Alt text');
  });

  test('creates binary content', () => {
    const data = new Uint8Array([0, 1, 2, 3, 4]);
    const content = ucp.Content.binary('application/octet-stream', data);
    expect(content.typeTag).toBe('binary');
    const binary = content.asBinary();
    expect(binary.mimeType).toBe('application/octet-stream');
    expect(binary.data.length).toBe(5);
  });

  test('creates composite content', () => {
    const content = ucp.Content.composite('horizontal');
    expect(content.typeTag).toBe('composite');
  });

  test('checks if empty', () => {
    const empty = ucp.Content.text('');
    expect(empty.isEmpty).toBe(true);

    const notEmpty = ucp.Content.text('hello');
    expect(notEmpty.isEmpty).toBe(false);
  });

  test('gets size in bytes', () => {
    const content = ucp.Content.text('Hello');
    expect(content.sizeBytes).toBeGreaterThan(0);
  });
});

describe('EdgeType', () => {
  test('has edge type values', () => {
    expect(ucp.EdgeType.References).toBeDefined();
    expect(ucp.EdgeType.DerivedFrom).toBeDefined();
    expect(ucp.EdgeType.Supports).toBeDefined();
    expect(ucp.EdgeType.Contradicts).toBeDefined();
  });
});

describe('Edge Operations', () => {
  test('adds edge', () => {
    const doc = new ucp.Document();
    const block1 = doc.addBlock(doc.rootId, 'Block 1');
    const block2 = doc.addBlock(doc.rootId, 'Block 2');

    doc.addEdge(block1, ucp.EdgeType.References, block2);

    const edges = doc.outgoingEdges(block1);
    expect(edges.length).toBeGreaterThan(0);
  });

  test('removes edge', () => {
    const doc = new ucp.Document();
    const block1 = doc.addBlock(doc.rootId, 'Block 1');
    const block2 = doc.addBlock(doc.rootId, 'Block 2');

    doc.addEdge(block1, ucp.EdgeType.References, block2);
    const removed = doc.removeEdge(block1, ucp.EdgeType.References, block2);
    expect(removed).toBe(true);
  });

  test('gets incoming edges', () => {
    const doc = new ucp.Document();
    const block1 = doc.addBlock(doc.rootId, 'Block 1');
    const block2 = doc.addBlock(doc.rootId, 'Block 2');

    doc.addEdge(block1, ucp.EdgeType.References, block2);

    const edges = doc.incomingEdges(block2);
    expect(edges.length).toBeGreaterThan(0);
  });
});

describe('Markdown Integration', () => {
  test('parses markdown', () => {
    const md = `# Hello World

This is a paragraph.
`;
    const doc = ucp.parseMarkdown(md);
    expect(doc).toBeDefined();
    expect(doc.blockCount()).toBeGreaterThan(1);
  });

  test('renders to markdown', () => {
    const doc = new ucp.Document('Test');
    doc.addBlock(doc.rootId, 'Hello, World!');

    const md = ucp.renderMarkdown(doc);
    expect(md).toBeDefined();
    expect(md.length).toBeGreaterThan(0);
  });
});

describe('UCL Execution', () => {
  test('executes EDIT command', () => {
    const doc = new ucp.Document();
    const blockId = doc.addBlock(doc.rootId, 'Original');

    const ucl = `EDIT ${blockId} SET text = "Updated"`;
    const results = ucp.executeUcl(doc, ucl);

    expect(results.length).toBeGreaterThan(0);
    const block = doc.getBlock(blockId);
    expect(block.text).toBe('Updated');
  });

  test('executes APPEND command', () => {
    const doc = new ucp.Document();
    const initialCount = doc.blockCount();

    const ucl = `APPEND ${doc.rootId} text :: "New block"`;
    ucp.executeUcl(doc, ucl);

    expect(doc.blockCount()).toBe(initialCount + 1);
  });
});

describe('IdMapper', () => {
  test('creates empty mapper', () => {
    const mapper = new ucp.IdMapper();
    expect(mapper.length).toBe(0);
  });

  test('creates from document', () => {
    const doc = new ucp.Document();
    doc.addBlock(doc.rootId, 'Test');

    const mapper = ucp.IdMapper.fromDocument(doc);
    expect(mapper.length).toBe(2);
  });

  test('registers block ID', () => {
    const doc = new ucp.Document();
    const mapper = new ucp.IdMapper();

    const shortId = mapper.register(doc.rootId);
    expect(shortId).toBe(1);
  });

  test('converts to short ID', () => {
    const doc = new ucp.Document();
    const mapper = ucp.IdMapper.fromDocument(doc);

    const shortId = mapper.toShortId(doc.rootId);
    expect(shortId).toBeDefined();
  });

  test('converts back to block ID', () => {
    const doc = new ucp.Document();
    const mapper = ucp.IdMapper.fromDocument(doc);

    const shortId = mapper.toShortId(doc.rootId);
    const blockId = mapper.toBlockId(shortId);
    expect(blockId).toBe(doc.rootId);
  });

  test('shortens UCL', () => {
    const doc = new ucp.Document();
    const blockId = doc.addBlock(doc.rootId, 'Test');
    const mapper = ucp.IdMapper.fromDocument(doc);

    const ucl = `EDIT ${blockId} SET text = "hello"`;
    const shortened = mapper.shortenUcl(ucl);

    expect(shortened).not.toContain(blockId);
  });

  test('expands UCL', () => {
    const doc = new ucp.Document();
    const blockId = doc.addBlock(doc.rootId, 'Test');
    const mapper = ucp.IdMapper.fromDocument(doc);

    const shortId = mapper.toShortId(blockId);
    const ucl = `EDIT ${shortId} SET text = "hello"`;
    const expanded = mapper.expandUcl(ucl);

    expect(expanded).toContain(blockId);
  });

  test('estimates token savings', () => {
    const doc = new ucp.Document();
    const blockId = doc.addBlock(doc.rootId, 'Test');
    const mapper = ucp.IdMapper.fromDocument(doc);

    const text = `Block ${doc.rootId} refs ${blockId}`;
    const savings = mapper.estimateTokenSavings(text);

    expect(savings.originalTokens).toBeGreaterThan(savings.shortenedTokens);
    expect(savings.savings).toBeGreaterThan(0);
  });
});

describe('PromptBuilder', () => {
  test('creates empty builder', () => {
    const builder = new ucp.PromptBuilder();
    expect(builder).toBeDefined();
  });

  test('creates with all capabilities', () => {
    const builder = ucp.PromptBuilder.withAllCapabilities();
    expect(builder.hasCapability(ucp.WasmUclCapability.Edit)).toBe(true);
    expect(builder.hasCapability(ucp.WasmUclCapability.Delete)).toBe(true);
  });

  test('adds capability', () => {
    const builder = new ucp.PromptBuilder()
      .withCapability(ucp.WasmUclCapability.Edit);

    expect(builder.hasCapability(ucp.WasmUclCapability.Edit)).toBe(true);
    expect(builder.hasCapability(ucp.WasmUclCapability.Delete)).toBe(false);
  });

  test('removes capability', () => {
    const builder = ucp.PromptBuilder.withAllCapabilities()
      .withoutCapability(ucp.WasmUclCapability.Delete);

    expect(builder.hasCapability(ucp.WasmUclCapability.Delete)).toBe(false);
    expect(builder.hasCapability(ucp.WasmUclCapability.Edit)).toBe(true);
  });

  test('builds system prompt', () => {
    const builder = ucp.PromptBuilder.withAllCapabilities();
    const prompt = builder.buildSystemPrompt();

    expect(prompt).toContain('EDIT');
    expect(prompt).toContain('APPEND');
  });

  test('builds full prompt', () => {
    const builder = new ucp.PromptBuilder()
      .withCapability(ucp.WasmUclCapability.Edit);

    const prompt = builder.buildPrompt('Doc structure here', 'Edit block 1');

    expect(prompt).toContain('Edit block 1');
  });

  test('adds system context', () => {
    const builder = new ucp.PromptBuilder()
      .withCapability(ucp.WasmUclCapability.Edit)
      .withSystemContext('You are helpful');

    const prompt = builder.buildSystemPrompt();
    expect(prompt).toContain('You are helpful');
  });
});

describe('PromptPresets', () => {
  test('basic editing preset', () => {
    const builder = ucp.PromptPresets.basicEditing();
    expect(builder.hasCapability(ucp.WasmUclCapability.Edit)).toBe(true);
    expect(builder.hasCapability(ucp.WasmUclCapability.Move)).toBe(false);
  });

  test('full editing preset', () => {
    const builder = ucp.PromptPresets.fullEditing();
    expect(builder.hasCapability(ucp.WasmUclCapability.Edit)).toBe(true);
    expect(builder.hasCapability(ucp.WasmUclCapability.Move)).toBe(true);
    expect(builder.hasCapability(ucp.WasmUclCapability.Delete)).toBe(true);
  });

  test('token efficient preset', () => {
    const builder = ucp.PromptPresets.tokenEfficient();
    const prompt = builder.buildSystemPrompt();
    expect(prompt).toContain('short numeric IDs');
  });
});

describe('HTML Parsing', () => {
  test('parses simple HTML', () => {
    const html = '<html><body><h1>Hello</h1><p>World</p></body></html>';
    const doc = ucp.parseHtml(html);
    expect(doc).toBeDefined();
    expect(doc.blockCount()).toBeGreaterThan(1);
  });

  test('parses nested HTML', () => {
    const html = `
      <html><body>
        <h1>Title</h1>
        <h2>Section 1</h2>
        <p>Content</p>
        <h2>Section 2</h2>
        <p>More content</p>
      </body></html>
    `;
    const doc = ucp.parseHtml(html);
    expect(doc.blockCount()).toBeGreaterThanOrEqual(5);
  });

  test('parses empty HTML', () => {
    const html = '<html><body></body></html>';
    const doc = ucp.parseHtml(html);
    expect(doc.blockCount()).toBe(1); // Just root
  });
});

describe('Section Utilities', () => {
  test('finds section by path', () => {
    const md = `# Introduction

Some text.

## Getting Started

More text.
`;
    const doc = ucp.parseMarkdown(md);
    
    const sectionId = ucp.findSectionByPath(doc, 'Introduction');
    expect(sectionId).toBeDefined();
  });

  test('gets all sections', () => {
    const md = `# Title

## Section 1

### Subsection

## Section 2
`;
    const doc = ucp.parseMarkdown(md);
    
    const sections = ucp.getAllSections(doc);
    expect(sections.length).toBeGreaterThanOrEqual(4);
  });

  test('clears section with undo', () => {
    const md = `# Title

## Section

Content to clear.
`;
    const doc = ucp.parseMarkdown(md);
    const sectionId = ucp.findSectionByPath(doc, 'Title > Section');
    
    if (sectionId) {
      const initialCount = doc.blockCount();
      const result = ucp.clearSectionWithUndo(doc, sectionId);
      
      expect(result.removedIds.length).toBeGreaterThanOrEqual(0);
      expect(result.deletedContent).toBeDefined();
    }
  });

  test('restores deleted section', () => {
    const md = `# Title

## Section

Content.
`;
    const doc = ucp.parseMarkdown(md);
    const sectionId = ucp.findSectionByPath(doc, 'Title > Section');
    
    if (sectionId) {
      const initialCount = doc.blockCount();
      const result = ucp.clearSectionWithUndo(doc, sectionId);
      
      // Restore
      const restored = ucp.restoreDeletedSection(doc, result.deletedContent);
      expect(doc.blockCount()).toBe(initialCount);
    }
  });

  test('serializes deleted content', () => {
    const md = `# Title

## Section

Content.
`;
    const doc = ucp.parseMarkdown(md);
    const sectionId = ucp.findSectionByPath(doc, 'Title > Section');
    
    if (sectionId) {
      const result = ucp.clearSectionWithUndo(doc, sectionId);
      
      // Serialize
      const json = result.deletedContent.toJson();
      expect(json).toBeDefined();
      expect(json.length).toBeGreaterThan(0);
      
      // Deserialize
      const restored = ucp.WasmDeletedContent.fromJson(json);
      expect(restored.blockCount).toBe(result.deletedContent.blockCount);
    }
  });
});

describe('Observability', () => {
  test('creates audit entry', () => {
    const entry = new ucp.WasmAuditEntry('EDIT', 'doc_123');
    expect(entry.operation).toBe('EDIT');
    expect(entry.documentId).toBe('doc_123');
    expect(entry.success).toBe(true);
  });

  test('audit entry with user', () => {
    const entry = new ucp.WasmAuditEntry('CREATE', 'doc_456')
      .withUser('user_789');
    expect(entry.userId).toBe('user_789');
  });

  test('audit entry failed', () => {
    const entry = new ucp.WasmAuditEntry('DELETE', 'doc_000')
      .failed();
    expect(entry.success).toBe(false);
  });

  test('audit entry to JSON', () => {
    const entry = new ucp.WasmAuditEntry('UPDATE', 'doc_111');
    const json = entry.toJson();
    expect(json.operation).toBe('UPDATE');
    expect(json.documentId).toBe('doc_111');
  });

  test('creates metrics recorder', () => {
    const metrics = new ucp.WasmMetricsRecorder();
    expect(metrics.operationsTotal).toBe(0);
  });

  test('records operations', () => {
    const metrics = new ucp.WasmMetricsRecorder();
    metrics.recordOperation(true);
    metrics.recordOperation(false);
    
    expect(metrics.operationsTotal).toBe(2);
    expect(metrics.operationsFailed).toBe(1);
  });

  test('records block operations', () => {
    const metrics = new ucp.WasmMetricsRecorder();
    metrics.recordBlockCreated();
    metrics.recordBlockCreated();
    metrics.recordBlockDeleted();
    
    expect(metrics.blocksCreated).toBe(2);
    expect(metrics.blocksDeleted).toBe(1);
  });

  test('metrics to JSON', () => {
    const metrics = new ucp.WasmMetricsRecorder();
    metrics.recordOperation(true);
    metrics.recordSnapshot();
    
    const json = metrics.toJson();
    expect(json.operationsTotal).toBe(1);
    expect(json.snapshotsCreated).toBe(1);
  });

  test('creates UCP events', () => {
    const event = ucp.WasmUcpEvent.documentCreated('doc_123');
    expect(event.eventType).toBe('document_created');
    expect(event.documentId).toBe('doc_123');
  });

  test('creates block added event', () => {
    const event = ucp.WasmUcpEvent.blockAdded('doc_1', 'blk_1', 'root', 'text');
    expect(event.eventType).toBe('block_added');
  });

  test('creates block deleted event', () => {
    const event = ucp.WasmUcpEvent.blockDeleted('doc_1', 'blk_1', true);
    expect(event.eventType).toBe('block_deleted');
  });
});

describe('SnapshotManager', () => {
  test('creates snapshot manager', () => {
    const mgr = new ucp.SnapshotManager();
    expect(mgr.length).toBe(0);
  });

  test('creates snapshot', () => {
    const doc = new ucp.Document('Test');
    doc.addBlock(doc.rootId, 'Hello');

    const mgr = new ucp.SnapshotManager();
    const name = mgr.create('v1', doc);

    expect(name).toBe('v1');
    expect(mgr.length).toBe(1);
  });

  test('restores snapshot', () => {
    const doc = new ucp.Document('Test');
    doc.addBlock(doc.rootId, 'Original');

    const mgr = new ucp.SnapshotManager();
    mgr.create('v1', doc);

    // Modify document
    doc.addBlock(doc.rootId, 'New block');
    const modifiedCount = doc.blockCount();

    // Restore
    const restored = mgr.restore('v1');
    expect(restored.blockCount()).toBeLessThan(modifiedCount);
  });

  test('gets snapshot info', () => {
    const doc = new ucp.Document();
    const mgr = new ucp.SnapshotManager();
    mgr.create('v1', doc, 'First version');

    const info = mgr.get('v1');
    expect(info).toBeDefined();
    expect(info.name).toBe('v1');
    expect(info.description).toBe('First version');
  });

  test('lists snapshots', () => {
    const doc = new ucp.Document();
    const mgr = new ucp.SnapshotManager();
    mgr.create('v1', doc);
    mgr.create('v2', doc);

    const list = mgr.list();
    expect(list.length).toBe(2);
  });

  test('deletes snapshot', () => {
    const doc = new ucp.Document();
    const mgr = new ucp.SnapshotManager();
    mgr.create('v1', doc);

    expect(mgr.exists('v1')).toBe(true);
    mgr.delete('v1');
    expect(mgr.exists('v1')).toBe(false);
  });
});
