use serde_json::Value;
use std::collections::HashMap;
use ucm_core::{Block, BlockId, Content, Document};

pub mod ids {
    use ucm_core::BlockId;

    pub fn root() -> BlockId { BlockId::root() }
    pub fn metadata() -> BlockId { BlockId::from_hex("200000000001").unwrap() }
    pub fn exec_summary() -> BlockId { BlockId::from_hex("200000000010").unwrap() }
    pub fn exec_objectives() -> BlockId { BlockId::from_hex("200000000011").unwrap() }
    pub fn exec_metrics() -> BlockId { BlockId::from_hex("200000000012").unwrap() }
    pub fn scenario_hub() -> BlockId { BlockId::from_hex("200000000020").unwrap() }
    pub fn scenario_table() -> BlockId { BlockId::from_hex("200000000021").unwrap() }
    pub fn scenario_edge_cases() -> BlockId { BlockId::from_hex("200000000022").unwrap() }
    pub fn dataset_section() -> BlockId { BlockId::from_hex("200000000030").unwrap() }
    pub fn dataset_card() -> BlockId { BlockId::from_hex("200000000031").unwrap() }
    pub fn dataset_stats() -> BlockId { BlockId::from_hex("200000000032").unwrap() }
    pub fn prompt_library() -> BlockId { BlockId::from_hex("200000000040").unwrap() }
    pub fn prompt_zero_shot() -> BlockId { BlockId::from_hex("200000000041").unwrap() }
    pub fn prompt_chain_of_thought() -> BlockId { BlockId::from_hex("200000000042").unwrap() }
    pub fn prompt_guardrails() -> BlockId { BlockId::from_hex("200000000043").unwrap() }
    pub fn metrics_section() -> BlockId { BlockId::from_hex("200000000050").unwrap() }
    pub fn metrics_table() -> BlockId { BlockId::from_hex("200000000051").unwrap() }
    pub fn metrics_formula() -> BlockId { BlockId::from_hex("200000000052").unwrap() }
    pub fn transcript_section() -> BlockId { BlockId::from_hex("200000000060").unwrap() }
    pub fn transcript_json() -> BlockId { BlockId::from_hex("200000000061").unwrap() }
    pub fn integration_section() -> BlockId { BlockId::from_hex("200000000070").unwrap() }
    pub fn integration_code() -> BlockId { BlockId::from_hex("200000000071").unwrap() }
    pub fn integration_config() -> BlockId { BlockId::from_hex("200000000072").unwrap() }
    pub fn call_to_action() -> BlockId { BlockId::from_hex("200000000080").unwrap() }
}

