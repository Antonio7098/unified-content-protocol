use tree_sitter::Node;

use crate::model::*;

pub(super) fn node_text<'a>(source: &'a str, node: Node<'_>) -> &'a str {
    let start = node.start_byte().min(source.len());
    let end = node.end_byte().min(source.len());
    &source[start..end]
}

pub(super) fn extract_symbol_description(
    source: &str,
    language: CodeLanguage,
    node: Node<'_>,
) -> Option<String> {
    match language {
        CodeLanguage::Rust => extract_preceding_symbol_comment(source, node, language),
        CodeLanguage::Python => extract_python_symbol_description(source, node)
            .or_else(|| extract_preceding_symbol_comment(source, node, language)),
        CodeLanguage::TypeScript | CodeLanguage::JavaScript => {
            extract_preceding_symbol_comment(source, node, language)
        }
    }
}

pub(super) fn extract_symbol_signature(
    source: &str,
    language: CodeLanguage,
    node: Node<'_>,
    kind: &str,
    name: &str,
) -> ExtractedSignature {
    match language {
        CodeLanguage::Rust => {
            extract_rust_symbol_signature(node_text(source, node), node.kind(), name)
        }
        CodeLanguage::Python => {
            extract_python_symbol_signature(node_text(source, node), node.kind(), name)
        }
        CodeLanguage::TypeScript | CodeLanguage::JavaScript => {
            extract_ts_js_symbol_signature(source, language, node, kind, name)
        }
    }
}

pub(super) fn extract_symbol_modifiers(
    source: &str,
    language: CodeLanguage,
    node: Node<'_>,
    kind: &str,
) -> ExtractedModifiers {
    match language {
        CodeLanguage::Rust => extract_rust_symbol_modifiers(node_text(source, node), node.kind()),
        CodeLanguage::Python => extract_python_symbol_modifiers(source, node, node.kind()),
        CodeLanguage::TypeScript | CodeLanguage::JavaScript => {
            extract_ts_js_symbol_modifiers(source, node, kind)
        }
    }
}

pub(super) fn extract_rust_symbol_modifiers(raw: &str, node_kind: &str) -> ExtractedModifiers {
    let header = take_until_top_level(raw, &['{', ';']).trim();
    ExtractedModifiers {
        is_async: node_kind == "function_item" && has_modifier_token(header, "async"),
        visibility: rust_visibility(header),
        ..Default::default()
    }
}

pub(super) fn extract_python_symbol_modifiers(
    source: &str,
    node: Node<'_>,
    node_kind: &str,
) -> ExtractedModifiers {
    let raw = node_text(source, node);
    ExtractedModifiers {
        is_async: node_kind == "async_function_definition"
            || take_until_top_level(raw, &[':'])
                .trim_start()
                .starts_with("async def "),
        is_static: has_python_decorator(source, node, "staticmethod"),
        ..Default::default()
    }
}

pub(super) fn has_python_decorator(source: &str, node: Node<'_>, decorator: &str) -> bool {
    let lines: Vec<&str> = source.lines().collect();
    let start_line = node.start_position().row;
    let decorator_line = format!("@{}", decorator);
    let mut index = start_line;
    while index > 0 {
        index -= 1;
        let line = lines.get(index).copied().unwrap_or_default().trim();
        if line.is_empty() {
            continue;
        }
        if line == decorator_line {
            return true;
        }
        if line.starts_with('@') {
            continue;
        }
        break;
    }
    false
}

