//! Test document generation for benchmarks.

use serde_json::Value;
use std::collections::HashMap;
use ucm_core::{Block, BlockId, Content, Document};

/// Block IDs for the test document (deterministic for reproducibility)
pub mod ids {
    use ucm_core::BlockId;

    pub fn root() -> BlockId {
        BlockId::root()
    }

    pub fn metadata() -> BlockId {
        BlockId::from_hex("000000000001").unwrap()
    }
    pub fn intro() -> BlockId {
        BlockId::from_hex("000000000010").unwrap()
    }
    pub fn intro_hook() -> BlockId {
        BlockId::from_hex("000000000011").unwrap()
    }
    pub fn intro_context() -> BlockId {
        BlockId::from_hex("000000000012").unwrap()
    }
    pub fn intro_thesis() -> BlockId {
        BlockId::from_hex("000000000013").unwrap()
    }
    pub fn body() -> BlockId {
        BlockId::from_hex("000000000020").unwrap()
    }
    pub fn section1() -> BlockId {
        BlockId::from_hex("000000000021").unwrap()
    }
    pub fn section1_heading() -> BlockId {
        BlockId::from_hex("000000000022").unwrap()
    }
    pub fn section1_para() -> BlockId {
        BlockId::from_hex("000000000023").unwrap()
    }
    pub fn section1_code() -> BlockId {
        BlockId::from_hex("000000000024").unwrap()
    }
    pub fn section1_table() -> BlockId {
        BlockId::from_hex("000000000025").unwrap()
    }
    pub fn section2() -> BlockId {
        BlockId::from_hex("000000000031").unwrap()
    }
    pub fn section2_heading() -> BlockId {
        BlockId::from_hex("000000000032").unwrap()
    }
    pub fn section2_math() -> BlockId {
        BlockId::from_hex("000000000033").unwrap()
    }
    pub fn section3() -> BlockId {
        BlockId::from_hex("000000000041").unwrap()
    }
    pub fn section3_heading() -> BlockId {
        BlockId::from_hex("000000000042").unwrap()
    }
    pub fn section3_list() -> BlockId {
        BlockId::from_hex("000000000043").unwrap()
    }
    pub fn conclusion() -> BlockId {
        BlockId::from_hex("000000000050").unwrap()
    }
    pub fn conclusion_summary() -> BlockId {
        BlockId::from_hex("000000000051").unwrap()
    }
    pub fn conclusion_cta() -> BlockId {
        BlockId::from_hex("000000000052").unwrap()
    }
    pub fn references() -> BlockId {
        BlockId::from_hex("000000000060").unwrap()
    }
}
/// Convert the canonical document into a JSON-friendly representation
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

