use tree_sitter::Node;

use crate::model::*;

use super::super::{is_python_package_init, make_extracted_symbol, node_text};
use super::ts_js::simple_symbol_reference_name;

pub(in crate::legacy) fn analyze_python_tree(
    path: &str,
    source: &str,
    root: Node<'_>,
    analysis: &mut FileAnalysis,
) {
    let mut cursor = root.walk();
    for node in root.named_children(&mut cursor) {
        analyze_python_node(source, node, analysis, &[], None);
    }

    apply_python_explicit_exports(analysis);

    if is_python_package_init(path) {
        for import in &mut analysis.imports {
            import.reexported = true;
        }
    }
}

pub(super) fn analyze_python_node(
    source: &str,
    node: Node<'_>,
    analysis: &mut FileAnalysis,
    scope: &[String],
    parent_identity: Option<&str>,
) {
    if let Some(source_identity) = parent_identity {
        if let Some((target_expr, target_name)) = python_call_target(node, source) {
            analysis.usages.push(ExtractedUsage::new(
                source_identity,
                target_expr,
                target_name,
            ));
        }
    }

    match node.kind() {
        "import_statement" => {
            analysis
                .imports
                .extend(python_imports_from_import_statement(node, source));
        }
        "import_from_statement" => {
            analysis
                .imports
                .extend(python_imports_from_from_statement(node, source));
        }
        "expression_statement" => {
            if scope.is_empty() {
                analysis
                    .exported_symbol_names
                    .extend(python_explicit_exports_from_statement(node, source));
            }
            if scope.is_empty() || parent_identity.is_some() {
                analysis.aliases.extend(python_aliases_from_statement(
                    node,
                    source,
                    parent_identity,
                ));
            }
        }
        _ => {}
    }

    let mut child_scope = scope.to_vec();
    let mut child_parent_identity = parent_identity.map(str::to_string);

    if let Some(symbol) = python_symbol_from_node(node, source, scope, parent_identity) {
        analysis
            .relationships
            .extend(python_symbol_relationships(node, source, &symbol));
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
        analyze_python_node(
            source,
            child,
            analysis,
            scope_ref,
            child_parent_identity.as_deref(),
        );
    }
}

pub(super) fn python_symbol_from_node(
    node: Node<'_>,
    source: &str,
    scope: &[String],
    parent_identity: Option<&str>,
) -> Option<ExtractedSymbol> {
    let kind = match node.kind() {
        "class_definition" => "class",
        "function_definition" => "function",
        "async_function_definition" => "function",
        _ => return None,
    };

    let name_node = node
        .child_by_field_name("name")
        .or_else(|| node.child_by_field_name("property"))?;
    let name = node_text(source, name_node).trim().to_string();
    if name.is_empty() {
        return None;
    }

    Some(make_extracted_symbol(
        name.clone(),
        kind,
        scope.is_empty() && !name.starts_with('_'),
        scope,
        parent_identity,
        CodeLanguage::Python,
        source,
        node,
    ))
}

pub(super) fn python_symbol_relationships(
    node: Node<'_>,
    source: &str,
    symbol: &ExtractedSymbol,
) -> Vec<ExtractedRelationship> {
    if node.kind() != "class_definition" {
        return Vec::new();
    }

    let Some(superclasses) = node.child_by_field_name("superclasses") else {
        return Vec::new();
    };

    let mut relationships = Vec::new();
    let mut cursor = superclasses.walk();
    for child in superclasses.named_children(&mut cursor) {
        if let Some((target_expr, target_name)) = python_class_base_reference(child, source) {
            relationships.push(ExtractedRelationship::new(
                symbol.identity.clone(),
                "extends",
                target_expr,
                target_name,
            ));
        }
    }

    relationships
}

pub(super) fn python_class_base_reference(
    node: Node<'_>,
    source: &str,
) -> Option<(String, String)> {
    if matches!(
        node.kind(),
        "keyword_argument" | "list_splat" | "dictionary_splat"
    ) {
        return None;
    }

    let text = node_text(source, node).trim();
    let name = simple_symbol_reference_name(text)?;
    Some((text.to_string(), name))
}

