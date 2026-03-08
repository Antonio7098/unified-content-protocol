## Codegraph programmatic API demo transcript

This transcript manually exercises the new Python CodeGraph API for agent-style traversal,
including regex discovery, stateful expansion, provenance inspection, frontier-driven actions,
path finding, and coderef-backed source hydration.

## Build repository graph

```json
{
  "nodes": 5471,
  "repr": "CodeGraph(nodes=5471)"
}
```

## Seed overview

```json
{
  "added": [
    "99b62cfb483779db26d3be75",
    "11f2222350db8eee5bc5ef71",
    "7cf2a27244897a4b9b334414",
    "1463cfacf2bd8c2604b76bde",
    "6a0e503cbb91ad4b20bf7083",
    "54c55b45f69468fdb717d1f2",
    "8deca5103a3b10e631b1fa62",
    "3043405a7c5239bd734b2298",
    "27dd45b9f4f423b3da20d031",
    "123d8e68e78ebec442607b86",
    "686b241bbbc1ebd533b4fb9c",
    "d2cef3475e8ab2b5ed3ec587",
    "3febc0ada88477685bc61d98",
    "e75297e577b337ca2fb68a7a",
    "6c752b79da406a1c951e3a8b",
    "b07c00f9cb6f24a8023c3cf2",
    "27181c8b52e4b06023cc9774",
    "d5685d95a7229a5fb2b8a2e3",
    "73114ea8e67a3c5edd51420d",
    "da1e5784051bb4cf30e6d66b",
    "35d1be9f87753900056d83e5"
  ],
  "changed": [],
  "focus": "99b62cfb483779db26d3be75",
  "removed": [],
  "warnings": []
}
```

## Expand file symbols for crates/ucp-cli/src/commands/agent.rs

```json
{
  "added": [
    "421333df5881c48ef6b4be16",
    "6235791ed9ac4d7a14aac319",
    "4f940e434b235b682b66a993",
    "22cc5420d70d16fc97e384f9",
    "61b08fbe32bf13550b809e0e",
    "81f3d43984931b527f7f03e1",
    "17f326535b810acf87cb5480",
    "5cfb844f59b2ef0e148b802e",
    "0cb4f27ad738e059268f66dc",
    "33c464f1fa016a2a873ce023",
    "32c2f79214c85e15ed7111ca",
    "388c43cdf82cf3be4c3a2de3",
    "2fe932adcdb289e658dd95c3",
    "75ba4b5be43d622dbdbea654",
    "4e5834618eba57dc2705bf21",
    "2f2ff11e7823297e4a3a58ca",
    "038a0d36c4bfe68f61ae207d",
    "ab3db77747df355713964772",
    "36297514b84ae49ca30429cf",
    "878416793291749c3f8c45f0",
    "1c2377fdb5c6e58265653ec2",
    "7573ba527b717f92b1f9ff37",
    "a03f786951e9056adeb7d847",
    "9cb55c80be4d1e6e672635b5",
    "252a08abfd351262785f1af7",
    "83fe331988dd1204ee617928",
    "83c18aaef4cd71a1c2471814"
  ],
  "changed": [
    "ab3db77747df355713964772",
    "a03f786951e9056adeb7d847",
    "9cb55c80be4d1e6e672635b5",
    "878416793291749c3f8c45f0",
    "83fe331988dd1204ee617928",
    "83c18aaef4cd71a1c2471814",
    "81f3d43984931b527f7f03e1",
    "75ba4b5be43d622dbdbea654",
    "7573ba527b717f92b1f9ff37",
    "6235791ed9ac4d7a14aac319",
    "61b08fbe32bf13550b809e0e",
    "5cfb844f59b2ef0e148b802e",
    "4f940e434b235b682b66a993",
    "4e5834618eba57dc2705bf21",
    "388c43cdf82cf3be4c3a2de3",
    "36297514b84ae49ca30429cf",
    "33c464f1fa016a2a873ce023",
    "32c2f79214c85e15ed7111ca",
    "2fe932adcdb289e658dd95c3",
    "2f2ff11e7823297e4a3a58ca",
    "252a08abfd351262785f1af7",
    "22cc5420d70d16fc97e384f9",
    "1c2377fdb5c6e58265653ec2",
    "17f326535b810acf87cb5480",
    "0cb4f27ad738e059268f66dc",
    "038a0d36c4bfe68f61ae207d"
  ],
  "focus": "421333df5881c48ef6b4be16",
  "removed": [
    "fcbaff9c723b3c545893b460",
    "f97e4beae70a52e156496818",
    "effc5cb92c68fc2fc7c6060c",
    "ec5990bce475a7f5e5b35c8f",
    "e82bc834beb1e93b6beb9320",
    "e804bec6a3d59ded23434300",
    "e2bd3b36c044c451a0f19fc5",
    "e1461da0092b93ba436329f5",
    "dabdd7fa1f031b5b15d50adf",
    "da1643f054f4e69d501e077f",
    "d26b84666d6b90b00dc8abb9",
    "ce4deb0d861f1c377b1fb833",
    "caa0b63234a21634ac29a13c",
    "ca0802fbc374965adcf94c36",
    "bb03ce046fceeccdba185a54",
    "b2bc11f492b1d0c47f16e483",
    "b0cf9a52abda636795e63fd5"
  ],
  "warnings": []
}
```