pub(super) fn extract_ts_js_symbol_modifiers(
    source: &str,
    node: Node<'_>,
    kind: &str,
) -> ExtractedModifiers {
    let raw = node_text(source, node);
    let header = take_until_top_level(raw, &['{', ';']).trim();
    let target_header = if node.kind() == "variable_declarator" {
        node.child_by_field_name("value")
            .map(|value| {
                take_until_top_level(node_text(source, value), &['{', ';'])
                    .trim()
                    .to_string()
            })
            .unwrap_or_else(|| header.to_string())
    } else {
        header.to_string()
    };

    let generator = matches!(
        node.kind(),
        "generator_function_declaration" | "generator_function"
    ) || target_header.starts_with("function*")
        || target_header.starts_with("async function*")
        || first_non_modifier_fragment(header)
            .map(|fragment| fragment.starts_with('*'))
            .unwrap_or(false);

    ExtractedModifiers {
        is_async: matches!(kind, "function" | "method")
            && (has_modifier_token(header, "async") || has_modifier_token(&target_header, "async")),
        generator,
        is_static: has_modifier_token(header, "static"),
        visibility: extract_ts_js_visibility(header),
    }
}

pub(super) fn rust_visibility(header: &str) -> Option<String> {
    let trimmed = header.trim_start();
    if trimmed.starts_with("pub(") {
        Some("restricted".to_string())
    } else if trimmed.starts_with("pub ") || trimmed == "pub" {
        Some("public".to_string())
    } else {
        None
    }
}

pub(super) fn extract_ts_js_visibility(header: &str) -> Option<String> {
    ["public", "private", "protected"]
        .into_iter()
        .find(|candidate| has_modifier_token(header, candidate))
        .map(|value| value.to_string())
}

pub(super) fn has_modifier_token(header: &str, target: &str) -> bool {
    header
        .split(|ch: char| !ch.is_ascii_alphanumeric() && ch != '_')
        .any(|token| token == target)
}

pub(super) fn first_non_modifier_fragment(header: &str) -> Option<&str> {
    let mut rest = header.trim_start();
    loop {
        let next = strip_modifier_prefix(rest, "export")
            .or_else(|| strip_modifier_prefix(rest, "default"))
            .or_else(|| strip_modifier_prefix(rest, "declare"))
            .or_else(|| strip_modifier_prefix(rest, "abstract"))
            .or_else(|| strip_modifier_prefix(rest, "public"))
            .or_else(|| strip_modifier_prefix(rest, "private"))
            .or_else(|| strip_modifier_prefix(rest, "protected"))
            .or_else(|| strip_modifier_prefix(rest, "readonly"))
            .or_else(|| strip_modifier_prefix(rest, "static"))
            .or_else(|| strip_modifier_prefix(rest, "async"));
        match next {
            Some(value) => rest = value,
            None => break,
        }
    }

    if rest.is_empty() {
        None
    } else {
        Some(rest)
    }
}

pub(super) fn strip_modifier_prefix<'a>(value: &'a str, token: &str) -> Option<&'a str> {
    let rest = value.strip_prefix(token)?;
    match rest.chars().next() {
        Some(ch) if ch.is_whitespace() => Some(rest.trim_start()),
        _ => None,
    }
}

pub(super) fn extract_rust_symbol_signature(
    raw: &str,
    node_kind: &str,
    name: &str,
) -> ExtractedSignature {
    let header = take_until_top_level(raw, &['{', ';']);
    match node_kind {
        "function_item" => extract_function_like_signature(header, CodeLanguage::Rust),
        "trait_item" => ExtractedSignature {
            type_info: extract_rust_trait_bounds(header, name),
            ..Default::default()
        },
        "type_item" => ExtractedSignature {
            type_info: extract_after_top_level_char(header, '='),
            ..Default::default()
        },
        "const_item" => ExtractedSignature {
            type_info: extract_between_top_level_chars(header, ':', '='),
            ..Default::default()
        },
        "impl_item" => ExtractedSignature {
            type_info: normalize_signature_fragment(header.trim_start_matches("impl").trim(), 160),
            ..Default::default()
        },
        _ => ExtractedSignature::default(),
    }
}

pub(super) fn extract_python_symbol_signature(
    raw: &str,
    node_kind: &str,
    _name: &str,
) -> ExtractedSignature {
    let header = take_until_top_level(raw, &[':']);
    match node_kind {
        "function_definition" | "async_function_definition" => {
            extract_function_like_signature(header, CodeLanguage::Python)
        }
        "class_definition" => ExtractedSignature {
            type_info: extract_parenthesized_clause(header),
            ..Default::default()
        },
        _ => ExtractedSignature::default(),
    }
}

