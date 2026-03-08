## UCP graph runtime demo transcript

This transcript demonstrates generic graph traversal over a plain UCP document, plus JSON-backed and SQLite-backed graph persistence.

## In-memory graph stats

```json
{
  "backend": "memory",
  "captured_at": "2026-03-08T19:25:20.399883882Z",
  "document_id": "doc_189af43700e8f0a1",
  "explicit_edge_count": 1,
  "node_count": 4,
  "root_block_id": "ff0000000000000000000000",
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
    "captured_at": "2026-03-08T19:25:20.424267997Z",
    "document_id": "doc_189af43700e8f0a1",
    "explicit_edge_count": 1,
    "graph_key": "demo",
    "node_count": 4,
    "root_block_id": "ff0000000000000000000000",
    "structural_edge_count": 3
  }
}
```

## Regex graph search

```json
[
  {
    "block_id": "288c3139f7400323f96ed67e",
    "children": 0,
    "content_type": "code",
    "incoming_edges": 1,
    "label": "helper",
    "outgoing_edges": 0,
    "parent": "6a050bd9f8f78bb95dd911ab",
    "tags": []
  },
  {
    "block_id": "647a8f492b004bb420732faf",
    "children": 0,
    "content_type": "text",
    "incoming_edges": 0,
    "label": "note",
    "outgoing_edges": 1,
    "parent": "6a050bd9f8f78bb95dd911ab",
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
    "block_id": "288c3139f7400323f96ed67e",
    "children": 0,
    "content_type": "code",
    "incoming_edges": 1,
    "label": "helper",
    "outgoing_edges": 0,
    "parent": "6a050bd9f8f78bb95dd911ab",
    "tags": []
  },
  "hops": [
    {
      "direction": "outgoing",
      "from": "647a8f492b004bb420732faf",
      "relation": "references",
      "to": "288c3139f7400323f96ed67e"
    }
  ],
  "start": {
    "block_id": "647a8f492b004bb420732faf",
    "children": 0,
    "content_type": "text",
    "incoming_edges": 0,
    "label": "note",
    "outgoing_edges": 1,
    "parent": "6a050bd9f8f78bb95dd911ab",
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
    "ff0000000000000000000000",
    "6a050bd9f8f78bb95dd911ab"
  ],
  "changed": [],
  "removed": [],
  "warnings": []
}
```

## Select note

```json
{
  "added": [
    "647a8f492b004bb420732faf"
  ],
  "changed": [],
  "removed": [],
  "warnings": []
}
```

## Expand outgoing edges from note

```json
{
  "added": [
    "288c3139f7400323f96ed67e"
  ],
  "changed": [],
  "removed": [],
  "warnings": []
}
```

## Why helper is selected

```json
{
  "anchor": {
    "block_id": "647a8f492b004bb420732faf",
    "children": 0,
    "content_type": "text",
    "incoming_edges": 0,
    "label": "note",
    "outgoing_edges": 1,
    "parent": "6a050bd9f8f78bb95dd911ab",
    "semantic_role": "paragraph",
    "tags": [
      "important",
      "demo"
    ]
  },
  "block_id": "288c3139f7400323f96ed67e",
  "detail_level": "summary",
  "explanation": "Node was selected while following outgoing semantic edges.",
  "focus": false,
  "node": {
    "block_id": "288c3139f7400323f96ed67e",
    "children": 0,
    "content_type": "code",
    "incoming_edges": 1,
    "label": "helper",
    "outgoing_edges": 0,
    "parent": "6a050bd9f8f78bb95dd911ab",
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
      "source": "6a050bd9f8f78bb95dd911ab",
      "target": "288c3139f7400323f96ed67e"
    },
    {
      "direction": "structural",
      "relation": "contains",
      "source": "6a050bd9f8f78bb95dd911ab",
      "target": "647a8f492b004bb420732faf"
    },
    {
      "direction": "structural",
      "relation": "contains",
      "source": "ff0000000000000000000000",
      "target": "6a050bd9f8f78bb95dd911ab"
    },
    {
      "direction": "structural",
      "relation": "parent",
      "source": "288c3139f7400323f96ed67e",
      "target": "6a050bd9f8f78bb95dd911ab"
    },
    {
      "direction": "structural",
      "relation": "parent",
      "source": "647a8f492b004bb420732faf",
      "target": "6a050bd9f8f78bb95dd911ab"
    },
    {
      "direction": "structural",
      "relation": "parent",
      "source": "6a050bd9f8f78bb95dd911ab",
      "target": "ff0000000000000000000000"
    },
    {
      "direction": "incoming",
      "relation": "references",
      "source": "288c3139f7400323f96ed67e",
      "target": "647a8f492b004bb420732faf"
    },
    {
      "direction": "outgoing",
      "relation": "references",
      "source": "647a8f492b004bb420732faf",
      "target": "288c3139f7400323f96ed67e"
    }
  ],
  "nodes": [
    {
      "block_id": "ff0000000000000000000000",
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
      "block_id": "288c3139f7400323f96ed67e",
      "children": 0,
      "content_type": "code",
      "detail_level": "summary",
      "incoming_edges": 1,
      "label": "helper",
      "outgoing_edges": 0,
      "parent": "6a050bd9f8f78bb95dd911ab",
      "pinned": false,
      "tags": []
    },
    {
      "block_id": "647a8f492b004bb420732faf",
      "children": 0,
      "content_type": "text",
      "detail_level": "full",
      "incoming_edges": 0,
      "label": "note",
      "outgoing_edges": 1,
      "parent": "6a050bd9f8f78bb95dd911ab",
      "pinned": false,
      "semantic_role": "paragraph",
      "tags": [
        "important",
        "demo"
      ]
    },
    {
      "block_id": "6a050bd9f8f78bb95dd911ab",
      "children": 2,
      "content_type": "text",
      "detail_level": "summary",
      "incoming_edges": 0,
      "label": "section",
      "outgoing_edges": 0,
      "parent": "ff0000000000000000000000",
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