pub(super) fn python_call_target(node: Node<'_>, source: &str) -> Option<(String, String)> {
    if node.kind() != "call" {
        return None;
    }

    let function_node = node.child_by_field_name("function")?;
    match function_node.kind() {
        "identifier" => {
            let target_expr = node_text(source, function_node).trim().to_string();
            if target_expr.is_empty() {
                return None;
            }
            Some((target_expr.clone(), target_expr))
        }
        "attribute" => {
            let object = function_node.child_by_field_name("object")?;
            let attribute = function_node.child_by_field_name("attribute")?;
            if object.kind() != "identifier" {
                return None;
            }
            let object_name = node_text(source, object).trim().to_string();
            let attr_name = node_text(source, attribute).trim().to_string();
            if object_name.is_empty() || attr_name.is_empty() {
                return None;
            }
            Some((format!("{object_name}.{attr_name}"), attr_name))
        }
        _ => None,
    }
}

pub(super) fn python_imports_from_import_statement(
    node: Node<'_>,
    source: &str,
) -> Vec<ExtractedImport> {
    let mut imports = Vec::new();
    let mut cursor = node.walk();
    for child in node.named_children(&mut cursor) {
        match child.kind() {
            "aliased_import" => {
                let module_name = child
                    .child_by_field_name("name")
                    .map(|name_node| node_text(source, name_node).trim().to_string())
                    .unwrap_or_default();
                let alias = child
                    .child_by_field_name("alias")
                    .map(|alias_node| node_text(source, alias_node).trim().to_string())
                    .unwrap_or_default();
                if !module_name.is_empty() {
                    imports.push(ExtractedImport::module(module_name).with_module_alias(alias));
                }
            }
            "dotted_name" => {
                let module_name = node_text(source, child).trim().to_string();
                if module_name.is_empty() {
                    continue;
                }
                let mut import = ExtractedImport::module(module_name.clone());
                if !module_name.contains('.') {
                    import = import.with_module_alias(module_name);
                }
                imports.push(import);
            }
            _ => {}
        }
    }
    imports
}

pub(super) fn python_aliases_from_statement(
    node: Node<'_>,
    source: &str,
    owner_identity: Option<&str>,
) -> Vec<ExtractedAlias> {
    if node.kind() != "expression_statement" {
        return Vec::new();
    }

    let mut aliases = Vec::new();
    let mut cursor = node.walk();
    for child in node.named_children(&mut cursor) {
        if child.kind() != "assignment" {
            continue;
        }

        let Some(left) = child.child_by_field_name("left") else {
            continue;
        };
        let Some(right) = child.child_by_field_name("right") else {
            continue;
        };
        if left.kind() != "identifier" {
            continue;
        }

        let alias_name = node_text(source, left).trim().to_string();
        if alias_name.is_empty() {
            continue;
        }

        let (target_expr, target_name) =
            python_alias_reference(right, source).unwrap_or_else(|| (String::new(), String::new()));
        aliases.push(ExtractedAlias::new(
            alias_name,
            target_expr,
            target_name,
            owner_identity,
        ));
    }

    aliases
}

pub(super) fn python_alias_reference(node: Node<'_>, source: &str) -> Option<(String, String)> {
    match node.kind() {
        "identifier" => {
            let text = node_text(source, node).trim().to_string();
            if text.is_empty() {
                None
            } else {
                Some((text.clone(), text))
            }
        }
        "attribute" => {
            let object = node.child_by_field_name("object")?;
            let attribute = node.child_by_field_name("attribute")?;
            if object.kind() != "identifier" {
                return None;
            }
            let object_name = node_text(source, object).trim().to_string();
            let attr_name = node_text(source, attribute).trim().to_string();
            if object_name.is_empty() || attr_name.is_empty() {
                None
            } else {
                Some((format!("{object_name}.{attr_name}"), attr_name))
            }
        }
        _ => None,
    }
}

pub(super) fn python_imports_from_from_statement(
    node: Node<'_>,
    source: &str,
) -> Vec<ExtractedImport> {
    let module_name = node
        .child_by_field_name("module_name")
        .map(|module| node_text(source, module).trim().to_string())
        .unwrap_or_default();
    let imported_bindings = python_imported_bindings(node, source);
    let wildcard = python_has_wildcard_import(node);

    if module_name.is_empty() {
        return Vec::new();
    }

    if module_name.chars().all(|ch| ch == '.') {
        return imported_bindings
            .into_iter()
            .map(|binding| {
                let mut import =
                    ExtractedImport::module(format!("{}{}", module_name, binding.source_name));
                if !binding.local_name.is_empty() {
                    import = import.with_module_alias(binding.local_name);
                }
                import
            })
            .collect();
    }

    let mut import = ExtractedImport::bindings(module_name, imported_bindings);
    if wildcard {
        import = import.wildcard();
    }
    vec![import]
}