pub(super) fn extract_ts_js_symbol_signature(
    source: &str,
    language: CodeLanguage,
    node: Node<'_>,
    kind: &str,
    name: &str,
) -> ExtractedSignature {
    let target = if node.kind() == "variable_declarator" {
        node.child_by_field_name("value").unwrap_or(node)
    } else {
        node
    };
    let raw = node_text(source, target);

    match kind {
        "function" | "method" => {
            extract_function_like_signature_ts_js(take_until_top_level(raw, &['{', ';']), language)
        }
        "class" => ExtractedSignature {
            type_info: extract_ts_js_heritage(
                take_until_top_level(raw, &['{', ';']),
                name,
                "class",
            ),
            ..Default::default()
        },
        "interface" => ExtractedSignature {
            type_info: extract_ts_js_heritage(
                take_until_top_level(raw, &['{', ';']),
                name,
                "interface",
            ),
            ..Default::default()
        },
        "type" => ExtractedSignature {
            type_info: extract_after_top_level_char(take_until_top_level(raw, &[';']), '='),
            ..Default::default()
        },
        "variable" => ExtractedSignature {
            type_info: extract_ts_js_annotation(take_until_top_level(raw, &[';'])),
            ..Default::default()
        },
        _ => ExtractedSignature::default(),
    }
}

pub(super) fn extract_function_like_signature(
    header: &str,
    language: CodeLanguage,
) -> ExtractedSignature {
    let mut signature = ExtractedSignature::default();
    if let Some((params, close_index)) = extract_first_parenthesized_with_end(header) {
        signature.inputs = parse_parameter_list(&params, language);
        signature.output = match language {
            CodeLanguage::Rust => extract_rust_return_type(&header[close_index + 1..]),
            CodeLanguage::Python => extract_python_return_type(&header[close_index + 1..]),
            CodeLanguage::TypeScript | CodeLanguage::JavaScript => None,
        };
    }
    signature
}

pub(super) fn extract_function_like_signature_ts_js(
    header: &str,
    language: CodeLanguage,
) -> ExtractedSignature {
    let mut signature = ExtractedSignature::default();
    if let Some((params, close_index)) = extract_first_parenthesized_with_end(header) {
        signature.inputs = parse_parameter_list(&params, language);
        if language == CodeLanguage::TypeScript {
            signature.output = extract_ts_return_type(&header[close_index + 1..]);
        }
        return signature;
    }

    if let Some((input, output)) = extract_single_param_arrow_signature(header, language) {
        signature.inputs = vec![input];
        signature.output = output;
    }
    signature
}

pub(super) fn extract_single_param_arrow_signature(
    header: &str,
    language: CodeLanguage,
) -> Option<(ExtractedInput, Option<String>)> {
    let arrow_index = header.find("=>")?;
    let before_arrow = header[..arrow_index].trim_end();
    let candidate = before_arrow
        .rsplit_once('=')
        .map(|(_, value)| value.trim())
        .unwrap_or(before_arrow);
    if candidate.contains('(') {
        return None;
    }

    let (name_part, type_part) =
        split_top_level_once(candidate, ':').unwrap_or((candidate.to_string(), String::new()));
    let input = ExtractedInput {
        name: normalize_parameter_name(&name_part),
        type_name: normalize_signature_fragment(type_part.trim(), 120),
    };
    let output = if language == CodeLanguage::TypeScript {
        None
    } else {
        None
    };
    Some((input, output))
}

pub(super) fn extract_rust_trait_bounds(header: &str, name: &str) -> Option<String> {
    let tail = substring_after_name(header, name)?;
    let bounds = tail.trim_start().strip_prefix(':')?.trim();
    normalize_signature_fragment(bounds, 160)
}

