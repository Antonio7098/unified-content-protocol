#!/usr/bin/env python3
"""Release helper for Unified Content Protocol."""

from __future__ import annotations

import argparse
import subprocess
import sys
from pathlib import Path

REPO_ROOT = Path(__file__).resolve().parents[1]
CARGO_TOML = REPO_ROOT / "Cargo.toml"
CHECK_SCRIPT = REPO_ROOT / "scripts" / "check_version_sync.py"


def run(
    cmd: list[str], *, dry_run: bool = False
) -> subprocess.CompletedProcess[str] | None:
    if dry_run:
        print(f"[dry-run] {' '.join(cmd)}")
        return None
    return subprocess.run(
        cmd, cwd=REPO_ROOT, check=True, text=True, capture_output=False
    )


def captured(cmd: list[str]) -> str:
    result = subprocess.run(
        cmd, cwd=REPO_ROOT, check=True, text=True, capture_output=True
    )
    return result.stdout.strip()


def ensure_clean_worktree() -> None:
    status = captured(["git", "status", "--porcelain"])
    if status:
        raise SystemExit(
            "Working tree has uncommitted changes. Commit or stash before releasing."
        )


def ensure_tag_absent(tag_name: str) -> None:
    tags = captured(["git", "tag", "--list", tag_name])
    if tags:
        raise SystemExit(f"Tag {tag_name} already exists. Delete or bump the version.")


def ensure_on_branch(expected: str | None) -> None:
    if expected is None:
        return
    branch = captured(["git", "rev-parse", "--abbrev-ref", "HEAD"])
    if branch != expected:
        raise SystemExit(
            f"Releases must run from {expected}, current branch is {branch}."
        )


def load_version() -> str:
    import tomllib

    with CARGO_TOML.open("rb") as fp:
        data = tomllib.load(fp)
    return data["workspace"]["package"]["version"]


def main(argv: list[str] | None = None) -> int:
    parser = argparse.ArgumentParser(
        description="Run release checks and optionally tag the repo."
    )
    parser.add_argument(
        "--branch", help="Require running from this branch.", default="main"
    )
    parser.add_argument(
        "--tag", action="store_true", help="Create and push a git tag after checks."
    )
    parser.add_argument(
        "--push", action="store_true", help="Push the created tag to origin."
    )
    parser.add_argument(
        "--dry-run", action="store_true", help="Print commands without executing."
    )
    args = parser.parse_args(argv)

    ensure_clean_worktree()
    ensure_on_branch(args.branch)

    version = load_version()
    tag_name = f"v{version}"
    ensure_tag_absent(tag_name)

    run([sys.executable, str(CHECK_SCRIPT)])

    if args.tag:
        run(["git", "tag", tag_name], dry_run=args.dry_run)
        if args.push:
            run(["git", "push", "origin", tag_name], dry_run=args.dry_run)

    print(f"Release checks complete for v{version}.")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
