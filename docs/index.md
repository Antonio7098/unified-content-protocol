# Unified Content Protocol

Welcome to the Unified Content Protocol (UCP) documentation. This site explains how to use the JavaScript and Python SDKs to build LLM-native content editors, automated document transformations, and benchmarking pipelines.

## What is UCP?

UCP is a structured content layer designed for large language model workflows. It combines:

- **Unified Content Model (UCM)** — immutable block-based document graph
- **Unified Content Language (UCL)** — declarative command language for edits
- **UCP Bench** — benchmarking harness for evaluating LLM editing quality
- **SDKs** — portable TypeScript & Python libraries for integrating UCP from any stack

## Why UCP?

1. **Model-friendly structure** — Content is block-addressable and consistent across markdown, JSON, LLM prompts, etc.
2. **Reliable editing** — UCL commands are verifiable and can be replayed, diffed, or canonicalized.
3. **Token efficiency** — Built-in ID mapping keeps prompts compact without losing referential integrity.
4. **Cross-language parity** — Identical JS/TS and Python APIs make it easy to build on both server and client.

## Core Concepts

| Concept | Description |
| --- | --- |
| **Document** | A tree of blocks, each block has a semantic role (heading, paragraph, quote, etc.) |
| **Block ID** | Stable identifier derived from content; used in UCL commands |
| **Prompt Builder** | Capability-aware system prompt generator for LLMs |
| **ID Mapper** | Maps long block IDs to short numbers for token savings |
| **UCL Builder** | Fluent API for generating valid UCL commands programmatically |

## Quick Links

- [Getting Started](getting-started.md)
- [JavaScript SDK](sdk/javascript.md)
- [Python SDK](sdk/python.md)
- [Examples](examples.md)
- [Release Notes](releases.md)
