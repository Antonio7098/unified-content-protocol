//! Tree display command

use anyhow::Result;
use serde::Serialize;

use crate::cli::OutputFormat;
use crate::output::{print_tree, read_document};

/// Display document as a tree
pub fn tree(
    input: Option<String>,
    depth: Option<usize>,
    ids: bool,
    format: OutputFormat,
) -> Result<()> {
    let doc = read_document(input)?;

    match format {
        OutputFormat::Json => {
            // Build a JSON tree structure
            #[derive(Serialize)]
            struct TreeNode {
                id: String,
                label: Option<String>,
                role: Option<String>,
                content_type: String,
                children: Vec<TreeNode>,
            }

            fn build_tree(
                doc: &ucm_core::Document,
                block_id: &ucm_core::BlockId,
                current_depth: usize,
                max_depth: Option<usize>,
            ) -> Option<TreeNode> {
                if let Some(max) = max_depth {
                    if current_depth > max {
                        return None;
                    }
                }

                let block = doc.get_block(block_id)?;
                let children: Vec<TreeNode> = doc
                    .children(block_id)
                    .iter()
                    .filter_map(|cid| build_tree(doc, cid, current_depth + 1, max_depth))
                    .collect();

                Some(TreeNode {
                    id: block_id.to_string(),
                    label: block.metadata.label.clone(),
                    role: block.metadata.semantic_role.clone(),
                    content_type: block.content.type_tag().to_string(),
                    children,
                })
            }

            let tree = build_tree(&doc, &doc.root, 0, depth);
            println!("{}", serde_json::to_string_pretty(&tree)?);
        }
        OutputFormat::Text => {
            print_tree(&doc, depth, ids);
        }
    }

    Ok(())
}