## Expand file symbols for crates/ucp-cli/src/commands/codegraph.rs

```json
{
  "added": [
    "e4fc6ac96af7e1f5b261fe1f",
    "3646aae8c636bd629cb14366",
    "161c61d5da7740a0bc13a64e",
    "375556c414d9721671b244c7",
    "26e431149695347e143b19cf",
    "047a6b142bd13e2db299bf35",
    "3151bec71c3e401b32d89e13",
    "040772f35cb58849119a71d3",
    "1d53058ff34da01f81742a3a",
    "3103889e0868639d19d5dd3a",
    "062fa5d2fb2e61baf1c646f3",
    "014ce77409a8733bccd0f8d0",
    "0e7d0f83b415d245f8c23dc2",
    "2161df5de0a7600080bb8616",
    "19cfcdf5e1c6dacfa6119063"
  ],
  "changed": [
    "375556c414d9721671b244c7",
    "3646aae8c636bd629cb14366",
    "3151bec71c3e401b32d89e13",
    "3103889e0868639d19d5dd3a",
    "26e431149695347e143b19cf",
    "2161df5de0a7600080bb8616",
    "1d53058ff34da01f81742a3a",
    "19cfcdf5e1c6dacfa6119063",
    "161c61d5da7740a0bc13a64e",
    "0e7d0f83b415d245f8c23dc2",
    "062fa5d2fb2e61baf1c646f3",
    "047a6b142bd13e2db299bf35",
    "040772f35cb58849119a71d3",
    "014ce77409a8733bccd0f8d0",
    "421333df5881c48ef6b4be16"
  ],
  "focus": "e4fc6ac96af7e1f5b261fe1f",
  "removed": [
    "fbfdddbcc02c8067b53b27fe",
    "f47adb3aefb90cb841185e13",
    "eb004d6c64bc2e36e41ea42e",
    "e9e0aaa367d526981ddb3bd8",
    "e81a9910a424fc896400553d",
    "e28fb03581432f00751e5aa1",
    "db3f7f37e4a98c80ab7b56e3",
    "d1fab9b7f09d11d524cad890",
    "d0d52950985309a94e6f6850",
    "c7e49d010c98625831da4ac0",
    "bd92be797081a333ff5fe2ef",
    "b88ee3d1f959fb29c50e3394",
    "b738f29aa1ec2082a1ec17ba",
    "b50a20b247a70d746a6eecf9",
    "b1514cfae7b8f5469d4e1fd0",
    "ab3db77747df355713964772",
    "a9da1b867af922f88886b437",
    "a744b420301741a16b8fa63d",
    "a67dfd8379bec3da9c9282ec",
    "a03f786951e9056adeb7d847",
    "9cb55c80be4d1e6e672635b5",
    "8bf08260fbf72b9acc4dfff8",
    "89a19cc0e721bb3337a34809",
    "878416793291749c3f8c45f0",
    "83fe331988dd1204ee617928",
    "83c18aaef4cd71a1c2471814",
    "81f3d43984931b527f7f03e1",
    "75ba4b5be43d622dbdbea654",
    "7573ba527b717f92b1f9ff37",
    "72378fe9f8f5b777ae450ac1",
    "6f33d3963c5f676aef6346bf",
    "633ebbcde7448a93b661f796",
    "6235791ed9ac4d7a14aac319",
    "61b08fbe32bf13550b809e0e",
    "5cfb844f59b2ef0e148b802e",
    "58274abbe22f3a5e56527269",
    "53fa15e7333e649f4eb74d9c",
    "4f940e434b235b682b66a993",
    "4e5834618eba57dc2705bf21",
    "471425c007861b13a70219c9",
    "417c1f9bfe4c790b8f40b26c",
    "388c43cdf82cf3be4c3a2de3",
    "384b68c8c1725eba6a9eb9da"
... clipped 3 more lines ...

```

## Expand file symbols for crates/ucp-codegraph/src/context.rs

