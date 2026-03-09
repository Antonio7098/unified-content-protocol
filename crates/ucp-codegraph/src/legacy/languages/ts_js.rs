use std::collections::BTreeSet;

use regex::Regex;
use tree_sitter::Node;
use ucm_core::BlockId;

use crate::model::*;

use super::super::{
    first_named_identifier, make_extracted_symbol, node_text, split_top_level_commas,
};

pub(in crate::legacy) fn analyze_ts_tree(
    source: &str,
    root: Node<'_>,
    analysis: &mut FileAnalysis,
) {
    let mut cursor = root.walk();
    for node in root.named_children(&mut cursor) {
        analyze_ts_node(source, node, analysis, &[], None, false);
    }

    mark_exported_symbols(&mut analysis.symbols, &analysis.exported_symbol_names);
}

pub(super) fn analyze_ts_node(
    source: &str,
    node: Node<'_>,
    analysis: &mut FileAnalysis,
    scope: &[String],
    parent_identity: Option<&str>,
    exported_context: bool,
) {
    if let Some(source_identity) = parent_identity {
        if let Some((target_expr, target_name)) = ts_call_target(node, source) {
            analysis.usages.push(ExtractedUsage::new(
                source_identity,
                target_expr,
                target_name,
            ));
        }
    }

    match node.kind() {
        "import_statement" => {
            if let Some(module) = extract_ts_module_from_text(node_text(source, node)) {
                let imported_bindings = ts_import_bindings(node, source);
                let mut import = ExtractedImport::bindings(module, imported_bindings);
                for alias in ts_namespace_import_aliases(node, source) {
                    import = import.with_module_alias(alias);
                }
                analysis.imports.push(import);
            }
        }
        "export_statement" => {
            if let Some(module) = extract_ts_module_from_text(node_text(source, node)) {
                let export_bindings = ts_reexport_bindings(node, source);
                let mut import = ExtractedImport::bindings(module, export_bindings).reexported();
                if ts_is_wildcard_reexport(node, source) {
                    import = import.wildcard();
                }
                analysis.imports.push(import);
            }
            collect_ts_local_export_names(node, source, &mut analysis.exported_symbol_names);
            analysis
                .default_exported_symbol_names
                .extend(ts_default_export_names_from_text(node_text(source, node)));

            let mut cursor = node.walk();
            for child in node.named_children(&mut cursor) {
                analyze_ts_node(source, child, analysis, scope, parent_identity, true);
            }
            return;
        }
        "lexical_declaration" | "variable_statement" => {
            analysis
                .imports
                .extend(ts_require_imports_from_variable_statement(node, source));
            analysis.symbols.extend(ts_variable_symbols(
                node,
                source,
                exported_context,
                scope,
                parent_identity,
            ));
            if scope.is_empty() || parent_identity.is_some() {
                analysis.aliases.extend(ts_aliases_from_variable_statement(
                    node,
                    source,
                    parent_identity,
                ));
            }
            return;
        }
        "expression_statement" => {
            if scope.is_empty() && parent_identity.is_none() {
                collect_ts_commonjs_exports(node, source, analysis);
            }
        }
        _ => {}
    }

    let mut child_scope = scope.to_vec();
    let mut child_parent_identity = parent_identity.map(str::to_string);

    if let Some(symbol) =
        ts_symbol_from_declaration(node, source, exported_context, scope, parent_identity)
    {
        analysis
            .relationships
            .extend(ts_symbol_relationships(node, source, &symbol));
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
        analyze_ts_node(
            source,
            child,
            analysis,
            scope_ref,
            child_parent_identity.as_deref(),
            false,
        );
    }
}

