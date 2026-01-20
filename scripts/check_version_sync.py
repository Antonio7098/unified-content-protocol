#!/usr/bin/env python3
"""Ensure core version references across UCP stay aligned."""

from __future__ import annotations

import argparse
import json
import re
import subprocess
import sys
from pathlib import Path

import tomllib

REPO_ROOT = Path(__file__).resolve().parents[1]
CARGO_TOML = REPO_ROOT / "Cargo.toml"
PYPROJECT_TOML = REPO_ROOT / "packages" / "ucp-python" / "pyproject.toml"
PACKAGE_JSON = REPO_ROOT / "packages" / "ucp-js" / "package.json"
README_PATH = REPO_ROOT / "README.md"
DOCS_DIR = REPO_ROOT / "docs"
DOC_INSTALL_PATH = DOCS_DIR / "getting-started" / "installation.md"
DOC_README_PATH = DOCS_DIR / "README.md"
DOC_UCM_CORE_PATH = DOCS_DIR / "ucm-core" / "README.md"
DOC_UCM_ENGINE_PATH = DOCS_DIR / "ucm-engine" / "README.md"
DOC_UCL_PARSER_PATH = DOCS_DIR / "ucl-parser" / "README.md"
DOC_UCP_API_PATH = DOCS_DIR / "ucp-api" / "README.md"
DOC_UCP_OBSERVE_PATH = DOCS_DIR / "ucp-observe" / "README.md"
DOC_TRANSLATOR_MD_PATH = DOCS_DIR / "translators" / "markdown" / "README.md"
CHANGELOG_PATH = REPO_ROOT / "changelog.json"

VERSION_CAPTURE = r"(?P<version>[0-9][0-9A-Za-z.\-]*)"

README_PATTERNS: list[tuple[re.Pattern[str], str]] = [
    (
        re.compile(rf"Latest release:\s*v{VERSION_CAPTURE}"),
        "latest release badge",
    ),
    (
        re.compile(rf"ucp-api\s*=\s*\"{VERSION_CAPTURE}\""),
        "Rust dependency example",
    ),
    (
        re.compile(rf"pip install ucp-content=={VERSION_CAPTURE}"),
        "Python install command",
    ),
    (
        re.compile(rf"npm install @ucp-core/core@{VERSION_CAPTURE}"),
        "JavaScript install command",
    ),
]
DOC_PATTERNS: list[tuple[Path, re.Pattern[str], str]] = [
    (
        DOC_INSTALL_PATH,
        re.compile(rf"ucp-api\s*=\s*\"{VERSION_CAPTURE}\""),
        "getting-started ucp-api dependency example",
    ),
    (
        DOC_INSTALL_PATH,
        re.compile(rf"ucm-core\s*=\s*\"{VERSION_CAPTURE}\""),
        "getting-started ucm-core dependency example",
    ),
    (
        DOC_INSTALL_PATH,
        re.compile(rf"ucm-engine\s*=\s*\"{VERSION_CAPTURE}\""),
        "getting-started ucm-engine dependency example",
    ),
    (
        DOC_INSTALL_PATH,
        re.compile(rf"ucl-parser\s*=\s*\"{VERSION_CAPTURE}\""),
        "getting-started ucl-parser dependency example",
    ),
    (
        DOC_INSTALL_PATH,
        re.compile(rf"ucp-translator-markdown\s*=\s*\"{VERSION_CAPTURE}\""),
        "getting-started markdown translator dependency example",
    ),
    (
        DOC_INSTALL_PATH,
        re.compile(rf"ucp-observe\s*=\s*\"{VERSION_CAPTURE}\""),
        "getting-started ucp-observe dependency example",
    ),
    (
        DOC_INSTALL_PATH,
        re.compile(rf"ucm-core\s*=\s*\"{VERSION_CAPTURE}\"\s*\nucm-engine"),
        "getting-started version conflict snippet",
    ),
    (
        DOC_README_PATH,
        re.compile(rf"ucp-api\s*=\s*\"{VERSION_CAPTURE}\""),
        "docs README installation snippet",
    ),
    (
        DOC_UCM_CORE_PATH,
        re.compile(rf"ucm-core\s*=\s*\"{VERSION_CAPTURE}\""),
        "ucm-core README dependency example",
    ),
    (
        DOC_UCM_ENGINE_PATH,
        re.compile(rf"ucm-engine\s*=\s*\"{VERSION_CAPTURE}\""),
        "ucm-engine README dependency example",
    ),
    (
        DOC_UCL_PARSER_PATH,
        re.compile(rf"ucl-parser\s*=\s*\"{VERSION_CAPTURE}\""),
        "ucl-parser README dependency example",
    ),
    (
        DOC_UCP_API_PATH,
        re.compile(rf"ucp-api\s*=\s*\"{VERSION_CAPTURE}\""),
        "ucp-api README dependency example",
    ),
    (
        DOC_UCP_OBSERVE_PATH,
        re.compile(rf"ucp-observe\s*=\s*\"{VERSION_CAPTURE}\""),
        "ucp-observe README dependency example",
    ),
    (
        DOC_TRANSLATOR_MD_PATH,
        re.compile(rf"ucp-translator-markdown\s*=\s*\"{VERSION_CAPTURE}\""),
        "markdown translator README dependency example",
    ),
]


