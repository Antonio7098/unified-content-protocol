## UCP graph runtime demo transcript

This transcript demonstrates generic graph traversal over a plain UCP document, plus JSON-backed and SQLite-backed graph persistence.

## In-memory graph stats

```json
{
  "backend": "memory",
  "captured_at": "2026-03-08T20:42:42.832026881Z",
  "document_id": "doc_189af86fe76be463",
  "explicit_edge_count": 1,
  "node_count": 4,
  "root_block_id": "blk_ff0000000000000000000000",
  "structural_edge_count": 3
}
```

## SQLite graph observability

```json
{
  "indexed_fields": [
    "block_id",
    "label",
    "content_type",
    "semantic_role",
    "parent_block_id",
    "source_block_id",
    "target_block_id"
  ],
  "stats": {
    "backend": "sqlite",
    "captured_at": "2026-03-08T20:42:42.860452837Z",
    "document_id": "doc_189af86fe76be463",
    "explicit_edge_count": 1,
    "graph_key": "demo",
    "node_count": 4,
    "root_block_id": "blk_ff0000000000000000000000",
    "structural_edge_count": 3
  }
}
```

## Regex graph search

```json
[
  {
    "block_id": "blk_288c3139f7400323f96ed67e",
    "children": 0,
    "content_type": "code",
    "incoming_edges": 1,
    "label": "helper",
    "outgoing_edges": 0,
    "parent": "blk_6a050bd9f8f78bb95dd911ab",
    "tags": []
  },
  {
    "block_id": "blk_647a8f492b004bb420732faf",
    "children": 0,
    "content_type": "text",
    "incoming_edges": 0,
    "label": "note",
    "outgoing_edges": 1,
    "parent": "blk_6a050bd9f8f78bb95dd911ab",
    "semantic_role": "paragraph",
    "tags": [
      "important",
      "demo"
    ]
  }
]
```

## Path between note and helper

```json
{
  "end": {
    "block_id": "blk_288c3139f7400323f96ed67e",
    "children": 0,
    "content_type": "code",
    "incoming_edges": 1,
    "label": "helper",
    "outgoing_edges": 0,
    "parent": "blk_6a050bd9f8f78bb95dd911ab",
    "tags": []
  },
  "hops": [
    {
      "direction": "outgoing",
      "from": "blk_647a8f492b004bb420732faf",
      "relation": "references",
      "to": "blk_288c3139f7400323f96ed67e"
    }
  ],
  "start": {
    "block_id": "blk_647a8f492b004bb420732faf",
    "children": 0,
    "content_type": "text",
    "incoming_edges": 0,
    "label": "note",
    "outgoing_edges": 1,
    "parent": "blk_6a050bd9f8f78bb95dd911ab",
    "semantic_role": "paragraph",
    "tags": [
      "important",
      "demo"
    ]
  }
}
```

## Seed overview

```json
{
  "added": [
    "blk_ff0000000000000000000000",
    "blk_6a050bd9f8f78bb95dd911ab"
  ],
  "changed": [],
  "focus": null,
  "removed": [],
  "warnings": []
}
```

## Select note

```json
{
  "added": [
    "blk_647a8f492b004bb420732faf"
  ],
  "changed": [],
  "focus": null,
  "removed": [],
  "warnings": []
}
```

## Expand outgoing edges from note

```json
{
  "added": [
    "blk_288c3139f7400323f96ed67e"
  ],
  "changed": [],
  "focus": null,
  "removed": [],
  "warnings": []
}
```

## Why helper is selected

```json
{
  "anchor": {
    "block_id": "blk_647a8f492b004bb420732faf",
    "children": 0,
    "content_type": "text",
    "incoming_edges": 0,
    "label": "note",
    "outgoing_edges": 1,
    "parent": "blk_6a050bd9f8f78bb95dd911ab",
    "semantic_role": "paragraph",
    "tags": [
      "important",
      "demo"
    ]
  },
  "block_id": "blk_288c3139f7400323f96ed67e",
  "detail_level": "summary",
  "explanation": "Node was selected while following outgoing semantic edges.",
  "focus": false,
  "node": {
    "block_id": "blk_288c3139f7400323f96ed67e",
    "children": 0,
    "content_type": "code",
    "incoming_edges": 1,
    "label": "helper",
    "outgoing_edges": 0,
    "parent": "blk_6a050bd9f8f78bb95dd911ab",
    "tags": []
  },
  "origin": {
    "anchor": "647a8f492b004bb420732faf",
    "kind": "outgoing",
    "relation": "references"
  },
  "pinned": false,
  "selected": true
}
```