pub(super) fn extract_rust_return_type(tail: &str) -> Option<String> {
    let tail = tail.trim();
    let rest = tail.strip_prefix("->")?.trim();
    let before_where = rest.split(" where ").next().unwrap_or(rest).trim();
    normalize_signature_fragment(before_where, 160)
}

pub(super) fn extract_python_return_type(tail: &str) -> Option<String> {
    let tail = tail.trim();
    let rest = tail.strip_prefix("->")?.trim();
    normalize_signature_fragment(rest.trim_end_matches(':').trim(), 160)
}

pub(super) fn extract_ts_return_type(tail: &str) -> Option<String> {
    let tail = tail.trim();
    let rest = tail.strip_prefix(':')?.trim();
    let before_arrow = rest.split("=>").next().unwrap_or(rest).trim();
    normalize_signature_fragment(before_arrow, 160)
}

pub(super) fn extract_ts_js_annotation(header: &str) -> Option<String> {
    let before_equals = header.split('=').next().unwrap_or(header).trim();
    let (_, annotation) = split_top_level_once(before_equals, ':')?;
    normalize_signature_fragment(annotation.trim(), 160)
}

pub(super) fn extract_ts_js_heritage(header: &str, name: &str, keyword: &str) -> Option<String> {
    let after_keyword = header.trim_start().strip_prefix(keyword)?.trim_start();
    let tail = if let Some(after_name) = after_keyword.strip_prefix(name) {
        after_name.trim_start()
    } else {
        after_keyword
    };
    normalize_signature_fragment(tail, 160)
}

pub(super) fn extract_parenthesized_clause(header: &str) -> Option<String> {
    let (inner, _) = extract_first_parenthesized_with_end(header)?;
    normalize_signature_fragment(&inner, 160)
}

pub(super) fn parse_parameter_list(raw: &str, language: CodeLanguage) -> Vec<ExtractedInput> {
    split_top_level(raw, ',')
        .into_iter()
        .filter_map(|part| parse_parameter(&part, language))
        .collect()
}

pub(super) fn parse_parameter(raw: &str, language: CodeLanguage) -> Option<ExtractedInput> {
    let trimmed = raw.trim();
    if trimmed.is_empty() || trimmed == "/" || trimmed == "*" {
        return None;
    }

    let without_default = split_top_level_once(trimmed, '=')
        .map(|(left, _)| left)
        .unwrap_or_else(|| trimmed.to_string());
    let without_default = without_default.trim();

    if language == CodeLanguage::Rust
        && without_default.contains("self")
        && !without_default.contains(':')
    {
        return Some(ExtractedInput {
            name: "self".to_string(),
            type_name: None,
        });
    }

    let (name_part, type_part) = split_top_level_once(without_default, ':')
        .unwrap_or((without_default.to_string(), String::new()));

    let name = normalize_parameter_name(&name_part);
    if name.is_empty() {
        return None;
    }

    Some(ExtractedInput {
        name,
        type_name: normalize_signature_fragment(type_part.trim(), 120),
    })
}

pub(super) fn normalize_parameter_name(raw: &str) -> String {
    raw.trim()
        .trim_start_matches("...")
        .trim_start_matches("**")
        .trim_start_matches('*')
        .trim_start_matches("mut ")
        .trim_start_matches("ref ")
        .trim_start_matches("readonly ")
        .trim_start_matches("public ")
        .trim_start_matches("private ")
        .trim_start_matches("protected ")
        .trim()
        .to_string()
}

pub(super) fn extract_first_parenthesized_with_end(header: &str) -> Option<(String, usize)> {
    let open = header.find('(')?;
    let close = find_matching_delimiter(header, open, '(', ')')?;
    Some((header[open + 1..close].to_string(), close))
}

pub(super) fn extract_between_top_level_chars(
    raw: &str,
    start_char: char,
    end_char: char,
) -> Option<String> {
    let start = find_top_level_signature_char(raw, start_char)?;
    let after_start = &raw[start + start_char.len_utf8()..];
    let end = find_top_level_signature_char(after_start, end_char)?;
    normalize_signature_fragment(&after_start[..end], 160)
}