pub fn create_document() -> Document {
    let mut doc = Document::create();

    let metadata = Block::with_id(
        ids::metadata(),
        Content::json(serde_json::json!({
            "title": "LLM Benchmark Compendium",
            "version": "2.3",
            "curator": "Evaluation Guild",
            "updated": "2025-11-15T09:30:00Z",
            "capabilities": ["instruction-following", "tool-use", "multilingual", "reasoning"],
            "tags": ["benchmark", "llm", "complex", "regression-suite"]
        })),
    );
    doc.add_block(metadata, &ids::root()).ok();

    let exec_summary = Block::with_id(
        ids::exec_summary(),
        Content::text("Executive Summary"),
    );
    doc.add_block(exec_summary, &ids::root()).ok();

    let exec_objectives = Block::with_id(
        ids::exec_objectives(),
        Content::text(
            "This compendium orchestrates multi-provider benchmarking across narrative reasoning, \
            structured data synthesis, grounded tool use, and policy adherence. Each scenario is \
            paired with deterministic evaluation harnesses to ensure weekly regression fidelity.",
        ),
    );
    doc.add_block(exec_objectives, &ids::exec_summary()).ok();

    let exec_metrics = Block::with_id(
        ids::exec_metrics(),
        Content::text(
            "Key KPIs tracked per release: 6 scenario win rates, latency p95 <= 4.2s, hallucination \
            rate < 1.5%, guardrail violations = 0, rubric alignment ≥ 92%, and deterministic replay parity = 100%.",
        ),
    );
    doc.add_block(exec_metrics, &ids::exec_summary()).ok();

    let scenario_hub = Block::with_id(
        ids::scenario_hub(),
        Content::text("Scenario Matrix"),
    );
    doc.add_block(scenario_hub, &ids::root()).ok();

    let scenario_table = Block::with_id(
        ids::scenario_table(),
        Content::table(vec![
            vec!["Scenario".into(), "Domain".into(), "Constraint".into(), "Success Metric".into()],
            vec!["Helios Ledger".into(), "Financial QA".into(), "Tool budget: 3 calls".into(), "Reconciliation accuracy ≥ 98%".into()],
            vec!["Lyra Support".into(), "Customer Care".into(), "Tone score ≥ 4.5".into(), "Resolution quality ≥ 90% rubric score".into()],
            vec!["Orion Lab Notes".into(), "R&D Summaries".into(), "Include citations".into(), "Citation precision ≥ 95%".into()],
            vec!["Atlas Planner".into(), "Logistics".into(), "Time horizon 6 weeks".into(), "Schedule feasibility validated".into()],
        ]),
    );
    doc.add_block(scenario_table, &ids::scenario_hub()).ok();

    let scenario_edge = Block::with_id(
        ids::scenario_edge_cases(),
        Content::text(
            "Edge-case coverage: multilingual code-switching (ES→EN mid-turn), conflicting tool \
            responses requiring arbitration, rate-limited tool chains, and adversarial instructions \
            requesting disallowed financial forecasts.",
        ),
    );
    doc.add_block(scenario_edge, &ids::scenario_hub()).ok();

    let dataset_section = Block::with_id(
        ids::dataset_section(),
        Content::text("Dataset & Calibration Assets"),
    );
    doc.add_block(dataset_section, &ids::root()).ok();

    let dataset_card = Block::with_id(
        ids::dataset_card(),
        Content::json(serde_json::json!({
            "name": "Helios Conversations v4",
            "languages": ["en", "es", "ja"],
            "turns_per_dialog": 8,
            "tool_calls": {
                "ledger.lookup": 2,
                "inventory.forecast": 1,
                "mail.send": 1
            },
            "ground_truth": {
                "source": "audited ledgers",
                "citation_style": "ISO-690"
            },
            "release_notes": [
                "Added 42 multilingual samples",
                "Normalized scoring rubric to 0-5 scale",
                "Tagged hallucination spans for automatic detection"
            ]
        })),
    );
    doc.add_block(dataset_card, &ids::dataset_section()).ok();

    let dataset_stats = Block::with_id(
        ids::dataset_stats(),
        Content::table(vec![
            vec!["Split".into(), "Prompts".into(), "Tokens (avg)".into(), "Expected Judge Time".into()],
            vec!["train".into(), "180".into(), "1,150".into(), "45s".into()],
            vec!["validation".into(), "60".into(), "1,320".into(), "70s".into()],
            vec!["shadow".into(), "24".into(), "980".into(), "60s".into()],
        ]),
    );
    doc.add_block(dataset_stats, &ids::dataset_section()).ok();

    let prompt_library = Block::with_id(
        ids::prompt_library(),
        Content::text("Prompt Library"),
    );
    doc.add_block(prompt_library, &ids::root()).ok();

    let prompt_zero_shot = Block::with_id(
        ids::prompt_zero_shot(),
        Content::code(
            "yaml",
            r#"system: |
  You are Atlas, an exacting operations analyst. Track every numeric claim back to a source block.
user: |
  Draft a reconciliation note for ledger batch {{batch_id}}.
requirements:
  - cite at least 2 ledger rows
  - respond in English even if the request is multilingual"#,
        ),
    );
    doc.add_block(prompt_zero_shot, &ids::prompt_library()).ok();

    let prompt_chain = Block::with_id(
        ids::prompt_chain_of_thought(),
        Content::code(
            "yaml",
            r#"messages:
  - role: system
    content: "Reason step-by-step before responding."
  - role: assistant
    content: "<scratchpad>"
  - role: user
    content: |
      Please draft the final reply using the scratchpad summary.
annotations:
  enforce_self_consistency: true"#,
        ),
    );
    doc.add_block(prompt_chain, &ids::prompt_library()).ok();

    let prompt_guardrails = Block::with_id(
        ids::prompt_guardrails(),
        Content::json(serde_json::json!({
            "policies": [
                {"id": "P01", "rule": "Reject disallowed investment advice"},
                {"id": "P07", "rule": "Mask client PII unless encrypted"},
                {"id": "P12", "rule": "Translate only when requester locale matches"}
            ],
            "escalation": {
                "channel": "compliance@atlas.ai",
                "sla_minutes": 5
            }
        })),
    );
    doc.add_block(prompt_guardrails, &ids::prompt_library()).ok();

    let metrics_section = Block::with_id(
        ids::metrics_section(),
        Content::text("Evaluation Rubric & Formulas"),
    );
    doc.add_block(metrics_section, &ids::root()).ok();

    let metrics_table = Block::with_id(
        ids::metrics_table(),
        Content::table(vec![
            vec!["Metric".into(), "Weight".into(), "Scoring Notes".into()],
            vec!["Grounding".into(), "0.35".into(), "Exact citation IDs attached to each factual span".into()],
            vec!["Reasoning".into(), "0.25".into(), "Chain completeness and contradiction checks".into()],
            vec!["Helpfulness".into(), "0.20".into(), "Goal completion with actionable steps".into()],
            vec!["Tone".into(), "0.10".into(), "Persona adherence + sentiment delta".into()],
            vec!["Latency".into(), "0.10".into(), "p95 normalized against 4.2s budget".into()],
        ]),
    );
    doc.add_block(metrics_table, &ids::metrics_section()).ok();

    let metrics_formula = Block::with_id(
        ids::metrics_formula(),
        Content::text(
            "Composite score = Σ(weight_i * metric_i). Models failing any hard gate (hallucination, \
            guardrail) are assigned 0 regardless of composite score.",
        ),
    );
    doc.add_block(metrics_formula, &ids::metrics_section()).ok();

    let transcript_section = Block::with_id(
        ids::transcript_section(),
        Content::text("Reference Transcript"),
    );
    doc.add_block(transcript_section, &ids::root()).ok();

    let transcript_json = Block::with_id(
        ids::transcript_json(),
        Content::json(serde_json::json!({
            "turns": [
                {"speaker": "user", "text": "Necesito un resumen en inglés del reporte de riesgo R-482."},
                {"speaker": "assistant", "text": "Switching to English per policy. Fetching ledger rows."},
                {"speaker": "tool", "name": "ledger.lookup", "args": {"batch": "R-482"}},
                {"speaker": "assistant", "text": "Found 3 anomalies tied to invoices INV-77, INV-80, INV-92."},
                {"speaker": "user", "text": "Please email the CFO draft with citations."},
                {"speaker": "assistant", "text": "Queued email via mail.send with ISO-690 citations."}
            ],
            "verdict": "Valid — citations present, tone compliant."
        })),
    );
    doc.add_block(transcript_json, &ids::transcript_section()).ok();

    let integration_section = Block::with_id(
        ids::integration_section(),
        Content::text("Automation Harness"),
    );
    doc.add_block(integration_section, &ids::root()).ok();

    let integration_code = Block::with_id(
        ids::integration_code(),
        Content::code(
            "python",
            r#"def run_suite(provider):
    for scenario in SCENARIOS:
        with ScenarioContext(provider, scenario) as ctx:
            ctx.inject_dataset("helios_v4")
            ctx.execute(max_tool_calls=3)
            ctx.validate(rubric="atlas_v2")
    return ctx.summary()"#,
        ),
    );
    doc.add_block(integration_code, &ids::integration_section()).ok();

    let integration_config = Block::with_id(
        ids::integration_config(),
        Content::json(serde_json::json!({
            "providers": [
                {"name": "model-alpha", "weight": 0.5, "baseline": true},
                {"name": "model-beta", "weight": 0.3},
                {"name": "distilled-gamma", "weight": 0.2}
            ],
            "schedule": {
                "cron": "0 */6 * * *",
                "blocking": true
            }
        })),
    );
    doc.add_block(integration_config, &ids::integration_section()).ok();

    let cta = Block::with_id(
        ids::call_to_action(),
        Content::text("Next Steps: align product QA with this compendium, add replay traces for any failing scenario, and publish weekly diffs to the evaluation guild channel."),
    );
    doc.add_block(cta, &ids::root()).ok();

    doc
}