## Exported working set

```json
{
  "edges": [
    {
      "direction": "structural",
      "relation": "contains",
      "source": "blk_6a050bd9f8f78bb95dd911ab",
      "target": "blk_288c3139f7400323f96ed67e"
    },
    {
      "direction": "structural",
      "relation": "contains",
      "source": "blk_6a050bd9f8f78bb95dd911ab",
      "target": "blk_647a8f492b004bb420732faf"
    },
    {
      "direction": "structural",
      "relation": "contains",
      "source": "blk_ff0000000000000000000000",
      "target": "blk_6a050bd9f8f78bb95dd911ab"
    },
    {
      "direction": "structural",
      "relation": "parent",
      "source": "blk_288c3139f7400323f96ed67e",
      "target": "blk_6a050bd9f8f78bb95dd911ab"
    },
    {
      "direction": "structural",
      "relation": "parent",
      "source": "blk_647a8f492b004bb420732faf",
      "target": "blk_6a050bd9f8f78bb95dd911ab"
    },
    {
      "direction": "structural",
      "relation": "parent",
      "source": "blk_6a050bd9f8f78bb95dd911ab",
      "target": "blk_ff0000000000000000000000"
    },
    {
      "direction": "incoming",
      "relation": "references",
      "source": "blk_288c3139f7400323f96ed67e",
      "target": "blk_647a8f492b004bb420732faf"
    },
    {
      "direction": "outgoing",
      "relation": "references",
      "source": "blk_647a8f492b004bb420732faf",
      "target": "blk_288c3139f7400323f96ed67e"
    }
  ],
  "nodes": [
    {
      "block_id": "blk_ff0000000000000000000000",
      "children": 1,
      "content_type": "text",
      "detail_level": "summary",
      "incoming_edges": 0,
      "label": "blk_ff00",
      "outgoing_edges": 0,
      "pinned": false,
      "tags": []
    },
    {
      "block_id": "blk_288c3139f7400323f96ed67e",
      "children": 0,
      "content_type": "code",
      "detail_level": "summary",
      "incoming_edges": 1,
      "label": "helper",
      "outgoing_edges": 0,
      "parent": "blk_6a050bd9f8f78bb95dd911ab",
      "pinned": false,
      "tags": []
    },
    {
      "block_id": "blk_647a8f492b004bb420732faf",
      "children": 0,
      "content_type": "text",
      "detail_level": "full",
      "incoming_edges": 0,
      "label": "note",
      "outgoing_edges": 1,
      "parent": "blk_6a050bd9f8f78bb95dd911ab",
      "pinned": false,
      "semantic_role": "paragraph",
      "tags": [
        "important",
        "demo"
      ]
    },
    {
      "block_id": "blk_6a050bd9f8f78bb95dd911ab",
      "children": 2,
      "content_type": "text",
      "detail_level": "summary",
      "incoming_edges": 0,
      "label": "section",
      "outgoing_edges": 0,
      "parent": "blk_ff0000000000000000000000",
      "pinned": false,
      "tags": []
    }
  ],
  "summary": {
    "focused": false,
    "leaves": 2,
    "pinned": 0,
    "roots": 1,
    "selected": 4
  }
}
```

## Final summary

- JSON artifact: `/home/antonio/programming/Hivemind/unified-content-protocol/artifacts/ucp-graph-runtime-demo.json`
- SQLite artifact: `/home/antonio/programming/Hivemind/unified-content-protocol/artifacts/ucp-graph-runtime-demo.db`
- selected nodes: 4
- transcript: `artifacts/ucp-graph-runtime-demo-transcript.md`