pub(super) fn extract_after_top_level_char(raw: &str, target: char) -> Option<String> {
    let start = find_top_level_signature_char(raw, target)?;
    normalize_signature_fragment(&raw[start + target.len_utf8()..], 160)
}

pub(super) fn substring_after_name<'a>(header: &'a str, name: &str) -> Option<&'a str> {
    let index = header.find(name)?;
    Some(&header[index + name.len()..])
}

pub(super) fn normalize_signature_fragment(raw: &str, max_len: usize) -> Option<String> {
    truncate_text(
        raw.trim()
            .trim_end_matches('{')
            .trim_end_matches(';')
            .trim(),
        max_len,
    )
}

pub(super) fn take_until_top_level<'a>(raw: &'a str, stop_chars: &[char]) -> &'a str {
    let mut paren_depth = 0usize;
    let mut bracket_depth = 0usize;
    let mut angle_depth = 0usize;

    for (idx, ch) in raw.char_indices() {
        if stop_chars.contains(&ch) && paren_depth == 0 && bracket_depth == 0 && angle_depth == 0 {
            return &raw[..idx];
        }

        match ch {
            '(' => paren_depth += 1,
            ')' => paren_depth = paren_depth.saturating_sub(1),
            '[' => bracket_depth += 1,
            ']' => bracket_depth = bracket_depth.saturating_sub(1),
            '<' => angle_depth += 1,
            '>' => angle_depth = angle_depth.saturating_sub(1),
            _ => {}
        }
    }

    raw
}

pub(super) fn find_top_level_signature_char(raw: &str, target: char) -> Option<usize> {
    let mut paren_depth = 0usize;
    let mut bracket_depth = 0usize;
    let mut angle_depth = 0usize;

    for (idx, ch) in raw.char_indices() {
        if ch == target && paren_depth == 0 && bracket_depth == 0 && angle_depth == 0 {
            return Some(idx);
        }

        match ch {
            '(' => paren_depth += 1,
            ')' => paren_depth = paren_depth.saturating_sub(1),
            '[' => bracket_depth += 1,
            ']' => bracket_depth = bracket_depth.saturating_sub(1),
            '<' => angle_depth += 1,
            '>' => angle_depth = angle_depth.saturating_sub(1),
            _ => {}
        }
    }

    None
}

pub(super) fn split_top_level_once(raw: &str, delimiter: char) -> Option<(String, String)> {
    let index = find_top_level_signature_char(raw, delimiter)?;
    Some((
        raw[..index].trim().to_string(),
        raw[index + delimiter.len_utf8()..].trim().to_string(),
    ))
}

pub(super) fn split_top_level(raw: &str, delimiter: char) -> Vec<String> {
    let mut parts = Vec::new();
    let mut start = 0usize;
    let mut paren_depth = 0usize;
    let mut bracket_depth = 0usize;
    let mut brace_depth = 0usize;
    let mut angle_depth = 0usize;

    for (idx, ch) in raw.char_indices() {
        match ch {
            '(' => paren_depth += 1,
            ')' => paren_depth = paren_depth.saturating_sub(1),
            '[' => bracket_depth += 1,
            ']' => bracket_depth = bracket_depth.saturating_sub(1),
            '{' => brace_depth += 1,
            '}' => brace_depth = brace_depth.saturating_sub(1),
            '<' => angle_depth += 1,
            '>' => angle_depth = angle_depth.saturating_sub(1),
            _ => {}
        }

        if ch == delimiter
            && paren_depth == 0
            && bracket_depth == 0
            && brace_depth == 0
            && angle_depth == 0
        {
            parts.push(raw[start..idx].trim().to_string());
            start = idx + delimiter.len_utf8();
        }
    }

    parts.push(raw[start..].trim().to_string());
    parts
}

