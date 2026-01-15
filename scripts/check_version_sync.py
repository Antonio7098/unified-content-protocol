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
DOC_INSTALL_PATH = REPO_ROOT / "docs" / "getting-started" / "installation.md"
CHANGELOG_PATH = REPO_ROOT / "changelog.json"

README_PATTERN = re.compile(r"Latest release:\s*v(?P<version>[0-9][0-9A-Za-z.\-]*)")
DOC_PATTERNS: list[tuple[Path, re.Pattern[str], str]] = [
    (
        DOC_INSTALL_PATH,
        re.compile(r"ucp-api\s*=\s*\"(?P<version>[0-9][0-9A-Za-z.\-]*)\""),
        "ucp-api dependency example",
    ),
    (
        DOC_INSTALL_PATH,
        re.compile(r"ucm-core\s*=\s*\"(?P<version>[0-9][0-9A-Za-z.\-]*)\""),
        "ucm-core dependency example",
    ),
    (
        DOC_INSTALL_PATH,
        re.compile(r"ucm-engine\s*=\s*\"(?P<version>[0-9][0-9A-Za-z.\-]*)\""),
        "ucm-engine dependency example",
    ),
    (
        DOC_INSTALL_PATH,
        re.compile(r"ucl-parser\s*=\s*\"(?P<version>[0-9][0-9A-Za-z.\-]*)\""),
        "ucl-parser dependency example",
    ),
    (
        DOC_INSTALL_PATH,
        re.compile(r"ucp-translator-markdown\s*=\s*\"(?P<version>[0-9][0-9A-Za-z.\-]*)\""),
        "markdown translator dependency example",
    ),
    (
        DOC_INSTALL_PATH,
        re.compile(r"ucp-observe\s*=\s*\"(?P<version>[0-9][0-9A-Za-z.\-]*)\""),
        "ucp-observe dependency example",
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
    content = README_PATH.read_text(encoding="utf-8")
    match = README_PATTERN.search(content)
    if not match:
        return ["README.md is missing the latest release marker."]
    found = match.group("version")
    if found != version:
        return [f"README.md references v{found}, expected v{version}."]
    return []


def check_git_tag(version: str) -> list[str]:
    expected_tag = f"v{version}"
    try:
        tag = (
            subprocess.run(
                ["git", "describe", "--tags", "--exact-match", "HEAD"],
                cwd=REPO_ROOT,
                check=True,
                text=True,
                capture_output=True,
            )
            .stdout.strip()
        )
    except subprocess.CalledProcessError:
        return ["HEAD is not tagged. Create the release tag first or omit --require-tag."]
    if tag != expected_tag:
        return [f"HEAD tag is {tag}, expected {expected_tag}."]
    return []


def check_docs(version: str) -> list[str]:
    errors: list[str] = []
    content = DOC_INSTALL_PATH.read_text(encoding="utf-8")
    for path, pattern, label in DOC_PATTERNS:
        match = pattern.search(content)
        if not match:
            errors.append(f"{path.relative_to(REPO_ROOT)} is missing {label}.")
            continue
        found = match.group("version")
        if found != version:
            errors.append(
                f"{path.relative_to(REPO_ROOT)} {label} references {found}, expected {version}."
            )
    return errors


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

    if errors:
        for error in errors:
            print(f"[version-sync] {error}", file=sys.stderr)
        return 1

    if not args.quiet:
        print(f"[version-sync] All references match v{workspace_version}.")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
