## UCP Python query façade demo

This transcript demonstrates the thin agent-facing Python façade over the generic graph runtime, using loops, regex, and conditional traversal without a separate graph DSL.

## Raw graph stats

```json
{
  "backend": "memory",
  "captured_at": "2026-03-09T11:08:42.630258370Z",
  "document_id": "doc_189b27b1bcbe189a",
  "explicit_edge_count": 1,
  "node_count": 4,
  "root_block_id": "blk_ff0000000000000000000000",
  "structural_edge_count": 3
}
```

## Facade graph.find(...) results

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
    "block_id": "blk_0788a26e14a5194864d22deb",
    "children": 0,
    "content_type": "text",
    "incoming_edges": 0,
    "label": "note",
    "outgoing_edges": 1,
    "parent": "blk_6a050bd9f8f78bb95dd911ab",
    "semantic_role": "paragraph",
    "tags": [
      "important"
    ]
  }
]
```

## Query runner result

```json
{
  "error": null,
  "export": {
    "edges": [
      {
        "direction": "incoming",
        "relation": "cited_by",
        "source": "blk_288c3139f7400323f96ed67e",
        "target": "blk_0788a26e14a5194864d22deb"
      },
      {
        "direction": "outgoing",
        "relation": "references",
        "source": "blk_0788a26e14a5194864d22deb",
        "target": "blk_288c3139f7400323f96ed67e"
      }
    ],
    "nodes": [
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
        "block_id": "blk_0788a26e14a5194864d22deb",
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
          "important"
        ]
      }
    ],
    "summary": {
      "focused": false,
      "leaves": 2,
      "pinned": 0,
      "roots": 0,
      "selected": 2
    }
  },
  "limits": {
    "max_operations": null,
    "max_seconds": null,
    "max_stdout_chars": null,
    "max_trace_events": null
  },
  "ok": true,
  "result": {
    "labels": [
      "helper",
      "note"
    ],
    "path": {
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
          "direction": "structural",
          "from": "blk_ff0000000000000000000000",
          "relation": "contains",
          "to": "blk_6a050bd9f8f78bb95dd911ab"
        },
        {
          "direction": "structural",
          "from": "blk_6a050bd9f8f78bb95dd911ab",
          "relation": "contains",
          "to": "blk_288c3139f7400323f96ed67e"
        }
      ],
      "start": {
        "block_id": "blk_ff0000000000000000000000",
        "children": 1,
        "content_type": "text",
        "incoming_edges": 0,
        "label": "blk_ff00",
        "outgoing_edges": 0,
        "tags": []
      }
    },
    "why_helper": {
      "anchor": {
        "block_id": "blk_0788a26e14a5194864d22deb",
        "children": 0,
        "content_type": "text",
        "incoming_edges": 0,
        "label": "note",
        "outgoing_edges": 1,
        "parent": "blk_6a050bd9f8f78bb95dd911ab",
        "semantic_role": "paragraph",
        "tags": [
          "important"
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
        "anchor": "0788a26e14a5194864d22deb",
        "kind": "outgoing",
        "relation": "references"
      },
      "pinned": false,
      "selected": true
    }
  },
  "selected_block_ids": [
    "blk_0788a26e14a5194864d22deb",
    "blk_288c3139f7400323f96ed67e"
  ],
  "stdout": "",
  "summary": {
    "focused": false,
    "leaves": 2,
    "pinned": 0,
    "roots": 0,
    "selected": 2
  },
  "usage": {
    "elapsed_seconds": 0.010325,
    "operation_count": 5,
    "stdout_chars": 0,
    "trace_events": 701
  }
}
```

## Final session export

```json
{
  "edges": [
    {
      "direction": "incoming",
      "relation": "cited_by",
      "source": "blk_288c3139f7400323f96ed67e",
      "target": "blk_0788a26e14a5194864d22deb"
    },
    {
      "direction": "outgoing",
      "relation": "references",
      "source": "blk_0788a26e14a5194864d22deb",
      "target": "blk_288c3139f7400323f96ed67e"
    }
  ],
  "nodes": [
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
      "block_id": "blk_0788a26e14a5194864d22deb",
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
        "important"
      ]
    }
  ],
  "summary": {
    "focused": false,
    "leaves": 2,
    "pinned": 0,
    "roots": 0,
    "selected": 2
  }
}
```

## Final summary

- ok: True
- selected nodes: 2
- transcript: `artifacts/ucp-python-query-demo-transcript.md`