```json
{
  "added": [
    "a3e405bc4c3b866ba88b4abf",
    "01ed95dc82a44e58ddd09413",
    "067cf50fe3a960d4a9abf46e",
    "11da0dff38408a4acce6ffa1",
    "0381c32bbd548c3242bcb6d5",
    "0a3801fdf0420775e6e1d03c",
    "0053b58411298b4292bb2f75",
    "0c76d9bf7be13c3fcac16fdf",
    "03051d05dea43f33b67334f3",
    "100ad4273916c2d577d032c7",
    "0710c5abf5ca5e903a53cc2b",
    "0e941dccb05e78d98eceb563",
    "0367e9c75308d52846eaa2d1",
    "10ba15bb8defcf4a41a893a7",
    "0dcc972ff1c3efcfebeeb8bd",
    "0cded6f25ea15aaeac8f5676",
    "09203ddb0037c565c85efea3",
    "07082130fd7777bda29875f3"
  ],
  "changed": [
    "11da0dff38408a4acce6ffa1",
    "10ba15bb8defcf4a41a893a7",
    "100ad4273916c2d577d032c7",
    "0e941dccb05e78d98eceb563",
    "0dcc972ff1c3efcfebeeb8bd",
    "0cded6f25ea15aaeac8f5676",
    "0c76d9bf7be13c3fcac16fdf",
    "0a3801fdf0420775e6e1d03c",
    "09203ddb0037c565c85efea3",
    "0710c5abf5ca5e903a53cc2b",
    "07082130fd7777bda29875f3",
    "067cf50fe3a960d4a9abf46e",
    "0381c32bbd548c3242bcb6d5",
    "0367e9c75308d52846eaa2d1",
    "03051d05dea43f33b67334f3",
    "01ed95dc82a44e58ddd09413",
    "0053b58411298b4292bb2f75",
    "e4fc6ac96af7e1f5b261fe1f"
  ],
  "focus": "a3e405bc4c3b866ba88b4abf",
  "removed": [
    "fe0b229ce681f020e98ece18",
    "fc35a6f1e95bd9d5189d5f0b",
    "f9d8e38bb08b0efa1183e2f8",
    "f7e2b0de2e149648deed90b2",
    "f7dddbca107e88ccd6bb728a",
    "f3f7d547ca1d32730042f609",
    "f36543dac6e451cf1329f13d",
    "f11cc63188a6eb8d95ed69a2",
    "ee64daf298092e9b99bdb3e5",
    "ecca6ba880fde780c1c5d2cd",
    "ec0c871c10d4cb0c40feb7f9",
    "ec0816a9156e1dff05d3ffec",
    "ea2cc47c53a649693e270db9",
    "e906ea322f45cd7efc7e4f71",
    "e87c5f06952f1bd4e35c27c8",
    "e6dde791b9ca9b46bd9c03d7",
    "e52ea8e2dabdb702281b40e0",
    "e3f223155544d6ed1fbc1d55",
    "e08ae5dd8379dd0642e5d41b",
    "ddfcb11ee5e0926e2c32f37f",
    "d9d5cb61c1016ab0f44fdf66",
    "d5f6b7c8e629e8db9fde8cb0",
    "d582694aa475872b3479211e",
    "d1b4c434de790672bb420a17",
    "d1638a37550fd4eadf1ecdac",
    "d0656d4726bb94c70d35f474",
    "ce517652ab851da3bd3f2acb",
    "cdc3bf6dfe512c7c48535fb7",
    "cc709e9aaf921bcadd52c0b0",
    "c91928fb41334cc295a96b1a",
    "c8db92f70f39c22ae5265c03",
    "c82596b5d50c15d1694f219a",
    "c713dd7c6a0c6be47218fa67",
    "c5b3e67dac90fb0a005e2f36",
    "c316ea750038ca703f2a372e",
    "bd8e2e1069a0a01df340286a",
    "bc50ae99725041a517927232",
... clipped 126 more lines ...

```

## Find regex-matched seed symbols

```json
[
  {
    "block_id": "0cb4f27ad738e059268f66dc",
    "coderef": {
      "display": "crates/ucp-cli/src/commands/agent.rs#L1012-L1085",
      "end_line": 1085,
      "path": "crates/ucp-cli/src/commands/agent.rs",
      "start_line": 1012
    },
    "exported": false,
    "label": "context_show",
    "logical_key": "symbol:crates/ucp-cli/src/commands/agent.rs::context_show",
    "node_class": "symbol",
    "path": "crates/ucp-cli/src/commands/agent.rs",
    "symbol_name": "context_show"
  },
  {
    "block_id": "417c1f9bfe4c790b8f40b26c",
    "coderef": {
      "display": "crates/ucp-cli/src/commands/codegraph.rs#L657-L702",
      "end_line": 702,
      "path": "crates/ucp-cli/src/commands/codegraph.rs",
      "start_line": 657
    },
    "exported": false,
    "label": "context_show",
    "logical_key": "symbol:crates/ucp-cli/src/commands/codegraph.rs::context_show",
    "node_class": "symbol",
    "path": "crates/ucp-cli/src/commands/codegraph.rs",
    "symbol_name": "context_show"
  },
  {
    "block_id": "32c2f79214c85e15ed7111ca",
    "coderef": {
      "display": "crates/ucp-cli/src/commands/agent.rs#L1107-L1116",
      "end_line": 1116,
      "path": "crates/ucp-cli/src/commands/agent.rs",
      "start_line": 1107
    },
    "exported": false,
    "label": "get_session_mut",
    "logical_key": "symbol:crates/ucp-cli/src/commands/agent.rs::get_session_mut",
    "node_class": "symbol",
    "path": "crates/ucp-cli/src/commands/agent.rs",
    "symbol_name": "get_session_mut"
  },
  {
    "block_id": "f47adb3aefb90cb841185e13",
    "coderef": {
      "display": "crates/ucp-cli/src/commands/codegraph.rs#L1629-L1638",
      "end_line": 1638,
      "path": "crates/ucp-cli/src/commands/codegraph.rs",
      "start_line": 1629
    },
    "exported": false,
    "label": "get_session_mut",
    "logical_key": "symbol:crates/ucp-cli/src/commands/codegraph.rs::get_session_mut",
    "node_class": "symbol",
    "path": "crates/ucp-cli/src/commands/codegraph.rs",
    "symbol_name": "get_session_mut"
  },
  {
    "block_id": "89a19cc0e721bb3337a34809",
    "coderef": {
      "display": "crates/ucp-cli/src/commands/codegraph.rs#L1648-L1685",
      "end_line": 1685,
      "path": "crates/ucp-cli/src/commands/codegraph.rs",
      "start_line": 1648
    },
    "exported": false,
    "label": "print_context_update",
    "logical_key": "symbol:crates/ucp-cli/src/commands/codegraph.rs::print_context_update",
    "node_class": "symbol",
    "path": "crates/ucp-cli/src/commands/codegraph.rs",
    "symbol_name": "print_context_update"
  },
  {
    "block_id": "ec5990bce475a7f5e5b35c8f",
    "coderef": {
      "display": "crates/ucp-cli/src/commands/agent.rs#L1118-L1158",
... clipped 12 more lines ...

```

