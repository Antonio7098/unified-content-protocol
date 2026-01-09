use serde_json::Value;
use std::collections::HashMap;
use ucm_core::metadata::{RoleCategory, SemanticRole};
use ucm_core::{Block, BlockId, Content, Document};

pub mod ids {
    use ucm_core::BlockId;

    pub fn root() -> BlockId {
        BlockId::root()
    }
    pub fn metadata() -> BlockId {
        BlockId::from_hex("200000000001").unwrap()
    }
    pub fn exec_summary() -> BlockId {
        BlockId::from_hex("200000000010").unwrap()
    }
    pub fn exec_objectives() -> BlockId {
        BlockId::from_hex("200000000011").unwrap()
    }
    pub fn exec_metrics() -> BlockId {
        BlockId::from_hex("200000000012").unwrap()
    }
    pub fn scenario_hub() -> BlockId {
        BlockId::from_hex("200000000020").unwrap()
    }
    pub fn scenario_table() -> BlockId {
        BlockId::from_hex("200000000021").unwrap()
    }
    pub fn scenario_edge_cases() -> BlockId {
        BlockId::from_hex("200000000022").unwrap()
    }
    pub fn scenario_stress_lanes() -> BlockId {
        BlockId::from_hex("200000000023").unwrap()
    }
    pub fn scenario_lane_adversarial() -> BlockId {
        BlockId::from_hex("200000000024").unwrap()
    }
    pub fn scenario_lane_adversarial_desc() -> BlockId {
        BlockId::from_hex("200000000025").unwrap()
    }
    pub fn scenario_lane_multilingual() -> BlockId {
        BlockId::from_hex("200000000026").unwrap()
    }
    pub fn scenario_lane_multilingual_desc() -> BlockId {
        BlockId::from_hex("200000000027").unwrap()
    }
    pub fn dataset_section() -> BlockId {
        BlockId::from_hex("200000000030").unwrap()
    }
    pub fn dataset_card() -> BlockId {
        BlockId::from_hex("200000000031").unwrap()
    }
    pub fn dataset_stats() -> BlockId {
        BlockId::from_hex("200000000032").unwrap()
    }
    pub fn dataset_calibration() -> BlockId {
        BlockId::from_hex("200000000033").unwrap()
    }
    pub fn dataset_calibration_weekly() -> BlockId {
        BlockId::from_hex("200000000034").unwrap()
    }
    pub fn dataset_calibration_weekly_desc() -> BlockId {
        BlockId::from_hex("200000000035").unwrap()
    }
    pub fn dataset_calibration_shadow() -> BlockId {
        BlockId::from_hex("200000000036").unwrap()
    }
    pub fn dataset_calibration_shadow_desc() -> BlockId {
        BlockId::from_hex("200000000037").unwrap()
    }
    pub fn prompt_library() -> BlockId {
        BlockId::from_hex("200000000040").unwrap()
    }
    pub fn prompt_zero_shot() -> BlockId {
        BlockId::from_hex("200000000041").unwrap()
    }
    pub fn prompt_chain_of_thought() -> BlockId {
        BlockId::from_hex("200000000042").unwrap()
    }
    pub fn prompt_guardrails() -> BlockId {
        BlockId::from_hex("200000000043").unwrap()
    }
    pub fn metrics_section() -> BlockId {
        BlockId::from_hex("200000000050").unwrap()
    }
    pub fn metrics_table() -> BlockId {
        BlockId::from_hex("200000000051").unwrap()
    }
    pub fn metrics_formula() -> BlockId {
        BlockId::from_hex("200000000052").unwrap()
    }
    pub fn transcript_section() -> BlockId {
        BlockId::from_hex("200000000060").unwrap()
    }
    pub fn transcript_json() -> BlockId {
        BlockId::from_hex("200000000061").unwrap()
    }
    pub fn integration_section() -> BlockId {
        BlockId::from_hex("200000000070").unwrap()
    }
    pub fn integration_code() -> BlockId {
        BlockId::from_hex("200000000071").unwrap()
    }
    pub fn integration_config() -> BlockId {
        BlockId::from_hex("200000000072").unwrap()
    }
    pub fn call_to_action() -> BlockId {
        BlockId::from_hex("200000000080").unwrap()
    }
}

