use crate::documents::DocumentDefinition;

pub fn definition() -> DocumentDefinition {
    DocumentDefinition {
        id: "ml_tutorial",
        name: "ML Tutorial",
        summary: "Comprehensive machine learning tutorial document with sections, code, tables, math, and references.",
        tags: &["tutorial", "machine-learning", "rich-content"],
        builder: crate::test_document::create_test_document,
        llm_description: crate::test_document::document_description,
        ucm_serializer: crate::test_document::document_ucm_json,
    }
}