## Focus symbol:crates/ucp-cli/src/commands/agent.rs::context_show

```json
{
  "added": [],
  "changed": [
    "0cb4f27ad738e059268f66dc"
  ],
  "focus": "0cb4f27ad738e059268f66dc",
  "removed": [],
  "warnings": []
}
```

## Expand dependencies for symbol:crates/ucp-cli/src/commands/agent.rs::context_show

```json
{
  "added": [],
  "changed": [
    "0cb4f27ad738e059268f66dc",
    "a3e405bc4c3b866ba88b4abf"
  ],
  "focus": "0cb4f27ad738e059268f66dc",
  "removed": [
    "6557d3b244263e4971245831",
    "0c6c13995a670d18a116596b"
  ],
  "warnings": []
}
```

## Why is symbol:crates/ucp-cli/src/commands/agent.rs::context_show selected?

```json
{
  "block_id": "0cb4f27ad738e059268f66dc",
  "detail_level": "neighborhood",
  "explanation": "Node was selected directly by the agent.",
  "focus": true,
  "node": {
    "block_id": "0cb4f27ad738e059268f66dc",
    "coderef": {
      "display": "crates/ucp-cli/src/commands/agent.rs#L1012-L1085",
      "end_line": 1085,
      "path": "crates/ucp-cli/src/commands/agent.rs",
      "start_line": 1012
    },
    "exported": false,
    "label": "context_show",
    "logical_key": "symbol:crates/ucp-cli/src/commands/agent.rs::context_show",
    "node_class": "symbol",
    "path": "crates/ucp-cli/src/commands/agent.rs",
    "symbol_name": "context_show"
  },
  "origin": {
    "kind": "manual"
  },
  "pinned": false,
  "selected": true
}
```

## Hydrate symbol:crates/ucp-cli/src/commands/agent.rs::context_show

```json
{
  "added": [],
  "changed": [
    "0cb4f27ad738e059268f66dc",
    "0cb4f27ad738e059268f66dc"
  ],
  "focus": "0cb4f27ad738e059268f66dc",
  "removed": [],
  "warnings": []
}
```

## Apply recommended action near symbol:crates/ucp-cli/src/commands/agent.rs::context_show

```json
{
  "applied_actions": [
    "expand_dependencies S1 via uses_symbol"
  ],
  "update": {
    "added": [],
    "changed": [],
    "focus": "0cb4f27ad738e059268f66dc",
    "removed": [
      "6557d3b244263e4971245831",
      "0c6c13995a670d18a116596b"
    ],
    "warnings": []
  }
}
```

## Focus symbol:crates/ucp-cli/src/commands/codegraph.rs::context_show

```json
{
  "added": [
    "417c1f9bfe4c790b8f40b26c"
  ],
  "changed": [
    "0cb4f27ad738e059268f66dc"
  ],
  "focus": "417c1f9bfe4c790b8f40b26c",
  "removed": [
    "11da0dff38408a4acce6ffa1"
  ],
  "warnings": []
}
```

## Expand dependencies for symbol:crates/ucp-cli/src/commands/codegraph.rs::context_show

```json
{
  "added": [],
  "changed": [
    "417c1f9bfe4c790b8f40b26c"
  ],
  "focus": "417c1f9bfe4c790b8f40b26c",
  "removed": [
    "d1fab9b7f09d11d524cad890",
    "53fa15e7333e649f4eb74d9c",
    "384b68c8c1725eba6a9eb9da",
    "0c6c13995a670d18a116596b"
  ],
  "warnings": []
}
```

## Why is symbol:crates/ucp-cli/src/commands/codegraph.rs::context_show selected?

