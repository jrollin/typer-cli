# Release Process

## Standard Release Workflow

### Prerequisites
- Clean working directory (no uncommitted changes)
- All tests passing on main: `cargo test`
- Version number decided (following semver)
- `git-cliff` installed: `cargo install git-cliff` (script can auto-install)

### Steps

1. **Run the release script**:
   ```bash
   ./scripts/release.sh 0.8.0
   ```

2. **Review the changes**:
   - Script shows modified files (Cargo.toml, Cargo.lock, CHANGELOG.md)
   - Commit created with message: `chore(release): bump version to 0.8.0`
   - Tag created: `v0.8.0`

3. **Push to trigger release**:
   ```bash
   git push origin main && git push origin v0.8.0
   ```

4. **Verify release**:
   - Check GitHub Actions for release workflow
   - Verify binaries built successfully
   - Confirm GitHub Release published

### What the Script Does

1. ✓ Validates version format (X.Y.Z)
2. ✓ Checks tag doesn't already exist
3. ✓ Verifies working directory is clean
4. ✓ Updates Cargo.toml version (`cargo set-version`)
5. ✓ Updates Cargo.lock (`cargo check`)
6. ✓ Regenerates complete CHANGELOG.md (`git cliff`)
7. ✓ Shows diff of changes
8. ✓ Commits with conventional message
9. ✓ Creates annotated tag
10. ✓ Prompts you to push

### Rollback Procedure

If you need to undo a release before pushing:

```bash
# Delete tag locally
git tag -d v0.8.0

# Undo commit
git reset --hard HEAD~1
```

If you already pushed:

```bash
# Delete remote tag
git push origin :v0.8.0

# Revert commit on main
git revert HEAD
git push origin main
```

## Manual Release (Not Recommended)

If you skip the script and push a tag directly:

```bash
git tag v0.8.0
git push origin v0.8.0
```

**Warning**: This creates a release but leaves Cargo.toml and CHANGELOG.md out of sync with the tag version.

## Troubleshooting

### "Version must be in semver format"
- Use X.Y.Z format (e.g., 0.8.0, 1.0.0, 2.1.3)
- Don't include 'v' prefix
- Example: `./scripts/release.sh 0.8.0` ✓
- Not: `./scripts/release.sh v0.8.0` ✗

### "Tag already exists"
- Check existing tags: `git tag --list | tail -10`
- Use next version number
- To delete existing tag: `git tag -d v0.8.0`

### "You have uncommitted changes"
- Commit or stash your changes first
- Ensure clean working directory: `git status`

### "git-cliff not found"
- Install: `cargo install git-cliff`
- Or let script auto-install (first run takes longer)

### Release workflow doesn't trigger after push
- Verify tag pushed: `git ls-remote --tags origin | grep v0.8.0`
- Check release.yml exists in `.github/workflows/`
- Check Actions tab for errors