pub(super) fn find_matching_delimiter(
    raw: &str,
    open_index: usize,
    open: char,
    close: char,
) -> Option<usize> {
    let mut depth = 0usize;
    for (idx, ch) in raw[open_index..].char_indices() {
        let absolute = open_index + idx;
        if ch == open {
            depth += 1;
        } else if ch == close {
            depth = depth.saturating_sub(1);
            if depth == 0 {
                return Some(absolute);
            }
        }
    }
    None
}

pub(super) fn extract_file_description(source: &str, language: CodeLanguage) -> Option<String> {
    let lines: Vec<&str> = source.lines().collect();
    let mut index = 0usize;

    if lines
        .first()
        .map(|line| line.starts_with("#!"))
        .unwrap_or(false)
    {
        index += 1;
    }

    while index < lines.len() && lines[index].trim().is_empty() {
        index += 1;
    }

    let remaining = &lines[index..];
    if remaining.is_empty() {
        return None;
    }

    if language == CodeLanguage::Python {
        if let Some(description) = extract_python_module_docstring(remaining) {
            return Some(description);
        }
    }

    extract_leading_block_comment(remaining)
        .or_else(|| extract_leading_line_comment_block(remaining, language))
}

pub(super) fn extract_python_module_docstring(lines: &[&str]) -> Option<String> {
    let first = lines.first()?.trim_start();
    let quote = if first.starts_with("\"\"\"") {
        "\"\"\""
    } else if first.starts_with("'''") {
        "'''"
    } else {
        return None;
    };

    let mut raw = String::new();
    let rest = &first[quote.len()..];
    if let Some(end) = rest.find(quote) {
        raw.push_str(&rest[..end]);
        return normalize_description_text(&raw);
    }

    raw.push_str(rest);
    for line in &lines[1..] {
        raw.push('\n');
        if let Some(end) = line.find(quote) {
            raw.push_str(&line[..end]);
            return normalize_description_text(&raw);
        }
        raw.push_str(line);
    }

    normalize_description_text(&raw)
}

pub(super) fn extract_leading_block_comment(lines: &[&str]) -> Option<String> {
    let first = lines.first()?.trim_start();
    let rest = if let Some(rest) = first.strip_prefix("/**") {
        rest
    } else if let Some(rest) = first.strip_prefix("/*") {
        rest
    } else {
        return None;
    };

    let mut raw = String::new();
    if let Some(end) = rest.find("*/") {
        raw.push_str(&rest[..end]);
        return normalize_description_text(&raw);
    }

    raw.push_str(rest);
    for line in &lines[1..] {
        raw.push('\n');
        if let Some(end) = line.find("*/") {
            raw.push_str(&line[..end]);
            return normalize_description_text(&raw);
        }
        raw.push_str(line);
    }

    normalize_description_text(&raw)
}

pub(super) fn extract_leading_line_comment_block(
    lines: &[&str],
    language: CodeLanguage,
) -> Option<String> {
    let mut collected = Vec::new();

    for line in lines {
        let trimmed = line.trim_start();
        let Some(rest) = strip_line_comment_prefix(trimmed, language) else {
            break;
        };
        let content = rest.trim();
        if content.starts_with("<reference") {
            return None;
        }
        collected.push(content.to_string());
    }

    if collected.is_empty() {
        None
    } else {
        normalize_description_text(&collected.join("\n"))
    }
}

pub(super) fn strip_line_comment_prefix<'a>(
    line: &'a str,
    language: CodeLanguage,
) -> Option<&'a str> {
    match language {
        CodeLanguage::Rust => line
            .strip_prefix("//!")
            .or_else(|| line.strip_prefix("///"))
            .or_else(|| line.strip_prefix("//")),
        CodeLanguage::Python => line.strip_prefix('#'),
        CodeLanguage::TypeScript | CodeLanguage::JavaScript => line.strip_prefix("//"),
    }
}

pub(super) fn normalize_description_text(raw: &str) -> Option<String> {
    let mut lines = Vec::new();
    for line in raw.lines() {
        let cleaned = line.trim().trim_start_matches('*').trim();
        if cleaned.is_empty() {
            continue;
        }
        lines.push(cleaned);
    }

    if lines.is_empty() {
        return None;
    }

    truncate_text(&lines.join(" "), 200)
}