```json
{
  "block_id": "417c1f9bfe4c790b8f40b26c",
  "detail_level": "neighborhood",
  "explanation": "Node was selected directly by the agent.",
  "focus": true,
  "node": {
    "block_id": "417c1f9bfe4c790b8f40b26c",
    "coderef": {
      "display": "crates/ucp-cli/src/commands/codegraph.rs#L657-L702",
      "end_line": 702,
      "path": "crates/ucp-cli/src/commands/codegraph.rs",
      "start_line": 657
    },
    "exported": false,
    "label": "context_show",
    "logical_key": "symbol:crates/ucp-cli/src/commands/codegraph.rs::context_show",
    "node_class": "symbol",
    "path": "crates/ucp-cli/src/commands/codegraph.rs",
    "symbol_name": "context_show"
  },
  "origin": {
    "kind": "manual"
  },
  "pinned": false,
  "selected": true
}
```

## Hydrate symbol:crates/ucp-cli/src/commands/codegraph.rs::context_show

```json
{
  "added": [],
  "changed": [
    "417c1f9bfe4c790b8f40b26c",
    "417c1f9bfe4c790b8f40b26c"
  ],
  "focus": "417c1f9bfe4c790b8f40b26c",
  "removed": [],
  "warnings": []
}
```

## Apply recommended action near symbol:crates/ucp-cli/src/commands/codegraph.rs::context_show

```json
{
  "applied_actions": [
    "expand_dependencies S5 via uses_symbol"
  ],
  "update": {
    "added": [],
    "changed": [],
    "focus": "417c1f9bfe4c790b8f40b26c",
    "removed": [
      "d1fab9b7f09d11d524cad890",
      "53fa15e7333e649f4eb74d9c",
      "384b68c8c1725eba6a9eb9da",
      "0c6c13995a670d18a116596b"
    ],
    "warnings": []
  }
}
```

## Focus symbol:crates/ucp-cli/src/commands/agent.rs::get_session_mut

```json
{
  "added": [
    "32c2f79214c85e15ed7111ca"
  ],
  "changed": [
    "417c1f9bfe4c790b8f40b26c"
  ],
  "focus": "32c2f79214c85e15ed7111ca",
  "removed": [
    "10ba15bb8defcf4a41a893a7"
  ],
  "warnings": []
}
```

## Expand dependencies for symbol:crates/ucp-cli/src/commands/agent.rs::get_session_mut

```json
{
  "added": [],
  "changed": [
    "32c2f79214c85e15ed7111ca"
  ],
  "focus": "32c2f79214c85e15ed7111ca",
  "removed": [],
  "warnings": []
}
```

## Why is symbol:crates/ucp-cli/src/commands/agent.rs::get_session_mut selected?

```json
{
  "block_id": "32c2f79214c85e15ed7111ca",
  "detail_level": "neighborhood",
  "explanation": "Node was selected directly by the agent.",
  "focus": true,
  "node": {
    "block_id": "32c2f79214c85e15ed7111ca",
    "coderef": {
      "display": "crates/ucp-cli/src/commands/agent.rs#L1107-L1116",
      "end_line": 1116,
      "path": "crates/ucp-cli/src/commands/agent.rs",
      "start_line": 1107
    },
    "exported": false,
    "label": "get_session_mut",
    "logical_key": "symbol:crates/ucp-cli/src/commands/agent.rs::get_session_mut",
    "node_class": "symbol",
    "path": "crates/ucp-cli/src/commands/agent.rs",
    "symbol_name": "get_session_mut"
  },
  "origin": {
    "kind": "manual"
  },
  "pinned": false,
  "selected": true
}
```

## Hydrate symbol:crates/ucp-cli/src/commands/agent.rs::get_session_mut

```json
{
  "added": [],
  "changed": [
    "32c2f79214c85e15ed7111ca",
    "32c2f79214c85e15ed7111ca"
  ],
  "focus": "32c2f79214c85e15ed7111ca",
  "removed": [],
  "warnings": []
}
```

## Apply recommended action near symbol:crates/ucp-cli/src/commands/agent.rs::get_session_mut

```json
{
  "applied_actions": [
    "expand_dependents S2 via uses_symbol"
  ],
  "update": {
    "added": [],
    "changed": [],
    "focus": "32c2f79214c85e15ed7111ca",
    "removed": [
      "e2bd3b36c044c451a0f19fc5",
      "da1643f054f4e69d501e077f",
      "ca0802fbc374965adcf94c36",
      "81f3d43984931b527f7f03e1",
      "61b08fbe32bf13550b809e0e",
      "5cfb844f59b2ef0e148b802e",
      "4f940e434b235b682b66a993",
      "22cc5420d70d16fc97e384f9",
      "17f326535b810acf87cb5480"
    ],
    "warnings": []
  }
}
```

## Path between the first two seed symbols