pub(super) fn ts_symbol_from_declaration(
    node: Node<'_>,
    source: &str,
    exported_hint: bool,
    scope: &[String],
    parent_identity: Option<&str>,
) -> Option<ExtractedSymbol> {
    let kind = match node.kind() {
        "function_declaration" => "function",
        "generator_function_declaration" => "function",
        "class_declaration" => "class",
        "interface_declaration" => "interface",
        "type_alias_declaration" => "type",
        "enum_declaration" => "enum",
        "module" => "namespace",
        "method_definition" => "method",
        "public_field_definition" => ts_public_field_kind(node),
        "field_definition" => ts_public_field_kind(node),
        _ => return None,
    };

    let name = node
        .child_by_field_name("name")
        .or_else(|| node.child_by_field_name("property"))
        .map(|n| node_text(source, n).trim().to_string())
        .or_else(|| first_named_identifier(node, source))?;
    if name.is_empty() {
        return None;
    }
    let exported = scope.is_empty()
        && (exported_hint || node_text(source, node).trim_start().starts_with("export "));

    Some(make_extracted_symbol(
        name,
        kind,
        exported,
        scope,
        parent_identity,
        CodeLanguage::TypeScript,
        source,
        node,
    ))
}

pub(super) fn ts_variable_symbols(
    node: Node<'_>,
    source: &str,
    exported_hint: bool,
    scope: &[String],
    parent_identity: Option<&str>,
) -> Vec<ExtractedSymbol> {
    let mut out = Vec::new();
    let exported = scope.is_empty()
        && (exported_hint || node_text(source, node).trim_start().starts_with("export "));

    let mut stack = vec![node];
    while let Some(current) = stack.pop() {
        if current.kind() == "variable_declarator" {
            if ts_is_require_declarator(current, source) {
                continue;
            }
            if let Some(name_node) = current.child_by_field_name("name") {
                let name = node_text(source, name_node).trim().to_string();
                if !name.is_empty() {
                    let kind = ts_variable_symbol_kind(current);
                    if !scope.is_empty() && kind == "variable" {
                        continue;
                    }
                    out.push(make_extracted_symbol(
                        name,
                        kind,
                        exported,
                        scope,
                        parent_identity,
                        CodeLanguage::TypeScript,
                        source,
                        current,
                    ));
                }
            }
            continue;
        }

        let mut cursor = current.walk();
        for child in current.named_children(&mut cursor) {
            stack.push(child);
        }
    }

    out
}