/// Create a canonical test document with diverse content types.
pub fn create_test_document() -> Document {
    let mut doc = Document::create();

    // Metadata
    let metadata = Block::with_id(
        ids::metadata(),
        Content::json(serde_json::json!({
            "title": "Understanding Machine Learning",
            "author": "Test Author",
            "created": "2024-01-01T00:00:00Z",
            "version": "1.0",
            "tags": ["ml", "ai", "tutorial"]
        })),
    );
    doc.add_block(metadata, &ids::root()).ok();

    // Introduction section
    let intro = Block::with_id(ids::intro(), Content::text("Introduction"));
    doc.add_block(intro, &ids::root()).ok();

    let hook = Block::with_id(
        ids::intro_hook(),
        Content::text("Machine learning is transforming how we interact with technology."),
    );
    doc.add_block(hook, &ids::intro()).ok();

    let context = Block::with_id(
        ids::intro_context(),
        Content::text("From recommendation systems to autonomous vehicles, ML algorithms power countless applications we use daily."),
    );
    doc.add_block(context, &ids::intro()).ok();

    let thesis = Block::with_id(
        ids::intro_thesis(),
        Content::text("This document explores the fundamental concepts of machine learning and provides practical examples."),
    );
    doc.add_block(thesis, &ids::intro()).ok();

    // Body section
    let body = Block::with_id(ids::body(), Content::text("Body"));
    doc.add_block(body, &ids::root()).ok();

    // Section 1: Code example
    let section1 = Block::with_id(ids::section1(), Content::text("Section 1: Getting Started"));
    doc.add_block(section1, &ids::body()).ok();

    let s1_heading = Block::with_id(
        ids::section1_heading(),
        Content::text("Setting Up Your Environment"),
    );
    doc.add_block(s1_heading, &ids::section1()).ok();

    let s1_para = Block::with_id(
        ids::section1_para(),
        Content::text("Before diving into machine learning, you'll need to set up your development environment with the necessary tools and libraries."),
    );
    doc.add_block(s1_para, &ids::section1()).ok();

    let s1_code = Block::with_id(
        ids::section1_code(),
        Content::code(
            "python",
            r#"import numpy as np
import pandas as pd
from sklearn.model_selection import train_test_split
from sklearn.linear_model import LogisticRegression

# Load and prepare data
data = pd.read_csv('dataset.csv')
X = data.drop('target', axis=1)
y = data['target']

# Split into training and test sets
X_train, X_test, y_train, y_test = train_test_split(X, y, test_size=0.2)"#,
        ),
    );
    doc.add_block(s1_code, &ids::section1()).ok();

    let s1_table = Block::with_id(
        ids::section1_table(),
        Content::table(vec![
            vec!["Library".into(), "Version".into(), "Purpose".into()],
            vec![
                "numpy".into(),
                "1.24.0".into(),
                "Numerical computing".into(),
            ],
            vec!["pandas".into(), "2.0.0".into(), "Data manipulation".into()],
            vec![
                "scikit-learn".into(),
                "1.3.0".into(),
                "ML algorithms".into(),
            ],
        ]),
    );
    doc.add_block(s1_table, &ids::section1()).ok();

    // Section 2: Math content
    let section2 = Block::with_id(ids::section2(), Content::text("Section 2: The Mathematics"));
    doc.add_block(section2, &ids::body()).ok();

    let s2_heading = Block::with_id(
        ids::section2_heading(),
        Content::text("Linear Regression Fundamentals"),
    );
    doc.add_block(s2_heading, &ids::section2()).ok();

    let s2_math = Block::with_id(
        ids::section2_math(),
        Content::Text(ucm_core::Text {
            text: r"The cost function for linear regression is: J(\theta) = \frac{1}{2m} \sum_{i=1}^{m} (h_\theta(x^{(i)}) - y^{(i)})^2".into(),
            format: ucm_core::TextFormat::Plain,
        }),
    );
    doc.add_block(s2_math, &ids::section2()).ok();

    // Section 3: List content
    let section3 = Block::with_id(ids::section3(), Content::text("Section 3: Best Practices"));
    doc.add_block(section3, &ids::body()).ok();

    let s3_heading = Block::with_id(ids::section3_heading(), Content::text("Key Considerations"));
    doc.add_block(s3_heading, &ids::section3()).ok();

    let s3_list = Block::with_id(
        ids::section3_list(),
        Content::text("1. Always normalize your features\n2. Use cross-validation for model selection\n3. Monitor for overfitting\n4. Document your experiments\n5. Version your datasets"),
    );
    doc.add_block(s3_list, &ids::section3()).ok();

    // Conclusion
    let conclusion = Block::with_id(ids::conclusion(), Content::text("Conclusion"));
    doc.add_block(conclusion, &ids::root()).ok();

    let summary = Block::with_id(
        ids::conclusion_summary(),
        Content::text("Machine learning offers powerful tools for solving complex problems, but requires careful consideration of data quality, model selection, and validation strategies."),
    );
    doc.add_block(summary, &ids::conclusion()).ok();

    let cta = Block::with_id(
        ids::conclusion_cta(),
        Content::text("Start experimenting with the code examples provided and build your first ML model today!"),
    );
    doc.add_block(cta, &ids::conclusion()).ok();

    // References
    let refs = Block::with_id(
        ids::references(),
        Content::json(serde_json::json!({
            "references": [
                {"id": "ref1", "title": "Introduction to Statistical Learning", "authors": ["James", "Witten", "Hastie", "Tibshirani"]},
                {"id": "ref2", "title": "Deep Learning", "authors": ["Goodfellow", "Bengio", "Courville"]},
                {"id": "ref3", "title": "Pattern Recognition and Machine Learning", "authors": ["Bishop"]}
            ]
        })),
    );
    doc.add_block(refs, &ids::root()).ok();

    doc
}

/// Get a description of the test document structure for LLM context
/// Uses adjacency-list format for token efficiency per PROPOSAL.md
pub fn document_description() -> &'static str {
    r#"Machine Learning Tutorial Document

STRUCTURE
root: [blk_000000000001, blk_000000000010, blk_000000000020, blk_000000000050, blk_000000000060]
blk_000000000010: [blk_000000000011, blk_000000000012, blk_000000000013]
blk_000000000020: [blk_000000000021, blk_000000000031, blk_000000000041]
blk_000000000021: [blk_000000000022, blk_000000000023, blk_000000000024, blk_000000000025]
blk_000000000031: [blk_000000000032, blk_000000000033]
blk_000000000041: [blk_000000000042, blk_000000000043]
blk_000000000050: [blk_000000000051, blk_000000000052]

BLOCKS
blk_000000000001 json "metadata": title, author, created, version, tags
blk_000000000010 text "intro": Introduction section
blk_000000000011 text "intro_hook": Opening hook
blk_000000000012 text "intro_context": Context paragraph
blk_000000000013 text "intro_thesis": Thesis statement
blk_000000000020 text "body": Body section
blk_000000000021 text "section1": Getting Started
blk_000000000022 text "section1_heading": Section heading
blk_000000000023 text "section1_para": Intro paragraph
blk_000000000024 code "section1_code": Python example
blk_000000000025 table "section1_table": Library requirements
blk_000000000031 text "section2": The Mathematics
blk_000000000032 text "section2_heading": Section heading
blk_000000000033 math "section2_math": LaTeX formula
blk_000000000041 text "section3": Best Practices
blk_000000000042 text "section3_heading": Section heading
blk_000000000043 text "section3_list": Numbered list
blk_000000000050 text "conclusion": Conclusion section
blk_000000000051 text "conclusion_summary": Summary paragraph
blk_000000000052 text "conclusion_cta": Call to action
blk_000000000060 json "references": Bibliography"#
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_document() {
        let doc = create_test_document();
        assert!(doc.block_count() > 10);
        assert!(doc.get_block(&ids::metadata()).is_some());
        assert!(doc.get_block(&ids::section1_code()).is_some());
    }

    #[test]
    fn test_deterministic_ids() {
        let id1 = ids::intro_hook();
        let id2 = ids::intro_hook();
        assert_eq!(id1, id2);
    }
}