```json
{
  "end": {
    "block_id": "417c1f9bfe4c790b8f40b26c",
    "coderef": {
      "display": "crates/ucp-cli/src/commands/codegraph.rs#L657-L702",
      "end_line": 702,
      "path": "crates/ucp-cli/src/commands/codegraph.rs",
      "start_line": 657
    },
    "exported": false,
    "label": "context_show",
    "logical_key": "symbol:crates/ucp-cli/src/commands/codegraph.rs::context_show",
    "node_class": "symbol",
    "path": "crates/ucp-cli/src/commands/codegraph.rs",
    "symbol_name": "context_show"
  },
  "hops": [
    {
      "direction": "outgoing",
      "from": "0cb4f27ad738e059268f66dc",
      "relation": "uses_symbol",
      "to": "0c6c13995a670d18a116596b"
    },
    {
      "direction": "incoming",
      "from": "0c6c13995a670d18a116596b",
      "relation": "uses_symbol",
      "to": "417c1f9bfe4c790b8f40b26c"
    }
  ],
  "start": {
    "block_id": "0cb4f27ad738e059268f66dc",
    "coderef": {
      "display": "crates/ucp-cli/src/commands/agent.rs#L1012-L1085",
      "end_line": 1085,
      "path": "crates/ucp-cli/src/commands/agent.rs",
      "start_line": 1012
    },
    "exported": false,
    "label": "context_show",
    "logical_key": "symbol:crates/ucp-cli/src/commands/agent.rs::context_show",
    "node_class": "symbol",
    "path": "crates/ucp-cli/src/commands/agent.rs",
    "symbol_name": "context_show"
  }
}
```

## Diff between base session and exploration branch

```json
{
  "added": [
    {
      "block_id": "417c1f9bfe4c790b8f40b26c",
      "coderef": {
        "display": "crates/ucp-cli/src/commands/codegraph.rs#L657-L702",
        "end_line": 702,
        "path": "crates/ucp-cli/src/commands/codegraph.rs",
        "start_line": 657
      },
      "exported": false,
      "label": "context_show",
      "logical_key": "symbol:crates/ucp-cli/src/commands/codegraph.rs::context_show",
      "node_class": "symbol",
      "path": "crates/ucp-cli/src/commands/codegraph.rs",
      "symbol_name": "context_show"
    },
    {
      "block_id": "32c2f79214c85e15ed7111ca",
      "coderef": {
        "display": "crates/ucp-cli/src/commands/agent.rs#L1107-L1116",
        "end_line": 1116,
        "path": "crates/ucp-cli/src/commands/agent.rs",
        "start_line": 1107
      },
      "exported": false,
      "label": "get_session_mut",
      "logical_key": "symbol:crates/ucp-cli/src/commands/agent.rs::get_session_mut",
      "node_class": "symbol",
      "path": "crates/ucp-cli/src/commands/agent.rs",
      "symbol_name": "get_session_mut"
    }
  ],
  "changed_focus": true,
  "focus_after": "32c2f79214c85e15ed7111ca",
  "focus_before": "a3e405bc4c3b866ba88b4abf",
  "removed": [
    {
      "block_id": "11da0dff38408a4acce6ffa1",
      "coderef": {
        "display": "crates/ucp-codegraph/src/context.rs#L237-L246",
        "end_line": 246,
        "path": "crates/ucp-codegraph/src/context.rs",
        "start_line": 237
      },
      "exported": false,
      "label": "CodeGraphTraversalConfig",
      "logical_key": "symbol:crates/ucp-codegraph/src/context.rs::CodeGraphTraversalConfig#237",
      "node_class": "symbol",
      "path": "crates/ucp-codegraph/src/context.rs",
      "symbol_name": "CodeGraphTraversalConfig"
    },
    {
      "block_id": "10ba15bb8defcf4a41a893a7",
      "coderef": {
        "display": "crates/ucp-codegraph/src/context.rs#L2125-L2133",
        "end_line": 2133,
        "path": "crates/ucp-codegraph/src/context.rs",
        "start_line": 2125
      },
      "exported": false,
      "label": "relation_prune_rank",
      "logical_key": "symbol:crates/ucp-codegraph/src/context.rs::relation_prune_rank",
      "node_class": "symbol",
      "path": "crates/ucp-codegraph/src/context.rs",
      "symbol_name": "relation_prune_rank"
    }
  ]
}
```

## Compact structured export from the exploration branch

