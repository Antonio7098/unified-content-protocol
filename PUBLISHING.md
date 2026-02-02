# Publishing to crates.io

This guide explains how to publish the UCP CLI and its dependencies to crates.io.

## Prerequisites

1. **crates.io Account**: You need an account on [crates.io](https://crates.io)
2. **API Token**: Generate an API token from your [crates.io settings](https://crates.io/settings/tokens)
3. **Git configured**: Ensure git user.name and user.email are set

## Setup Steps

### 1. Login to crates.io

```bash
cargo login <YOUR_CRATES_IO_TOKEN>
```

Verify login:
```bash
cargo whoami
```

### 2. Verify Package Metadata

Check that all the following are correct:

- `Cargo.toml` files have proper metadata
- Version numbers are correct
- README files exist for each crate
- LICENSE file is present

### 3. Test Publishing (Dry Run)

Before actually publishing, run the dry-run to catch any issues:

```bash
./publish.sh --dry-run
```

This will:
- Verify all packages can be packaged
- Check dependencies are available
- Validate metadata

### 4. Publish to crates.io

Once dry-run passes, publish for real:

```bash
./publish.sh
```

This script will:
1. Publish crates in dependency order
2. Wait between publishes for indexing
3. Verify each publish succeeds

## Manual Publishing

If you prefer to publish manually, follow this order:

```bash
# 1. Core crates (no internal deps)
cargo publish -p ucm-core
cargo publish -p ucp-observe
cargo publish -p ucp-translator-markdown
cargo publish -p ucp-translator-html

# 2. Engine (depends on core + markdown)
cargo publish -p ucm-engine

# 3. Parser (depends on core + engine)
cargo publish -p ucl-parser

# 4. LLM utilities (depends on core)
cargo publish -p ucp-llm

# 5. Agent (depends on core + engine + parser)
cargo publish -p ucp-agent

# 6. CLI (depends on everything above)
cargo publish -p ucp-cli
```

Wait 10-30 seconds between each publish for crates.io to index.

## Version Management

### Before Publishing

1. Update version in root `Cargo.toml`:
```toml
[workspace.package]
version = "0.1.11"  # Bump this
```

2. Update `CHANGELOG.md` with new version details

3. Commit changes:
```bash
git add Cargo.toml CHANGELOG.md
git commit -m "Bump version to 0.1.11"
git tag v0.1.11
git push origin main --tags
```

### Version Bump Script

You can use this helper to bump versions:

```bash
#!/bin/bash
NEW_VERSION=$1

if [ -z "$NEW_VERSION" ]; then
    echo "Usage: $0 <version>"
    exit 1
fi

# Update workspace version
sed -i "s/^version = \".*\"$/version = \"$NEW_VERSION\"/" Cargo.toml

# Commit and tag
git add Cargo.toml
git commit -m "Bump version to $NEW_VERSION"
git tag "v$NEW_VERSION"

echo "Version bumped to $NEW_VERSION"
echo "Run: git push origin main --tags"
```

## Troubleshooting

### "Authentication failed"
- Run `cargo login` again with a fresh token
- Check token has `publish` scope

### "Package already exists"
- You can't republish the same version
- Bump the version number and try again

### "Dependency not found"
- Dependencies must be published first
- Wait a bit longer between publishes (crates.io indexing delay)
- Run `cargo update` to refresh index

### "Failed to verify package"
- Ensure all files are committed to git
- Check `Cargo.toml` has all required fields
- Verify README.md and LICENSE exist

### "Unauthorized"
- You may not be an owner of the crate
- Contact existing owners to add you

## Post-Publish

After publishing:

1. **Verify installation works**:
```bash
cargo install ucp-cli
ucp --version
```

2. **Update documentation**:
- Update installation instructions
- Update version badges

3. **Create GitHub release**:
- Go to [GitHub Releases](https://github.com/unified-content/ucp/releases)
- Create new release from the tag
- Add release notes

## Crate Ownership

To add additional owners:

```bash
cargo owner --add username -p ucp-cli
cargo owner --add username -p ucm-core
# etc for other crates
```

To list owners:

```bash
cargo owner --list -p ucp-cli
```

## Yanking (Emergency Only)

If you need to yank a broken version:

```bash
cargo yank -p ucp-cli --version 0.1.10
```

To unyank:

```bash
cargo yank -p ucp-cli --version 0.1.10 --undo
```

## Questions?

- See [crates.io publishing guide](https://doc.rust-lang.org/cargo/reference/publishing.html)
- Check [semver guidelines](https://semver.org/)
