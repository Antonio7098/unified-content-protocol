use tree_sitter::Node;

use crate::model::*;

use super::super::{
    expand_rust_use_declaration_items, first_named_identifier, make_extracted_symbol, node_text,
};

pub(in crate::legacy) fn analyze_rust_tree(
    source: &str,
    root: Node<'_>,
    analysis: &mut FileAnalysis,
) {
    let mut cursor = root.walk();
    for node in root.named_children(&mut cursor) {
        analyze_rust_node(source, node, analysis, &[], None);
    }
}

pub(super) fn analyze_rust_node(
    source: &str,
    node: Node<'_>,
    analysis: &mut FileAnalysis,
    scope: &[String],
    parent_identity: Option<&str>,
) {
    if let Some(source_identity) = parent_identity {
        if let Some((target_expr, target_name)) = rust_call_target(node, source) {
            analysis.usages.push(ExtractedUsage::new(
                source_identity,
                target_expr,
                target_name,
            ));
        }
    }

    if node.kind() == "use_declaration" {
        let use_text = node_text(source, node);
        let reexported = use_text.trim_start().starts_with("pub use ");
        for (module, local_alias, wildcard) in expand_rust_use_declaration_items(use_text) {
            let mut import = if wildcard {
                ExtractedImport::module(module)
            } else if let Some(local_name) = local_alias {
                if let Some(source_name) = rust_imported_symbol_name(&module) {
                    ExtractedImport::bindings(
                        module,
                        vec![ImportBinding::new(source_name, local_name.clone())],
                    )
                    .with_module_alias(local_name)
                } else {
                    ExtractedImport::module(module).with_module_alias(local_name)
                }
            } else if let Some(symbol) = rust_imported_symbol_name(&module) {
                ExtractedImport::symbol(module, symbol)
            } else {
                ExtractedImport::module(module)
            };
            if reexported {
                import = import.reexported();
            }
            if wildcard {
                import = import.wildcard();
            }
            analysis.imports.push(import);
        }
    }

    if node.kind() == "let_declaration" {
        if let Some(source_identity) = parent_identity {
            analysis.aliases.extend(rust_aliases_from_let_declaration(
                node,
                source,
                source_identity,
            ));
        }
    }

    if node.kind() == "mod_item" {
        let text = node_text(source, node);
        if text.trim().ends_with(';') {
            if let Some(name) = rust_symbol_name(node, source) {
                analysis
                    .imports
                    .push(ExtractedImport::module(format!("mod:{}", name)));
            }
        }
    }

    let mut child_scope = scope.to_vec();
    let mut child_parent_identity = parent_identity.map(str::to_string);

    if let Some(symbol) = rust_symbol_from_node(node, source, scope, parent_identity) {
        analysis
            .relationships
            .extend(rust_symbol_relationships(node, source, &symbol));
        child_scope.push(symbol.name.clone());
        child_parent_identity = Some(symbol.identity.clone());
        analysis.symbols.push(symbol);
    }

    let scope_ref = if child_scope.len() == scope.len() {
        scope
    } else {
        &child_scope
    };

    let mut cursor = node.walk();
    for child in node.named_children(&mut cursor) {
        analyze_rust_node(
            source,
            child,
            analysis,
            scope_ref,
            child_parent_identity.as_deref(),
        );
    }
}

pub(super) fn rust_symbol_from_node(
    node: Node<'_>,
    source: &str,
    scope: &[String],
    parent_identity: Option<&str>,
) -> Option<ExtractedSymbol> {
    let kind = match node.kind() {
        "function_item" => "function",
        "struct_item" => "struct",
        "enum_item" => "enum",
        "trait_item" => "trait",
        "impl_item" => "impl",
        "type_item" => "type",
        "const_item" => "const",
        "mod_item" => "module",
        _ => return None,
    };

    let name = rust_symbol_name(node, source)?;
    let exported = scope.is_empty() && node_text(source, node).trim_start().starts_with("pub");

    Some(make_extracted_symbol(
        name,
        kind,
        exported,
        scope,
        parent_identity,
        CodeLanguage::Rust,
        source,
        node,
    ))
}

pub(super) fn rust_symbol_name(node: Node<'_>, source: &str) -> Option<String> {
    if let Some(name_node) = node.child_by_field_name("name") {
        let name = node_text(source, name_node).trim().to_string();
        if !name.is_empty() {
            return Some(name);
        }
    }

    if node.kind() == "impl_item" {
        if let Some(type_node) = node.child_by_field_name("type") {
            let name = node_text(source, type_node).trim().to_string();
            if !name.is_empty() {
                return Some(name);
            }
        }
    }

    first_named_identifier(node, source)
}