```json
{
  "edges": [
    {
      "multiplicity": 1,
      "relation": "exports",
      "source": "a3e405bc4c3b866ba88b4abf",
      "source_short_id": "F3",
      "target": "01ed95dc82a44e58ddd09413",
      "target_short_id": "S10"
    },
    {
      "multiplicity": 1,
      "relation": "exports",
      "source": "a3e405bc4c3b866ba88b4abf",
      "source_short_id": "F3",
      "target": "0710c5abf5ca5e903a53cc2b",
      "target_short_id": "S21"
    },
    {
      "multiplicity": 1,
      "relation": "uses_symbol",
      "source": "040772f35cb58849119a71d3",
      "source_short_id": "S5",
      "target": "014ce77409a8733bccd0f8d0",
      "target_short_id": "S8"
    },
    {
      "multiplicity": 1,
      "relation": "uses_symbol",
      "source": "0e7d0f83b415d245f8c23dc2",
      "source_short_id": "S9",
      "target": "014ce77409a8733bccd0f8d0",
      "target_short_id": "S8"
    }
  ],
  "export_mode": "compact",
  "focus": "32c2f79214c85e15ed7111ca",
  "focus_label": "symbol:crates/ucp-cli/src/commands/agent.rs::get_session_mut",
  "focus_short_id": "S2",
  "frontier": [
    {
      "action": "hydrate_source",
      "block_id": "32c2f79214c85e15ed7111ca",
      "candidate_count": 0,
      "description": "Hydrate source for symbol:crates/ucp-cli/src/commands/agent.rs::get_session_mut",
      "priority": 120,
      "short_id": "S2"
    },
    {
      "action": "expand_dependents",
      "block_id": "32c2f79214c85e15ed7111ca",
      "candidate_count": 9,
      "description": "expand_dependents incoming neighbors via uses_symbol for symbol:crates/ucp-cli/src/commands/agent.rs::get_session_mut",
      "direction": "incoming",
      "priority": 77,
      "relation": "uses_symbol",
      "short_id": "S2"
    },
    {
      "action": "collapse",
      "block_id": "32c2f79214c85e15ed7111ca",
      "candidate_count": 1,
      "description": "Collapse symbol:crates/ucp-cli/src/commands/agent.rs::get_session_mut from working set",
      "priority": 6,
      "short_id": "S2"
    }
  ],
  "heuristics": {
    "hidden_candidate_count": 9,
    "low_value_candidate_count": 0,
    "recommended_actions": [
      {
        "action": "expand_dependents",
        "block_id": "32c2f79214c85e15ed7111ca",
        "candidate_count": 9,
        "description": "expand_dependents incoming neighbors via uses_symbol for symbol:crates/ucp-cli/src/commands/agent.rs::get_session_mut",
        "direction": "incoming",
        "priority": 77,
        "relation": "uses_symbol",
        "short_id": "S2"
... clipped 959 more lines ...

```

## Read coderef-backed excerpts from the final working set

### S2 `symbol:crates/ucp-cli/src/commands/agent.rs::get_session_mut`

- ref: `crates/ucp-cli/src/commands/agent.rs:1107-1116`

```rust
1105 }
1106 
1107 fn get_session_mut<'a>(
1108     stateful: &'a mut crate::state::StatefulDocument,
1109     session: &str,
1110 ) -> Result<&'a mut AgentSessionState> {
1111     stateful
1112         .state_mut()
1113         .sessions
1114         .get_mut(session)
1115         .ok_or_else(|| anyhow!("Session not found: {}", session))
1116 }
1117 
1118 fn print_context_update(
```

### S1 `symbol:crates/ucp-cli/src/commands/agent.rs::context_show`

- ref: `crates/ucp-cli/src/commands/agent.rs:1012-1085`

```rust
1010 }
1011 
1012 fn context_show(input: Option<String>, session: String, format: OutputFormat) -> Result<()> {
1013     let stateful = read_stateful_document(input)?;
1014 
1015     let sess = stateful
1016         .state()
1017         .sessions
1018         .get(&session)
1019         .ok_or_else(|| anyhow!("Session not found: {}", session))?;
1020 
1021     if is_codegraph_document(&stateful.document) {
1022         if let Some(context) = sess.codegraph_context.as_ref() {
1023             let rendered = render_codegraph_context_prompt(
1024                 &stateful.document,
1025                 context,
1026                 &CodeGraphRenderConfig::default(),
1027             );
1028             match format {
1029                 OutputFormat::Json => {
1030                     println!(
1031                         "{}",
1032                         serde_json::to_string_pretty(&serde_json::json!({
1033                             "session": session,
1034                             "focus": context.focus.map(|id| id.to_string()),
1035                             "summary": context.summary(&stateful.document),
1036                             "blocks": sess.context_blocks,
1037                             "rendered": rendered
1038                         }))?
1039                     );
1040                 }
1041                 OutputFormat::Text => {
1042                     println!("{}", rendered);
1043                 }
1044             }
1045             return Ok(());
1046         }
1047     }
1048 
1049     match format {
1050         OutputFormat::Json => {
1051             #[derive(Serialize)]
1052             struct ContextInfo {
1053                 session: String,
1054                 blocks: Vec<String>,
1055                 count: usize,
1056             }
1057             let result = ContextInfo {
1058                 session,
1059                 blocks: sess.context_blocks.clone(),
1060                 count: sess.context_blocks.len(),
1061             };
1062             println!("{}", serde_json::to_string_pretty(&result)?);
1063         }
1064         OutputFormat::Text => {
1065             println!("{}", "Context Window:".cyan().bold());
1066             if sess.context_blocks.is_empty() {
1067                 println!("  (empty)");
1068             } else {
1069                 for id in &sess.context_blocks {
1070                     if let Ok(block_id) = BlockId::from_str(id) {
1071                         if let Some(block) = stateful.document.get_block(&block_id) {
1072                             let preview = content_preview(&block.content, 60);
1073                             let preview_line = preview.lines().next().unwrap_or("");
1074                             println!("  [{}] {}", id.yellow(), preview_line.dimmed());
1075                         } else {
1076                             println!("  [{}] (block not found)", id.yellow());
1077                         }
1078                     }
1079                 }
1080             }
1081         }
1082     }
1083 
1084     Ok(())
1085 }
1086 
1087 fn resolve_selectors(doc: &ucm_core::Document, selectors: &str) -> Result<Vec<BlockId>> {
```

