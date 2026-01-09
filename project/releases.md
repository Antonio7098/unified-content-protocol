# Release Notes

Track SDK changes, publishing steps, and compatibility notes.

## Versioning Strategy

- **SDKs** use semantic versioning (`MAJOR.MINOR.PATCH`).
- **Docs** automatically reference `main`; tag releases to create version snapshots if required.
- Keep JS and Python package versions aligned when practical to signal parity.

## Upcoming Release Template

```markdown
## v0.x.x - YYYY-MM-DD

### Added
- ...

### Fixed
- ...

### Changed
- ...

### Publishing Checklist
- [ ] `npm version <patch|minor|major>` (JS)
- [ ] `npm publish --access public` (JS)
- [ ] `python -m build && twine upload dist/*` (Python)
- [ ] `git tag v0.x.x && git push origin v0.x.x`
- [ ] Update docs if new APIs shipped
```

Use GitHub releases to store artifacts and changelogs.