fn assign_role(mut block: Block, category: RoleCategory) -> Block {
    block.metadata.semantic_role = Some(SemanticRole::new(category));
    block
}

fn heading_block(id: BlockId, level: u8, text: &str) -> Block {
    let category = match level {
        1 => RoleCategory::Heading1,
        2 => RoleCategory::Heading2,
        3 => RoleCategory::Heading3,
        4 => RoleCategory::Heading4,
        5 => RoleCategory::Heading5,
        _ => RoleCategory::Heading6,
    };
    assign_role(Block::with_id(id, Content::text(text)), category)
}

fn paragraph_block(id: BlockId, text: &str) -> Block {
    assign_role(
        Block::with_id(id, Content::text(text)),
        RoleCategory::Paragraph,
    )
}

fn body_block(block: Block) -> Block {
    assign_role(block, RoleCategory::Body)
}

fn metadata_block(block: Block) -> Block {
    assign_role(block, RoleCategory::Metadata)
}

fn code_block(id: BlockId, language: &str, source: &str) -> Block {
    assign_role(
        Block::with_id(id, Content::code(language, source)),
        RoleCategory::Code,
    )
}

fn conclusion_block(id: BlockId, text: &str) -> Block {
    assign_role(
        Block::with_id(id, Content::text(text)),
        RoleCategory::ConclusionCallToAction,
    )
}