pub(super) fn extract_python_symbol_description(source: &str, node: Node<'_>) -> Option<String> {
    let body = node.child_by_field_name("body")?;
    let mut cursor = body.walk();
    let first = body.named_children(&mut cursor).next()?;
    if first.kind() != "expression_statement" {
        return None;
    }

    extract_python_string_literal_text(node_text(source, first))
}

pub(super) fn extract_python_string_literal_text(raw: &str) -> Option<String> {
    let trimmed = raw.trim();
    let first_quote = trimmed.find(['"', '\''])?;
    let quoted = &trimmed[first_quote..];

    let quote = if quoted.starts_with("\"\"\"") {
        "\"\"\""
    } else if quoted.starts_with("'''") {
        "'''"
    } else if quoted.starts_with('"') {
        "\""
    } else if quoted.starts_with('\'') {
        "'"
    } else {
        return None;
    };

    let rest = &quoted[quote.len()..];
    let end = rest.rfind(quote)?;
    normalize_description_text(&rest[..end])
}

pub(super) fn extract_preceding_symbol_comment(
    source: &str,
    node: Node<'_>,
    language: CodeLanguage,
) -> Option<String> {
    let lines: Vec<&str> = source.lines().collect();
    if lines.is_empty() {
        return None;
    }

    let mut index = node.start_position().row.checked_sub(1)?;
    if language == CodeLanguage::Rust {
        while let Some(line) = lines.get(index) {
            let trimmed = line.trim_start();
            if trimmed.starts_with("#[") || trimmed.starts_with("#![") {
                if index == 0 {
                    return None;
                }
                index -= 1;
                continue;
            }
            break;
        }
    }

    extract_preceding_block_comment_before(&lines, index)
        .or_else(|| extract_preceding_line_comment_block_before(&lines, index, language))
}

pub(super) fn extract_preceding_block_comment_before(
    lines: &[&str],
    end_index: usize,
) -> Option<String> {
    let end_line = lines.get(end_index)?.trim_end();
    if !end_line.contains("*/") {
        return None;
    }

    let mut start_index = end_index;
    loop {
        let line = lines.get(start_index)?.trim_start();
        if line.contains("/*") {
            break;
        }
        if start_index == 0 {
            return None;
        }
        start_index -= 1;
    }

    let raw = lines[start_index..=end_index].join("\n");
    let start = raw.find("/*")? + 2;
    let end = raw.rfind("*/")?;
    normalize_description_text(&raw[start..end])
}

pub(super) fn extract_preceding_line_comment_block_before(
    lines: &[&str],
    end_index: usize,
    language: CodeLanguage,
) -> Option<String> {
    let mut collected = Vec::new();
    let mut index = end_index;

    loop {
        let line = lines.get(index)?.trim_start();
        if language == CodeLanguage::Rust {
            if let Some(doc_attr) = strip_rust_doc_attribute(line) {
                collected.push(doc_attr);
            } else if let Some(rest) = strip_symbol_line_comment_prefix(line, language) {
                collected.push(rest.trim().to_string());
            } else if collected.is_empty()
                && (line.starts_with("#(") || line.starts_with("#[") || line.starts_with("#!["))
            {
                // no-op branch retained for symmetry with Rust attribute skipping above
            } else {
                break;
            }
        } else if let Some(rest) = strip_symbol_line_comment_prefix(line, language) {
            collected.push(rest.trim().to_string());
        } else {
            break;
        }

        if index == 0 {
            break;
        }
        index -= 1;
    }

    if collected.is_empty() {
        None
    } else {
        collected.reverse();
        normalize_description_text(&collected.join("\n"))
    }
}

pub(super) fn strip_symbol_line_comment_prefix<'a>(
    line: &'a str,
    language: CodeLanguage,
) -> Option<&'a str> {
    match language {
        CodeLanguage::Rust => line.strip_prefix("///").or_else(|| line.strip_prefix("//")),
        CodeLanguage::Python => line.strip_prefix('#'),
        CodeLanguage::TypeScript | CodeLanguage::JavaScript => line.strip_prefix("//"),
    }
}