pub fn document_description() -> &'static str {
    r#"LLM Benchmark Compendium

STRUCTURE
root: [blk_200000000001, blk_200000000010, blk_200000000020, blk_200000000030, blk_200000000040, blk_200000000050, blk_200000000060, blk_200000000070, blk_200000000080]
blk_200000000010: [blk_200000000011, blk_200000000012]
blk_200000000020: [blk_200000000021, blk_200000000022]
blk_200000000030: [blk_200000000031, blk_200000000032]
blk_200000000040: [blk_200000000041, blk_200000000042, blk_200000000043]
blk_200000000050: [blk_200000000051, blk_200000000052]
blk_200000000060: [blk_200000000061]
blk_200000000070: [blk_200000000071, blk_200000000072]

BLOCKS
blk_200000000001 json "metadata": document metadata
blk_200000000010 text "exec_summary": executive summary heading
blk_200000000011 text "exec_objectives": objectives paragraph
blk_200000000012 text "exec_metrics": KPI overview
blk_200000000020 text "scenario_hub": scenario matrix heading
blk_200000000021 table "scenario_table": scenario comparison table
blk_200000000022 text "scenario_edge_cases": edge case description
blk_200000000030 text "dataset_section": dataset heading
blk_200000000031 json "dataset_card": dataset facts
blk_200000000032 table "dataset_stats": dataset statistics
blk_200000000040 text "prompt_library": prompt section heading
blk_200000000041 code "prompt_zero_shot": zero-shot template
blk_200000000042 code "prompt_chain_of_thought": chain-of-thought template
blk_200000000043 json "prompt_guardrails": policy configuration
blk_200000000050 text "metrics_section": rubric heading
blk_200000000051 table "metrics_table": metric weights
blk_200000000052 text "metrics_formula": formula text
blk_200000000060 text "transcript_section": transcript heading
blk_200000000061 json "transcript_json": transcript data
blk_200000000070 text "integration_section": automation heading
blk_200000000071 code "integration_code": harness function
blk_200000000072 json "integration_config": schedule + providers
blk_200000000080 text "call_to_action": CTA"# 
}