pub(super) fn rust_imported_symbol_name(module: &str) -> Option<String> {
    if module.starts_with("mod:") {
        return None;
    }

    let stripped = module
        .trim_start_matches("crate::")
        .trim_start_matches("self::")
        .trim_start_matches("super::");
    let segments: Vec<&str> = stripped
        .split("::")
        .filter(|segment| !segment.is_empty())
        .collect();

    if module.starts_with("crate::") && segments.len() == 1 {
        return segments.first().map(|segment| (*segment).to_string());
    }

    if segments.len() >= 2 {
        segments.last().map(|segment| (*segment).to_string())
    } else {
        None
    }
}

pub(super) fn rust_symbol_relationships(
    node: Node<'_>,
    source: &str,
    symbol: &ExtractedSymbol,
) -> Vec<ExtractedRelationship> {
    let mut relationships = Vec::new();

    match node.kind() {
        "impl_item" => {
            if let Some(type_node) = node.child_by_field_name("type") {
                if let Some((target_expr, target_name)) = rust_type_reference(type_node, source) {
                    relationships.push(ExtractedRelationship::new(
                        symbol.identity.clone(),
                        "for_type",
                        target_expr,
                        target_name,
                    ));
                }
            }

            if let Some(trait_node) = node.child_by_field_name("trait") {
                if let Some((target_expr, target_name)) = rust_type_reference(trait_node, source) {
                    relationships.push(ExtractedRelationship::new(
                        symbol.identity.clone(),
                        "implements",
                        target_expr,
                        target_name,
                    ));
                }
            }
        }
        "trait_item" => {
            if let Some(bounds_node) = node.child_by_field_name("bounds") {
                let mut cursor = bounds_node.walk();
                for bound in bounds_node.named_children(&mut cursor) {
                    if let Some((target_expr, target_name)) = rust_type_reference(bound, source) {
                        relationships.push(ExtractedRelationship::new(
                            symbol.identity.clone(),
                            "extends",
                            target_expr,
                            target_name,
                        ));
                    }
                }
            }
        }
        _ => {}
    }

    relationships
}

pub(super) fn rust_type_reference(node: Node<'_>, source: &str) -> Option<(String, String)> {
    let raw = node_text(source, node).trim();
    let trimmed = raw.trim_start_matches('&').trim_start();
    let trimmed = trimmed.strip_prefix("mut ").unwrap_or(trimmed).trim();
    let core = trimmed.split('<').next().unwrap_or(trimmed).trim();
    let name = rust_last_path_segment(core)?;
    Some((core.to_string(), name))
}

pub(super) fn rust_call_target(node: Node<'_>, source: &str) -> Option<(String, String)> {
    if node.kind() != "call_expression" {
        return None;
    }

    let function_node = node.child_by_field_name("function")?;
    rust_callable_reference(function_node, source)
}

pub(super) fn rust_aliases_from_let_declaration(
    node: Node<'_>,
    source: &str,
    owner_identity: &str,
) -> Vec<ExtractedAlias> {
    let Some(pattern_node) = node.child_by_field_name("pattern") else {
        return Vec::new();
    };
    if pattern_node.kind() != "identifier" {
        return Vec::new();
    }

    let alias_name = node_text(source, pattern_node).trim().to_string();
    if alias_name.is_empty() {
        return Vec::new();
    }

    let (target_expr, target_name) = node
        .child_by_field_name("value")
        .and_then(|value_node| rust_callable_reference(value_node, source))
        .unwrap_or_else(|| (String::new(), String::new()));

    vec![ExtractedAlias::new(
        alias_name,
        target_expr,
        target_name,
        Some(owner_identity),
    )]
}

pub(super) fn rust_callable_reference(node: Node<'_>, source: &str) -> Option<(String, String)> {
    let raw = match node.kind() {
        "identifier" | "scoped_identifier" => node_text(source, node).trim().to_string(),
        "generic_function" => node_text(source, node).trim().to_string(),
        _ => return None,
    };
    let core = raw.split('<').next().unwrap_or(raw.as_str()).trim();
    let name = rust_last_path_segment(core)?;
    Some((core.to_string(), name))
}

pub(in crate::legacy) fn rust_last_path_segment(path: &str) -> Option<String> {
    let segment = path.rsplit("::").next().unwrap_or(path).trim();
    if segment.is_empty() {
        None
    } else {
        Some(segment.to_string())
    }
}