pub(super) fn python_imported_bindings(node: Node<'_>, source: &str) -> Vec<ImportBinding> {
    let module_name = node.child_by_field_name("module_name");
    let mut cursor = node.walk();
    let mut bindings = Vec::new();

    for child in node.named_children(&mut cursor) {
        if Some(child) == module_name || child.kind() == "wildcard_import" {
            continue;
        }

        let binding = match child.kind() {
            "aliased_import" => {
                let source_name = child
                    .child_by_field_name("name")
                    .map(|name_node| node_text(source, name_node).trim().to_string())
                    .unwrap_or_default();
                let local_name = child
                    .child_by_field_name("alias")
                    .map(|alias_node| node_text(source, alias_node).trim().to_string())
                    .unwrap_or_default();
                let source_name = source_name.rsplit('.').next().unwrap_or("").trim();
                if source_name.is_empty() || local_name.is_empty() {
                    None
                } else {
                    Some(ImportBinding::new(source_name, local_name))
                }
            }
            _ => {
                let source_name = node_text(source, child)
                    .trim()
                    .rsplit('.')
                    .next()
                    .unwrap_or("")
                    .trim();
                if source_name.is_empty() {
                    None
                } else {
                    Some(ImportBinding::same(source_name))
                }
            }
        };

        if let Some(binding) = binding {
            bindings.push(binding);
        }
    }

    bindings.sort();
    bindings.dedup();
    bindings
}

pub(super) fn python_has_wildcard_import(node: Node<'_>) -> bool {
    let mut cursor = node.walk();
    let has_wildcard = node
        .named_children(&mut cursor)
        .any(|child| child.kind() == "wildcard_import");
    has_wildcard
}

pub(super) fn python_explicit_exports_from_statement(node: Node<'_>, source: &str) -> Vec<String> {
    let mut cursor = node.walk();
    for child in node.named_children(&mut cursor) {
        if child.kind() != "assignment" {
            continue;
        }

        let Some(left) = child.child_by_field_name("left") else {
            continue;
        };
        if node_text(source, left).trim() != "__all__" {
            continue;
        }

        let Some(right) = child.child_by_field_name("right") else {
            continue;
        };
        return python_string_sequence_values(right, source);
    }

    Vec::new()
}

pub(super) fn python_string_sequence_values(node: Node<'_>, source: &str) -> Vec<String> {
    let mut values = Vec::new();
    let mut stack = vec![node];

    while let Some(current) = stack.pop() {
        if current.kind() == "string" {
            if let Some(value) = python_string_literal_value(node_text(source, current)) {
                values.push(value);
            }
            continue;
        }

        let mut cursor = current.walk();
        for child in current.named_children(&mut cursor) {
            stack.push(child);
        }
    }

    values
}

pub(super) fn python_string_literal_value(raw: &str) -> Option<String> {
    let trimmed = raw.trim();
    let quote_index = trimmed.find(['\'', '"'])?;
    let quote = trimmed[quote_index..].chars().next()?;
    let rest = &trimmed[quote_index + quote.len_utf8()..];
    let end_index = rest.rfind(quote)?;
    let value = rest[..end_index].trim();
    if value.is_empty() {
        None
    } else {
        Some(value.to_string())
    }
}

pub(super) fn apply_python_explicit_exports(analysis: &mut FileAnalysis) {
    if analysis.exported_symbol_names.is_empty() {
        return;
    }

    for symbol in analysis.symbols.iter_mut() {
        if symbol.parent_identity.is_none() {
            symbol.exported = analysis.exported_symbol_names.contains(&symbol.name);
        }
    }

    for import in analysis.imports.iter_mut() {
        if import.wildcard && !analysis.exported_symbol_names.is_empty() {
            import.symbols = analysis.exported_symbol_names.iter().cloned().collect();
            import.reexported = true;
        }

        if import
            .symbols
            .iter()
            .any(|symbol| analysis.exported_symbol_names.contains(symbol))
        {
            import.reexported = true;
        }
    }
}