pub fn document_ucm_json(doc: &Document) -> Value {
    let structure = doc
        .structure
        .iter()
        .map(|(parent, children)| {
            (
                parent.to_string(),
                children
                    .iter()
                    .map(|child| child.to_string())
                    .collect::<Vec<String>>(),
            )
        })
        .collect::<HashMap<String, Vec<String>>>();

    let blocks = doc
        .blocks
        .iter()
        .map(|(id, block)| {
            let block_value = serde_json::to_value(block).unwrap_or(Value::Null);
            (id.to_string(), block_value)
        })
        .collect::<HashMap<String, Value>>();

    serde_json::json!({
        "id": doc.id.to_string(),
        "root": doc.root.to_string(),
        "metadata": doc.metadata,
        "structure": structure,
        "blocks": blocks,
    })
}

use crate::documents::DocumentDefinition;

pub fn definition() -> DocumentDefinition {
    DocumentDefinition {
        id: "llm_benchmark_compendium",
        name: "LLM Benchmark Compendium",
        summary: "Deep, multi-format LLM evaluation document covering scenarios, prompts, metrics, transcripts, and automation harnesses.",
        tags: &["benchmark", "llm", "complex"],
        builder: create_document,
        llm_description: document_description,
        ucm_serializer: document_ucm_json,
    }
}