pub(super) fn ts_symbol_relationships(
    node: Node<'_>,
    source: &str,
    symbol: &ExtractedSymbol,
) -> Vec<ExtractedRelationship> {
    if !matches!(node.kind(), "class_declaration" | "interface_declaration") {
        return Vec::new();
    }

    let mut relationships = Vec::new();
    let mut cursor = node.walk();
    for child in node.named_children(&mut cursor) {
        match child.kind() {
            "class_heritage" => {
                let mut heritage_cursor = child.walk();
                for clause in child.named_children(&mut heritage_cursor) {
                    match clause.kind() {
                        "extends_clause" => {
                            if let Some(value_node) = clause.child_by_field_name("value") {
                                if let Some((target_expr, target_name)) =
                                    ts_type_reference(value_node, source)
                                {
                                    relationships.push(ExtractedRelationship::new(
                                        symbol.identity.clone(),
                                        "extends",
                                        target_expr,
                                        target_name,
                                    ));
                                }
                            }
                        }
                        "implements_clause" => {
                            let mut clause_cursor = clause.walk();
                            for type_node in clause.named_children(&mut clause_cursor) {
                                if let Some((target_expr, target_name)) =
                                    ts_type_reference(type_node, source)
                                {
                                    relationships.push(ExtractedRelationship::new(
                                        symbol.identity.clone(),
                                        "implements",
                                        target_expr,
                                        target_name,
                                    ));
                                }
                            }
                        }
                        _ => {
                            if let Some((target_expr, target_name)) =
                                ts_type_reference(clause, source)
                            {
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
            }
            "extends_type_clause" => {
                let mut clause_cursor = child.walk();
                for type_node in child.named_children(&mut clause_cursor) {
                    if let Some((target_expr, target_name)) = ts_type_reference(type_node, source) {
                        relationships.push(ExtractedRelationship::new(
                            symbol.identity.clone(),
                            "extends",
                            target_expr,
                            target_name,
                        ));
                    }
                }
            }
            _ => {}
        }
    }

    relationships
}

pub(super) fn ts_type_reference(node: Node<'_>, source: &str) -> Option<(String, String)> {
    let raw = if node.kind() == "generic_type" {
        node.child_by_field_name("type")
            .map(|type_node| node_text(source, type_node).trim().to_string())
            .unwrap_or_else(|| node_text(source, node).trim().to_string())
    } else {
        node_text(source, node).trim().to_string()
    };
    let name = simple_symbol_reference_name(&raw)?;
    Some((raw, name))
}

pub(super) fn ts_call_target(node: Node<'_>, source: &str) -> Option<(String, String)> {
    let callable = match node.kind() {
        "call_expression" => node.child_by_field_name("function")?,
        "new_expression" => node.child_by_field_name("constructor")?,
        _ => return None,
    };

    match callable.kind() {
        "identifier" => {
            let target_expr = node_text(source, callable).trim().to_string();
            if target_expr.is_empty() {
                return None;
            }
            Some((target_expr.clone(), target_expr))
        }
        "member_expression" => {
            let object = callable.child_by_field_name("object")?;
            let property = callable.child_by_field_name("property")?;
            if object.kind() != "identifier" {
                return None;
            }
            let object_name = node_text(source, object).trim().to_string();
            let property_name = node_text(source, property).trim().to_string();
            if object_name.is_empty() || property_name.is_empty() {
                return None;
            }
            Some((format!("{object_name}.{property_name}"), property_name))
        }
        _ => None,
    }
}

pub(super) fn ts_require_imports_from_variable_statement(
    node: Node<'_>,
    source: &str,
) -> Vec<ExtractedImport> {
    let mut imports = Vec::new();
    let mut stack = vec![node];

    while let Some(current) = stack.pop() {
        if current.kind() == "variable_declarator" {
            let Some(name_node) = current.child_by_field_name("name") else {
                continue;
            };
            let Some(value_node) = current.child_by_field_name("value") else {
                continue;
            };
            let Some(module) = ts_require_module_from_value(value_node, source) else {
                continue;
            };

            match name_node.kind() {
                "identifier" => {
                    let local_name = node_text(source, name_node).trim().to_string();
                    if !local_name.is_empty() {
                        imports.push(
                            ExtractedImport::bindings(
                                module.clone(),
                                vec![ImportBinding::new("default", local_name.clone())],
                            )
                            .with_module_alias(local_name),
                        );
                    }
                }
                "object_pattern" => {
                    let bindings = ts_object_pattern_bindings(name_node, source);
                    if !bindings.is_empty() {
                        imports.push(ExtractedImport::bindings(module.clone(), bindings));
                    }
                }
                _ => {}
            }
            continue;
        }

        let mut cursor = current.walk();
        for child in current.named_children(&mut cursor) {
            stack.push(child);
        }
    }

    imports
}

pub(super) fn ts_require_module_from_value(node: Node<'_>, source: &str) -> Option<String> {
    if node.kind() != "call_expression" {
        return None;
    }
    let function = node.child_by_field_name("function")?;
    if function.kind() != "identifier" || node_text(source, function).trim() != "require" {
        return None;
    }
    let arguments = node.child_by_field_name("arguments")?;
    let mut cursor = arguments.walk();
    let argument = arguments.named_children(&mut cursor).next()?;
    if argument.kind() != "string" {
        return None;
    }
    let module = node_text(source, argument)
        .trim()
        .trim_matches(|c| c == '\'' || c == '"')
        .to_string();
    if module.is_empty() {
        None
    } else {
        Some(module)
    }
}

pub(super) fn ts_is_require_declarator(node: Node<'_>, source: &str) -> bool {
    node.child_by_field_name("value")
        .and_then(|value| ts_require_module_from_value(value, source))
        .is_some()
}

pub(super) fn ts_object_pattern_bindings(node: Node<'_>, source: &str) -> Vec<ImportBinding> {
    let text = node_text(source, node).trim();
    let Some(inner) = text
        .strip_prefix('{')
        .and_then(|rest| rest.strip_suffix('}'))
    else {
        return Vec::new();
    };

    let mut bindings = Vec::new();
    for part in inner.split(',') {
        let binding = part.trim();
        if binding.is_empty() || binding.starts_with("...") {
            continue;
        }
        let binding = binding.split('=').next().unwrap_or(binding).trim();
        let mut pieces = binding.split(':').map(str::trim);
        let Some(source_name) = pieces.next() else {
            continue;
        };
        let local_name = pieces.next().unwrap_or(source_name);
        if !source_name.is_empty()
            && !local_name.is_empty()
            && leading_js_identifier(source_name).as_deref() == Some(source_name)
            && leading_js_identifier(local_name).as_deref() == Some(local_name)
        {
            bindings.push(ImportBinding::new(source_name, local_name));
        }
    }

    bindings.sort();
    bindings.dedup();
    bindings
}

pub(super) fn collect_ts_commonjs_exports(
    node: Node<'_>,
    source: &str,
    analysis: &mut FileAnalysis,
) {
    let Some(assignment) = node
        .named_child(0)
        .filter(|child| child.kind() == "assignment_expression")
    else {
        return;
    };
    let Some(left) = assignment.child_by_field_name("left") else {
        return;
    };
    let Some(right) = assignment.child_by_field_name("right") else {
        return;
    };
    let Some(export_name) = ts_commonjs_export_name(left, source) else {
        return;
    };

    if export_name == "default" {
        let object_export_bindings = ts_commonjs_object_export_bindings(right, source, analysis);
        if !object_export_bindings.is_empty() {
            for binding in object_export_bindings {
                if binding.source_name == binding.local_name {
                    analysis
                        .exported_symbol_names
                        .insert(binding.local_name.clone());
                }
                analysis.export_bindings.push(binding);
            }
            return;
        }
    }

    let target_name = if right.kind() == "identifier" {
        let name = node_text(source, right).trim().to_string();
        if name.is_empty() {
            None
        } else {
            Some(name)
        }
    } else if let Some(symbol) = ts_commonjs_symbol_from_expression(right, source, &export_name) {
        let name = symbol.name.clone();
        analysis.symbols.push(symbol);
        Some(name)
    } else {
        None
    };

    if let Some(target_name) = target_name {
        analysis.exported_symbol_names.insert(target_name.clone());
        if export_name == "default" {
            analysis.default_exported_symbol_names.insert(target_name);
        }
    }
}

pub(super) fn ts_commonjs_object_export_bindings(
    node: Node<'_>,
    source: &str,
    analysis: &mut FileAnalysis,
) -> Vec<ImportBinding> {
    if node.kind() != "object" {
        return Vec::new();
    }

    let mut bindings = Vec::new();
    let mut cursor = node.walk();
    for property in node.named_children(&mut cursor) {
        let Some(export_name) = ts_commonjs_object_property_name(property, source) else {
            continue;
        };

        let local_name = if let Some(value) = property.child_by_field_name("value") {
            if value.kind() == "identifier" {
                let name = node_text(source, value).trim().to_string();
                if name.is_empty() {
                    None
                } else {
                    Some(name)
                }
            } else if let Some(symbol) =
                ts_commonjs_symbol_from_expression(value, source, &export_name)
            {
                let name = symbol.name.clone();
                analysis.symbols.push(symbol);
                Some(name)
            } else {
                None
            }
        } else {
            leading_js_identifier(node_text(source, property).trim())
        };

        if let Some(local_name) = local_name {
            bindings.push(ImportBinding::new(export_name, local_name));
        }
    }

    bindings.sort();
    bindings.dedup();
    bindings
}

pub(super) fn ts_commonjs_object_property_name(node: Node<'_>, source: &str) -> Option<String> {
    let key = node
        .child_by_field_name("key")
        .or_else(|| node.child_by_field_name("name"))
        .or_else(|| node.child_by_field_name("property"));

    let raw = key
        .map(|key| node_text(source, key).trim().to_string())
        .filter(|text| !text.is_empty())
        .or_else(|| leading_js_identifier(node_text(source, node).trim()))?;

    let normalized = raw.trim_matches(|c| c == '\'' || c == '"').to_string();
    if normalized.is_empty() {
        None
    } else {
        Some(normalized)
    }
}

pub(super) fn ts_commonjs_export_name(node: Node<'_>, source: &str) -> Option<String> {
    if node.kind() != "member_expression" {
        return None;
    }
    let object = node.child_by_field_name("object")?;
    let property = node.child_by_field_name("property")?;
    let property_name = node_text(source, property).trim().to_string();
    if property_name.is_empty() {
        return None;
    }

    if object.kind() == "identifier" {
        let object_name = node_text(source, object).trim();
        if object_name == "module" && property_name == "exports" {
            return Some("default".to_string());
        }
        if object_name == "exports" {
            return Some(property_name);
        }
    }

    if object.kind() == "member_expression" {
        let inner_object = object.child_by_field_name("object")?;
        let inner_property = object.child_by_field_name("property")?;
        if inner_object.kind() == "identifier"
            && node_text(source, inner_object).trim() == "module"
            && node_text(source, inner_property).trim() == "exports"
        {
            return Some(property_name);
        }
    }

    None
}

pub(super) fn ts_commonjs_symbol_from_expression(
    node: Node<'_>,
    source: &str,
    export_name: &str,
) -> Option<ExtractedSymbol> {
    let kind = match node.kind() {
        "function" | "function_expression" | "generator_function" | "arrow_function" => "function",
        "class" => "class",
        _ => return None,
    };

    let name = node
        .child_by_field_name("name")
        .map(|name_node| node_text(source, name_node).trim().to_string())
        .filter(|name| !name.is_empty())
        .or_else(|| {
            if export_name != "default" {
                Some(export_name.to_string())
            } else {
                None
            }
        })?;

    Some(make_extracted_symbol(
        name,
        kind,
        true,
        &[],
        None,
        CodeLanguage::JavaScript,
        source,
        node,
    ))
}

pub(super) fn ts_import_bindings(node: Node<'_>, source: &str) -> Vec<ImportBinding> {
    let mut bindings = Vec::new();
    let mut stack = vec![node];

    while let Some(current) = stack.pop() {
        if current.kind() == "import_clause" {
            let mut cursor = current.walk();
            for child in current.named_children(&mut cursor) {
                if child.kind() == "identifier" {
                    let local_name = node_text(source, child).trim().to_string();
                    if !local_name.is_empty() {
                        bindings.push(ImportBinding::new("default", local_name));
                    }
                } else {
                    stack.push(child);
                }
            }
            continue;
        }

        if current.kind() == "import_specifier" {
            if let Some(name_node) = current.child_by_field_name("name") {
                let source_name = node_text(source, name_node).trim().to_string();
                if !source_name.is_empty() {
                    let local_name = current
                        .child_by_field_name("alias")
                        .map(|alias_node| node_text(source, alias_node).trim().to_string())
                        .filter(|alias| !alias.is_empty())
                        .unwrap_or_else(|| source_name.clone());
                    bindings.push(ImportBinding::new(source_name, local_name));
                }
            }
            continue;
        }

        let mut cursor = current.walk();
        for child in current.named_children(&mut cursor) {
            stack.push(child);
        }
    }

    bindings.sort();
    bindings.dedup();
    bindings
}

pub(super) fn ts_namespace_import_aliases(node: Node<'_>, source: &str) -> Vec<String> {
    let mut aliases = Vec::new();
    let mut stack = vec![node];

    while let Some(current) = stack.pop() {
        if current.kind() == "namespace_import" {
            let mut cursor = current.walk();
            for child in current.named_children(&mut cursor) {
                let alias = node_text(source, child).trim().to_string();
                if !alias.is_empty() {
                    aliases.push(alias);
                }
            }
            continue;
        }

        let mut cursor = current.walk();
        for child in current.named_children(&mut cursor) {
            stack.push(child);
        }
    }

    aliases.sort();
    aliases.dedup();
    aliases
}

pub(super) fn ts_aliases_from_variable_statement(
    node: Node<'_>,
    source: &str,
    owner_identity: Option<&str>,
) -> Vec<ExtractedAlias> {
    let mut aliases = Vec::new();
    let mut stack = vec![node];

    while let Some(current) = stack.pop() {
        if current.kind() == "variable_declarator" {
            let Some(name_node) = current.child_by_field_name("name") else {
                continue;
            };
            let Some(value_node) = current.child_by_field_name("value") else {
                continue;
            };
            if ts_require_module_from_value(value_node, source).is_some() {
                continue;
            }
            if name_node.kind() != "identifier" {
                continue;
            }

            let alias_name = node_text(source, name_node).trim().to_string();
            if alias_name.is_empty() {
                continue;
            }

            let (target_expr, target_name) = ts_alias_reference(value_node, source)
                .unwrap_or_else(|| (String::new(), String::new()));
            aliases.push(ExtractedAlias::new(
                alias_name,
                target_expr,
                target_name,
                owner_identity,
            ));
            continue;
        }

        let mut cursor = current.walk();
        for child in current.named_children(&mut cursor) {
            stack.push(child);
        }
    }

    aliases
}

pub(super) fn ts_alias_reference(node: Node<'_>, source: &str) -> Option<(String, String)> {
    match node.kind() {
        "identifier" => {
            let text = node_text(source, node).trim().to_string();
            if text.is_empty() {
                None
            } else {
                Some((text.clone(), text))
            }
        }
        "member_expression" => {
            let object = node.child_by_field_name("object")?;
            let property = node.child_by_field_name("property")?;
            if object.kind() != "identifier" {
                return None;
            }
            let object_name = node_text(source, object).trim().to_string();
            let property_name = node_text(source, property).trim().to_string();
            if object_name.is_empty() || property_name.is_empty() {
                None
            } else {
                Some((format!("{object_name}.{property_name}"), property_name))
            }
        }
        _ => None,
    }
}

pub(super) fn ts_reexport_bindings(node: Node<'_>, source: &str) -> Vec<ImportBinding> {
    let mut bindings = Vec::new();
    let mut stack = vec![node];

    while let Some(current) = stack.pop() {
        if current.kind() == "export_specifier" {
            if let Some(name_node) = current.child_by_field_name("name") {
                let source_name = node_text(source, name_node).trim().to_string();
                if !source_name.is_empty() {
                    let local_name = current
                        .child_by_field_name("alias")
                        .map(|alias_node| node_text(source, alias_node).trim().to_string())
                        .filter(|alias| !alias.is_empty())
                        .unwrap_or_else(|| source_name.clone());
                    bindings.push(ImportBinding::new(source_name, local_name));
                }
            }
            continue;
        }

        let mut cursor = current.walk();
        for child in current.named_children(&mut cursor) {
            stack.push(child);
        }
    }

    bindings.sort();
    bindings.dedup();
    bindings
}

pub(super) fn ts_is_wildcard_reexport(node: Node<'_>, source: &str) -> bool {
    let text = node_text(source, node);
    text.contains("export *") && !text.contains('{')
}

pub(super) fn mark_exported_symbols(
    symbols: &mut [ExtractedSymbol],
    exported_names: &BTreeSet<String>,
) {
    if exported_names.is_empty() {
        return;
    }

    for symbol in symbols.iter_mut() {
        if symbol.parent_identity.is_none() && exported_names.contains(&symbol.name) {
            symbol.exported = true;
        }
    }
}

pub(super) fn collect_ts_local_export_names(
    node: Node<'_>,
    source: &str,
    exported_names: &mut BTreeSet<String>,
) {
    if node.child_by_field_name("source").is_some() {
        return;
    }

    for name in ts_local_export_names_from_text(node_text(source, node)) {
        exported_names.insert(name);
    }
}

pub(super) fn ts_local_export_names_from_text(text: &str) -> Vec<String> {
    let trimmed = text.trim().trim_end_matches(';').trim();
    let mut names = Vec::new();

    if let Some(rest) = trimmed.strip_prefix("export default ") {
        let rest = rest.trim();
        if !matches!(
            rest.split_whitespace().next(),
            Some("function" | "class" | "async")
        ) {
            if let Some(name) = leading_js_identifier(rest) {
                names.push(name);
            }
        }
    }

    if let (Some(open), Some(close)) = (trimmed.find('{'), trimmed.rfind('}')) {
        if close > open {
            for item in split_top_level_commas(&trimmed[open + 1..close]) {
                let local = item
                    .split_once(" as ")
                    .map(|(left, _)| left)
                    .unwrap_or(item.as_str())
                    .trim();
                let local = local.strip_prefix("type ").unwrap_or(local).trim();
                if let Some(name) = leading_js_identifier(local) {
                    names.push(name);
                }
            }
        }
    }

    names
}

pub(super) fn ts_default_export_names_from_text(text: &str) -> Vec<String> {
    let trimmed = text.trim().trim_end_matches(';').trim();
    let Some(rest) = trimmed.strip_prefix("export default ") else {
        return Vec::new();
    };
    let rest = rest.trim();

    let candidate = if let Some(rest) = rest.strip_prefix("async function ") {
        leading_js_identifier(rest)
    } else if let Some(rest) = rest.strip_prefix("function ") {
        leading_js_identifier(rest)
    } else if let Some(rest) = rest.strip_prefix("class ") {
        leading_js_identifier(rest)
    } else {
        leading_js_identifier(rest)
    };

    candidate.into_iter().collect()
}

pub(in crate::legacy) fn extend_unique_block_ids<I>(target: &mut Vec<BlockId>, ids: I) -> bool
where
    I: IntoIterator<Item = BlockId>,
{
    let before = target.len();
    for id in ids {
        if !target.contains(&id) {
            target.push(id);
        }
    }
    target.len() != before
}

pub(super) fn leading_js_identifier(text: &str) -> Option<String> {
    let mut chars = text.chars();
    let first = chars.next()?;
    if !(first == '_' || first == '$' || first.is_ascii_alphabetic()) {
        return None;
    }

    let mut ident = String::from(first);
    for ch in chars {
        if ch == '_' || ch == '$' || ch.is_ascii_alphanumeric() {
            ident.push(ch);
        } else {
            break;
        }
    }

    Some(ident)
}

pub(super) fn simple_symbol_reference_name(text: &str) -> Option<String> {
    let trimmed = text.trim();
    let trimmed = trimmed.strip_prefix("readonly ").unwrap_or(trimmed).trim();
    let name = leading_js_identifier(trimmed)?;
    let rest = trimmed[name.len()..].trim_start();
    if rest.is_empty() || rest.starts_with('<') || rest.starts_with('[') || rest.starts_with('?') {
        Some(name)
    } else {
        None
    }
}

pub(super) fn ts_public_field_kind(node: Node<'_>) -> &'static str {
    node.child_by_field_name("value")
        .map(|value| {
            if is_ts_function_like_kind(value.kind()) {
                "method"
            } else {
                "field"
            }
        })
        .unwrap_or("field")
}

pub(super) fn ts_variable_symbol_kind(node: Node<'_>) -> &'static str {
    node.child_by_field_name("value")
        .map(|value| match value.kind() {
            kind if is_ts_function_like_kind(kind) => "function",
            "class" => "class",
            _ => "variable",
        })
        .unwrap_or("variable")
}

pub(super) fn is_ts_function_like_kind(kind: &str) -> bool {
    matches!(
        kind,
        "arrow_function" | "function_expression" | "generator_function"
    )
}

pub(super) fn extract_ts_module_from_text(text: &str) -> Option<String> {
    let patterns = [
        Regex::new(r#"(?i)\bfrom\s+['"]([^'"]+)['"]"#).ok()?,
        Regex::new(r#"(?i)\bimport\s+['"]([^'"]+)['"]"#).ok()?,
        Regex::new(r#"(?i)require\(\s*['"]([^'"]+)['"]\s*\)"#).ok()?,
    ];
    for pattern in patterns {
        if let Some(caps) = pattern.captures(text) {
            if let Some(module) = caps.get(1).map(|m| m.as_str().trim()) {
                if !module.is_empty() {
                    return Some(module.to_string());
                }
            }
        }
    }
    None
}