def load_workspace_version() -> str:
    with CARGO_TOML.open("rb") as fp:
        data = tomllib.load(fp)
    try:
        return data["workspace"]["package"]["version"]
    except KeyError as exc:  # pragma: no cover
        raise SystemExit("Cargo.toml is missing [workspace.package].version") from exc


def load_python_version() -> str:
    with PYPROJECT_TOML.open("rb") as fp:
        data = tomllib.load(fp)
    try:
        return data["project"]["version"]
    except KeyError as exc:  # pragma: no cover
        raise SystemExit("pyproject.toml is missing [project].version") from exc


def load_js_version() -> str:
    content = PACKAGE_JSON.read_text(encoding="utf-8")
    data = json.loads(content)
    version = data.get("version")
    if not isinstance(version, str):  # pragma: no cover
        raise SystemExit("package.json is missing a string version field")
    return version


def check_readme(version: str) -> list[str]:
    errors: list[str] = []
    content = README_PATH.read_text(encoding="utf-8")
    for pattern, label in README_PATTERNS:
        match = pattern.search(content)
        if not match:
            errors.append(f"README.md is missing the {label}.")
            continue
        found = match.group("version")
        if found != version:
            errors.append(
                f"README.md {label} references v{found}, expected v{version}."
            )
    return errors


def check_git_tag(version: str) -> list[str]:
    expected_tag = f"v{version}"
    try:
        tag = subprocess.run(
            ["git", "describe", "--tags", "--exact-match", "HEAD"],
            cwd=REPO_ROOT,
            check=True,
            text=True,
            capture_output=True,
        ).stdout.strip()
    except subprocess.CalledProcessError:
        return [
            "HEAD is not tagged. Create the release tag first or omit --require-tag."
        ]
    if tag != expected_tag:
        return [f"HEAD tag is {tag}, expected {expected_tag}."]
    return []


def check_docs(version: str) -> list[str]:
    errors: list[str] = []
    dup_pattern = re.compile(
        rf'"(?P<dup_version>[0-9][0-9A-Za-z.\-]*)"[0-9][0-9A-Za-z.\-]*'
    )
    cache: dict[Path, str] = {}
    for path, pattern, label in DOC_PATTERNS:
        if path not in cache:
            if not path.exists():
                errors.append(f"{path.relative_to(REPO_ROOT)} is missing.")
                continue
            cache[path] = path.read_text(encoding="utf-8")
        content = cache[path]
        match = pattern.search(content)
        if not match:
            errors.append(f"{path.relative_to(REPO_ROOT)} is missing {label}.")
            continue
        found = match.group("version")
        if found != version:
            errors.append(
                f"{path.relative_to(REPO_ROOT)} {label} references {found}, expected {version}."
            )
        # Check for duplicated version pattern (e.g., "0.1.3"0.1.3)
        full_match = match.group(0)
        dup_match = dup_pattern.search(full_match)
        if dup_match and len(dup_match.group(0)) > len(f'"{found}"'):
            errors.append(
                f"{path.relative_to(REPO_ROOT)} has duplicated version in {label}: {full_match}"
            )
    return errors