pub(super) fn strip_rust_doc_attribute(line: &str) -> Option<String> {
    let trimmed = line.trim();
    if !trimmed.starts_with("#[doc") {
        return None;
    }

    let start = trimmed.find('"')? + 1;
    let end = trimmed.rfind('"')?;
    if end <= start {
        return None;
    }

    Some(trimmed[start..end].to_string())
}

pub(super) fn truncate_text(raw: &str, max_len: usize) -> Option<String> {
    let collapsed = raw.split_whitespace().collect::<Vec<_>>().join(" ");
    if collapsed.is_empty() {
        return None;
    }
    if collapsed.chars().count() <= max_len {
        return Some(collapsed);
    }

    let mut end = 0usize;
    for (count, (idx, ch)) in collapsed.char_indices().enumerate() {
        if count >= max_len.saturating_sub(1) {
            break;
        }
        end = idx + ch.len_utf8();
    }

    Some(format!("{}…", collapsed[..end].trim_end()))
}

pub(super) fn format_line_range(start_line: usize, end_line: usize) -> String {
    if start_line == end_line {
        format!("L{}", start_line)
    } else {
        format!("L{}-L{}", start_line, end_line)
    }
}

pub(super) fn format_coderef(path: &str, line_range: &str) -> String {
    format!("{}#{}", path, line_range)
}

pub(super) fn node_span(node: Node<'_>) -> (usize, usize, usize, usize) {
    let start = node.start_position();
    let end = node.end_position();
    (start.row + 1, start.column + 1, end.row + 1, end.column + 1)
}

pub(super) fn first_named_identifier(node: Node<'_>, source: &str) -> Option<String> {
    let mut stack = vec![node];
    while let Some(current) = stack.pop() {
        if matches!(current.kind(), "identifier" | "type_identifier") {
            let text = node_text(source, current).trim().to_string();
            if !text.is_empty() {
                return Some(text);
            }
        }

        let mut cursor = current.walk();
        for child in current.named_children(&mut cursor) {
            stack.push(child);
        }
    }
    None
}

pub(super) fn make_extracted_symbol(
    name: String,
    kind: &str,
    exported: bool,
    scope: &[String],
    parent_identity: Option<&str>,
    language: CodeLanguage,
    source: &str,
    node: Node<'_>,
) -> ExtractedSymbol {
    let qualified_name = qualify_symbol_name(scope, &name);
    let (start_line, start_col, end_line, end_col) = node_span(node);
    let signature = extract_symbol_signature(source, language, node, kind, &name);
    let modifiers = extract_symbol_modifiers(source, language, node, kind);

    ExtractedSymbol {
        name,
        qualified_name: qualified_name.clone(),
        identity: format!("{}@{}:{}", qualified_name, start_line, start_col),
        parent_identity: parent_identity.map(|s| s.to_string()),
        kind: kind.to_string(),
        description: extract_symbol_description(source, language, node),
        modifiers,
        inputs: signature.inputs,
        output: signature.output,
        type_info: signature.type_info,
        exported,
        start_line,
        start_col,
        end_line,
        end_col,
    }
}

pub(super) fn qualify_symbol_name(scope: &[String], name: &str) -> String {
    if scope.is_empty() {
        name.to_string()
    } else {
        format!("{}::{}", scope.join("::"), name)
    }
}

pub(super) fn compare_extracted_symbols(
    a: &ExtractedSymbol,
    b: &ExtractedSymbol,
) -> std::cmp::Ordering {
    a.start_line
        .cmp(&b.start_line)
        .then_with(|| a.start_col.cmp(&b.start_col))
        .then_with(|| b.end_line.cmp(&a.end_line))
        .then_with(|| b.end_col.cmp(&a.end_col))
        .then_with(|| a.qualified_name.cmp(&b.qualified_name))
}