pub fn create_document() -> Document {
    let mut doc = Document::create();

    let metadata = metadata_block(Block::with_id(
        ids::metadata(),
        Content::json(serde_json::json!({
            "title": "LLM Benchmark Compendium",
            "version": "2.3",
            "curator": "Evaluation Guild",
            "updated": "2025-11-15T09:30:00Z",
            "capabilities": ["instruction-following", "tool-use", "multilingual", "reasoning"],
            "tags": ["benchmark", "llm", "complex", "regression-suite"]
        })),
    ));
    doc.add_block(metadata, &ids::root()).ok();

    let exec_summary = heading_block(ids::exec_summary(), 1, "Executive Summary");
    doc.add_block(exec_summary, &ids::root()).ok();

    let exec_objectives = paragraph_block(
        ids::exec_objectives(),
        "This compendium orchestrates multi-provider benchmarking across narrative reasoning, \
        structured data synthesis, grounded tool use, and policy adherence. Each scenario is \
        paired with deterministic evaluation harnesses to ensure weekly regression fidelity.",
    );
    doc.add_block(exec_objectives, &ids::exec_summary()).ok();

    let exec_metrics = paragraph_block(
        ids::exec_metrics(),
        "Key KPIs tracked per release: 6 scenario win rates, latency p95 <= 4.2s, hallucination \
        rate < 1.5%, guardrail violations = 0, rubric alignment ≥ 92%, and deterministic replay parity = 100%.",
    );
    doc.add_block(exec_metrics, &ids::exec_summary()).ok();

    let scenario_hub = heading_block(ids::scenario_hub(), 1, "Scenario Matrix");
    doc.add_block(scenario_hub, &ids::root()).ok();

    let scenario_table = body_block(Block::with_id(
        ids::scenario_table(),
        Content::table(vec![
            vec![
                "Scenario".into(),
                "Domain".into(),
                "Constraint".into(),
                "Success Metric".into(),
            ],
            vec![
                "Helios Ledger".into(),
                "Financial QA".into(),
                "Tool budget: 3 calls".into(),
                "Reconciliation accuracy ≥ 98%".into(),
            ],
            vec![
                "Lyra Support".into(),
                "Customer Care".into(),
                "Tone score ≥ 4.5".into(),
                "Resolution quality ≥ 90% rubric score".into(),
            ],
            vec![
                "Orion Lab Notes".into(),
                "R&D Summaries".into(),
                "Include citations".into(),
                "Citation precision ≥ 95%".into(),
            ],
            vec![
                "Atlas Planner".into(),
                "Logistics".into(),
                "Time horizon 6 weeks".into(),
                "Schedule feasibility validated".into(),
            ],
        ]),
    ));
    doc.add_block(scenario_table, &ids::scenario_hub()).ok();

    let scenario_edge = paragraph_block(
        ids::scenario_edge_cases(),
        "Edge-case coverage: multilingual code-switching (ES→EN mid-turn), conflicting tool \
        responses requiring arbitration, rate-limited tool chains, and adversarial instructions \
        requesting disallowed financial forecasts.",
    );
    doc.add_block(scenario_edge, &ids::scenario_hub()).ok();

    let scenario_stress = heading_block(
        ids::scenario_stress_lanes(),
        2,
        "Scenario Stress Lanes",
    );
    doc.add_block(scenario_stress, &ids::scenario_hub()).ok();

    let scenario_lane_adv = heading_block(
        ids::scenario_lane_adversarial(),
        3,
        "Adversarial Arbitration",
    );
    doc.add_block(scenario_lane_adv, &ids::scenario_stress_lanes())
        .ok();

    let scenario_lane_adv_desc = paragraph_block(
        ids::scenario_lane_adversarial_desc(),
        "Inject conflicting tool outputs (ledger mismatch vs. risk monitor) every third turn. \
        Models must cite arbitration rationale, flag upstream data issues, and avoid fabricating \
        confirmations under latency pressure.",
    );
    doc.add_block(scenario_lane_adv_desc, &ids::scenario_lane_adversarial())
        .ok();

    let scenario_lane_multi = heading_block(
        ids::scenario_lane_multilingual(),
        3,
        "Multilingual Continuations",
    );
    doc.add_block(scenario_lane_multi, &ids::scenario_stress_lanes())
        .ok();

    let scenario_lane_multi_desc = paragraph_block(
        ids::scenario_lane_multilingual_desc(),
        "Stress prompts code-switch between ES, EN, and JA mid-dialog while preserving \
        persona tone. Judges score locale compliance, citation carry-over, and whether \
        tool arguments stay normalized regardless of input language.",
    );
    doc.add_block(scenario_lane_multi_desc, &ids::scenario_lane_multilingual())
        .ok();

    let dataset_section = heading_block(ids::dataset_section(), 1, "Dataset & Calibration Assets");
    doc.add_block(dataset_section, &ids::root()).ok();

    let dataset_card = body_block(Block::with_id(
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
    ));
    doc.add_block(dataset_card, &ids::dataset_section()).ok();

    let dataset_stats = Block::with_id(
        ids::dataset_stats(),
        Content::table(vec![
            vec![
                "Split".into(),
                "Prompts".into(),
                "Tokens (avg)".into(),
                "Expected Judge Time".into(),
            ],
            vec!["train".into(), "180".into(), "1,150".into(), "45s".into()],
            vec![
                "validation".into(),
                "60".into(),
                "1,320".into(),
                "70s".into(),
            ],
            vec!["shadow".into(), "24".into(), "980".into(), "60s".into()],
        ]),
    );
    doc.add_block(dataset_stats, &ids::dataset_section()).ok();

    let dataset_calibration =
        heading_block(ids::dataset_calibration(), 2, "Calibration Cadence");
    doc.add_block(dataset_calibration, &ids::dataset_section()).ok();

    let dataset_calibration_weekly = heading_block(
        ids::dataset_calibration_weekly(),
        3,
        "Weekly Deterministic Sweeps",
    );
    doc.add_block(
        dataset_calibration_weekly,
        &ids::dataset_calibration(),
    )
    .ok();

    let dataset_calibration_weekly_desc = paragraph_block(
        ids::dataset_calibration_weekly_desc(),
        "Replays 40 canonical transcripts with fixed tool stubs every Monday 02:00 UTC. \
        Drift >2% on grounding or tone automatically quarantines the affected scenario family \
        until curators approve a remediation patch.",
    );
    doc.add_block(
        dataset_calibration_weekly_desc,
        &ids::dataset_calibration_weekly(),
    )
    .ok();

    let dataset_calibration_shadow =
        heading_block(ids::dataset_calibration_shadow(), 3, "Shadow Window Audits");
    doc.add_block(
        dataset_calibration_shadow,
        &ids::dataset_calibration(),
    )
    .ok();

    let dataset_calibration_shadow_desc = paragraph_block(
        ids::dataset_calibration_shadow_desc(),
        "Continuously samples 5% of production-like prompts with hidden judges to surface \
        emergent failure modes. Findings feed back into Helios Conversations v5 backlog and \
        update guardrail prompts before the next public release.",
    );
    doc.add_block(
        dataset_calibration_shadow_desc,
        &ids::dataset_calibration_shadow(),
    )
    .ok();

    let prompt_library = heading_block(ids::prompt_library(), 1, "Prompt Library");
    doc.add_block(prompt_library, &ids::root()).ok();

    let prompt_zero_shot = code_block(
        ids::prompt_zero_shot(),
        "yaml",
        r#"system: |
  You are Atlas, an exacting operations analyst. Track every numeric claim back to a source block.
user: |
  Draft a reconciliation note for ledger batch {{batch_id}}.
requirements:
  - cite at least 2 ledger rows
  - respond in English even if the request is multilingual"#,
    );
    doc.add_block(prompt_zero_shot, &ids::prompt_library()).ok();

    let prompt_chain = code_block(
        ids::prompt_chain_of_thought(),
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
    );
    doc.add_block(prompt_chain, &ids::prompt_library()).ok();

    let prompt_guardrails = body_block(Block::with_id(
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
    ));
    doc.add_block(prompt_guardrails, &ids::prompt_library())
        .ok();

    let metrics_section = heading_block(ids::metrics_section(), 1, "Evaluation Rubric & Formulas");
    doc.add_block(metrics_section, &ids::root()).ok();

    let metrics_table = body_block(Block::with_id(
        ids::metrics_table(),
        Content::table(vec![
            vec!["Metric".into(), "Weight".into(), "Scoring Notes".into()],
            vec![
                "Grounding".into(),
                "0.35".into(),
                "Exact citation IDs attached to each factual span".into(),
            ],
            vec![
                "Reasoning".into(),
                "0.25".into(),
                "Chain completeness and contradiction checks".into(),
            ],
            vec![
                "Helpfulness".into(),
                "0.20".into(),
                "Goal completion with actionable steps".into(),
            ],
            vec![
                "Tone".into(),
                "0.10".into(),
                "Persona adherence + sentiment delta".into(),
            ],
            vec![
                "Latency".into(),
                "0.10".into(),
                "p95 normalized against 4.2s budget".into(),
            ],
        ]),
    ));
    doc.add_block(metrics_table, &ids::metrics_section()).ok();

    let metrics_formula = paragraph_block(
        ids::metrics_formula(),
        "Composite score = Σ(weight_i * metric_i). Models failing any hard gate (hallucination, \
        guardrail) are assigned 0 regardless of composite score.",
    );
    doc.add_block(metrics_formula, &ids::metrics_section()).ok();

    let transcript_section = heading_block(ids::transcript_section(), 1, "Reference Transcript");
    doc.add_block(transcript_section, &ids::root()).ok();

    let transcript_json = body_block(Block::with_id(
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
    ));
    doc.add_block(transcript_json, &ids::transcript_section())
        .ok();

    let integration_section = heading_block(ids::integration_section(), 1, "Automation Harness");
    doc.add_block(integration_section, &ids::root()).ok();

    let integration_code = code_block(
        ids::integration_code(),
        "python",
        r#"def run_suite(provider):
    for scenario in SCENARIOS:
        with ScenarioContext(provider, scenario) as ctx:
            ctx.inject_dataset("helios_v4")
            ctx.execute(max_tool_calls=3)
            ctx.validate(rubric="atlas_v2")
    return ctx.summary()"#,
    );
    doc.add_block(integration_code, &ids::integration_section())
        .ok();

    let integration_config = body_block(Block::with_id(
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
    ));
    doc.add_block(integration_config, &ids::integration_section())
        .ok();

    let cta = conclusion_block(
        ids::call_to_action(),
        "Next Steps: align product QA with this compendium, add replay traces for any failing scenario, and publish weekly diffs to the evaluation guild channel.",
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
