# Pull Request Playbook for Coding Agents

Use this checklist whenever you prepare a pull request in this repository. Treat every item as mandatory unless the section explicitly says "optional".

## 1. Prep Work
1. Create a descriptive branch name (e.g., `feat/<summary>` or `fix/<bug>`) if you are not already on the correct one.
2. If you start a new branch, ensure your working tree is clean before starting: `git status -sb`.

## 2. Tests, Lint, and Coverage
Run **all** language toolchains. Fail fast if any command exits non‑zero.

```bash
# Rust
cargo fmt --all
cargo clippy --all-targets --all-features -- -D warnings
cargo test --all-features

# Python
pip install -r requirements-dev.txt  # if file exists
ruff check .
ruff format --check .
pytest --maxfail=1 --disable-warnings
pytest --cov=packages/ucp-python/src/ucp --cov-report=term-missing  # ensure coverage for new code

# JavaScript/TypeScript
npm install
npm run lint
npm run test
npm run test:coverage || npm run coverage  # whichever script exists; ensure coverage captured
```

If you introduce a feature lacking meaningful coverage, add/extend tests before continuing. Document any intentionally uncovered logic in the PR description (with justification).

## 3. Documentation Cross-Check
1. Review relevant files under `docs/` and `README.md`.
2. Confirm examples, API signatures, feature flags, and version numbers align with the code changes.
3. Fix inconsistencies immediately—documentation updates are part of the same PR.

## 4. Version Synchronization
Whenever functionality or public surface changes, bump versions **consistently**:
- `Cargo.toml` workspace version
- `packages/ucp-python/pyproject.toml` and `packages/ucp-python/src/ucp/__init__.py`
- `packages/ucp-js/package.json`
- Any version references in `README.md`, `docs/`, examples, scripts, etc.

Use semantic versioning. Only skip a bump for docs-only tweaks with zero code impact, and note the rationale in the PR.

## 5. Changelog Update
Use `python scripts/log_helper.py add` to add a new entry, which will:
1. Prompt for version (e.g. `v0.1.3`) and date (defaults to today).
2. Walk you through adding one or more change entries with type, area, description, commit, files affected, and optional issues.
3. Insert the entry at the top of `changelog.json` with proper structure.
4. Ensure descriptions match the code and docs modifications.

## 6. Final Validation
1. Re-run `python scripts/check_version_sync.py [--require-tag]` to confirm alignment.
2. `git status -sb` should show only intentional changes.
3. `git diff` for a final manual review.

## 7. Commit and PR
1. `git commit -am "<type>: <summary>"` (or stage granularly).
2. `git push -u origin <branch>`.
3. Open a PR with a detailed description covering:
   - Summary of functional changes
   - Tests & coverage results (paste key command outputs or summarize)
   - Docs updates
   - Version bump rationale
   - Release impact (e.g., “Tag vX.Y.Z after merge to trigger PyPI publish”)

## 8. Post-Merge Release
1. Wait for maintainer confirmation that the PR is merged.
2. Switch to `main`, pull latest, and verify the merge commit is present.
3. Create a tag matching the new version (e.g., `git tag vX.Y.Z`).
4. Push the tag: `git push origin vX.Y.Z` to trigger the publish workflow.
5. Monitor GitHub Actions → `publish-python` until version-check and publish jobs succeed.
6. Confirm artifacts (PyPI/npm/crates) show the new version.

## 9. Failure Handling
- If any CI step fails, diagnose, fix, and re-run locally before pushing again.
- If publish fails after tagging, fix the root cause, bump versions again if required, and repeat the tagging process.

Following this document keeps releases deterministic and auditable. Update `pull.md` whenever the process changes.