### F1 `file:crates/ucp-cli/src/commands/agent.rs`

- ref: `crates/ucp-cli/src/commands/agent.rs:None-None`

```rust
   1 //! Agent traversal commands
   2 
   3 use anyhow::{anyhow, Result};
```

### S3 `symbol:crates/ucp-cli/src/commands/agent.rs::session_create`

- ref: `crates/ucp-cli/src/commands/agent.rs:73-119`

```rust
  71 }
  72 
  73 fn session_create(
  74     input: Option<String>,
  75     name: Option<String>,
  76     start: Option<String>,
  77     format: OutputFormat,
  78 ) -> Result<()> {
  79     let mut stateful = read_stateful_document(input.clone())?;
  80 
  81     let start_block = if let Some(s) = start {
  82         Some(BlockId::from_str(&s).map_err(|_| anyhow!("Invalid start block ID: {}", s))?)
  83     } else {
  84         None
  85     };
  86 
  87     // Generate session ID
  88     let session_id = format!("sess_{}", uuid_short());
  89 
  90     let session = AgentSessionState::new(session_id.clone(), name.clone(), start_block);
  91     stateful
  92         .state_mut()
  93         .sessions
  94         .insert(session_id.clone(), session);
  95 
  96     match format {
  97         OutputFormat::Json => {
  98             #[derive(Serialize)]
  99             struct SessionResult {
 100                 success: bool,
 101                 session_id: String,
 102                 name: Option<String>,
 103             }
 104             let result = SessionResult {
 105                 success: true,
 106                 session_id,
 107                 name,
 108             };
 109             println!("{}", serde_json::to_string_pretty(&result)?);
 110         }
 111         OutputFormat::Text => {
 112             print_success(&format!("Created session: {}", session_id));
 113         }
 114     }
 115 
 116     // Write back to same input file or stdout
 117     write_stateful_document(&stateful, input)?;
 118     Ok(())
 119 }
 120 
 121 fn session_list(input: Option<String>, format: OutputFormat) -> Result<()> {
```

### S6 `symbol:crates/ucp-cli/src/commands/codegraph.rs::context_show`

- ref: `crates/ucp-cli/src/commands/codegraph.rs:657-702`

```rust
 655 }
 656 
 657 fn context_show(
 658     input: Option<String>,
 659     session: String,
 660     max_tokens: usize,
 661     compact: bool,
 662     no_rendered: bool,
 663     levels: Option<usize>,
 664     only: Option<String>,
 665     exclude: Option<String>,
 666     format: OutputFormat,
 667 ) -> Result<()> {
 668     let stateful = read_stateful_document(input)?;
 669     ensure_codegraph_document(&stateful.document)?;
 670     let sess = get_session(&stateful, &session)?;
 671     let context = sess
 672         .codegraph_context
 673         .as_ref()
 674         .ok_or_else(|| anyhow!("Session has no codegraph context: {}", session))?;
 675     let config = CodeGraphRenderConfig::for_max_tokens(max_tokens);
 676     let export_config = make_export_config(
 677         &sess.codegraph_preferences,
 678         compact,
 679         no_rendered,
 680         levels,
 681         only.as_deref(),
 682         exclude.as_deref(),
 683     )?;
 684     let export =
 685         export_codegraph_context_with_config(&stateful.document, context, &config, &export_config);
 686 
 687     match format {
 688         OutputFormat::Json => {
 689             let mut value = serde_json::to_value(&export)?;
 690             if let Some(object) = value.as_object_mut() {
 691                 object.insert("session".to_string(), serde_json::Value::String(session));
 692             }
 693             println!("{}", serde_json::to_string_pretty(&value)?);
 694         }
 695         OutputFormat::Text => println!(
 696             "{}",
 697             render_context_show_text(&stateful.document, context, &config, &export)
 698         ),
 699     }
 700 
 701     Ok(())
 702 }
 703 
 704 fn context_export(
```

### S10 `symbol:crates/ucp-codegraph/src/context.rs::CodeGraphCoderef`

- ref: `crates/ucp-codegraph/src/context.rs:277-284`

```rust
 275 
 276 #[derive(Debug, Clone, Serialize, Deserialize)]
 277 pub struct CodeGraphCoderef {
 278     pub path: String,
 279     pub display: String,
 280     #[serde(default, skip_serializing_if = "Option::is_none")]
 281     pub start_line: Option<usize>,
 282     #[serde(default, skip_serializing_if = "Option::is_none")]
 283     pub end_line: Option<usize>,
 284 }
 285 
 286 #[derive(Debug, Clone, Serialize, Deserialize)]
```

## Final summary

- selected nodes: 48
- frontier actions remaining: 3
- transcript file: `artifacts/codegraph-context-demo-transcript.md`