def fix_docs(version: str) -> list[str]:
    """Rewrite doc files to align version references."""
    fixed: list[str] = []
    cache: dict[Path, str] = {}
    for path, pattern, label in DOC_PATTERNS:
        if path not in cache:
            if not path.exists():
                continue
            cache[path] = path.read_text(encoding="utf-8")
        content = cache[path]
        match = pattern.search(content)
        if not match:
            continue
        found = match.group("version")
        if found == version:
            continue
        # Replace only the captured version group
        new_content = pattern.sub(
            lambda match, new_version=version: match.group(0).replace(
                match.group("version"), new_version, 1
            ),
            content,
        )
        cache[path] = new_content
        path.write_text(new_content, encoding="utf-8")
        fixed.append(
            f"{path.relative_to(REPO_ROOT)} updated {label}: {found} → {version}"
        )
    return fixed


def fix_readme(version: str) -> list[str]:
    """Rewrite README badge and snippet references."""
    if not README_PATH.exists():
        return ["README.md is missing."]

    fixed: list[str] = []
    content = README_PATH.read_text(encoding="utf-8")
    original_content = content

    for pattern, label in README_PATTERNS:
        match = pattern.search(content)
        if not match:
            continue
        found = match.group("version")
        if found == version:
            continue
        content = pattern.sub(
            lambda match, new_version=version: match.group(0).replace(
                match.group("version"), new_version, 1
            ),
            content,
        )
        fixed.append(f"README.md updated {label}: {found} → {version}")

    if content != original_content:
        README_PATH.write_text(content, encoding="utf-8")

    return fixed


def check_changelog(version: str) -> list[str]:
    if not CHANGELOG_PATH.exists():
        return ["changelog.json is missing."]
    with CHANGELOG_PATH.open(encoding="utf-8") as fp:
        data = json.load(fp)
    entries = data.get("entries", [])
    if not entries:
        return ["changelog.json has no entries."]
    latest = entries[0]
    recorded = latest.get("version")
    expected = f"v{version}"
    if recorded != expected:
        return [f"changelog.json latest entry is {recorded}, expected {expected}."]
    return []


def main(argv: list[str] | None = None) -> int:
    parser = argparse.ArgumentParser(
        description="Validate that workspace, SDKs, docs, and changelog share the same version."
    )
    parser.add_argument("--quiet", action="store_true", help="Suppress success output.")
    parser.add_argument(
        "--require-tag",
        action="store_true",
        help="Ensure HEAD is tagged as v<version>. Useful for release workflows.",
    )
    parser.add_argument(
        "--fix",
        action="store_true",
        help="Automatically rewrite doc files to align version references.",
    )
    args = parser.parse_args(argv)

    errors: list[str] = []

    workspace_version = load_workspace_version()
    python_version = load_python_version()
    js_version = load_js_version()

    if python_version != workspace_version:
        errors.append(
            f"packages/ucp-python/pyproject.toml version {python_version} does not match {workspace_version}."
        )
    if js_version != workspace_version:
        errors.append(
            f"packages/ucp-js/package.json version {js_version} does not match {workspace_version}."
        )

    errors.extend(check_readme(workspace_version))
    errors.extend(check_docs(workspace_version))
    errors.extend(check_changelog(workspace_version))
    if args.require_tag:
        errors.extend(check_git_tag(workspace_version))

    if args.fix:
        fixed: list[str] = []
        fixed.extend(fix_readme(workspace_version))
        fixed.extend(fix_docs(workspace_version))
        for line in fixed:
            print(f"[version-sync] {line}")
        # Re-check after fixing
        errors = []
        errors.extend(check_readme(workspace_version))
        errors.extend(check_docs(workspace_version))
        errors.extend(check_changelog(workspace_version))
        if args.require_tag:
            errors.extend(check_git_tag(workspace_version))

    if errors:
        for error in errors:
            print(f"[version-sync] {error}", file=sys.stderr)
        return 1

    if not args.quiet:
        print(f"[version-sync] All references match v{workspace_version}.")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
