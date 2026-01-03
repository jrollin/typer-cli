# Typer CLI - AI Assistant Context

Quick reference for AI assistants working on this project.

## Quick Reference Links

- **User Documentation**: See `README.md` for installation and usage
- **Contributor Guide**: See `docs/README.md` for development workflow
- **Feature Details**: See `docs/features/<feature-name>/` for requirements/design/tasks

## Current Phase

**Phase 1: MVP** - ✅ Completed
**Phase 2: Home Row Levels 2-6** - ✅ Completed
**Phase 2+: Adaptive Mode** - ✅ Completed
**Phase 3: Visual Keyboard Display** - ✅ Completed
**Phase 3.1: Layout Improvements** - ✅ Completed
**Phase 3.2: Finger Training** - ✅ Completed
**Phase 3.3: Two-Level Menu System** - ✅ Completed
**Phase 3.4: Menu Grouping** - ✅ Completed
**Phase 3.5: Statistics Dashboard** - ✅ Completed
**Phase 3.6: AltGr Modifier Support** - ✅ Completed
**Phase 3.7: Custom Lessons** - ✅ Completed
**Current Focus**: Phase 3+ (Analytics visualization, data export, gamification)

## Dead Code Annotations

The codebase contains **29 intentional `#[allow(dead_code)]` annotations** marking Phase 3+ features that are:
- Fully tested and ready for future implementation
- Strategic placeholders for planned features (analytics, export, multi-layout support)
- Preserved to avoid reimplementation work

These are NOT technical debt - they represent well-architected future functionality.

**Categories**:
- Analytics & Statistics APIs (6): BigramStats methods, Stats methods, Storage.get_path()
- Lesson Metadata (3): title, description, keys - used in menu display
- Frequency Data (4): Linguistic research data for weighted practice
- Code Categorization (2): SymbolCategory for future UI filtering
- Keyboard Abstraction (3): AzertyLayout for multi-layout support
- UI Placeholders (3): CursorWindow fields for scrolling/pagination
- Test Utilities (3): Reusable drill generation patterns
- Adaptive Features (2): WeaknessDetector (fully tested, ready for UI)
- Scoring Utilities (1): calculate_accuracy() reference implementation
- Custom Lesson Validation (1): ParseError::InvalidFrontMatter for future strict YAML validation

## Project Overview

Terminal-based typing trainer for AZERTY keyboards with real-time feedback and code-focused practice.

## Documentation Structure

This project uses feature-based documentation. See [docs/README.md](docs/README.md) for:
- Requirements → Design → Tasks workflow
- Steering documents (product, tech, structure)
- Feature documentation organization

**77 lessons** across 6 categories: Adaptive, Finger Training, Key Training, Languages, Code, Custom

| Feature | Module | Status |
|---------|--------|--------|
| Typing Engine | `src/engine/` | ✅ Phase 1 |
| Home Row Lessons | `src/content/generator.rs` | ✅ Phase 2 |
| Bigram/Trigram Training | `src/content/bigram_generator.rs` | ✅ Phase 2 |
| Code Symbols | `src/content/code_generator.rs` | ✅ Phase 2 |
| Adaptive Mode | `src/engine/analytics.rs`, `adaptive.rs` | ✅ Phase 2+ |
| Visual Keyboard | `src/ui/keyboard.rs` | ✅ Phase 3.6 (with AltGr) |
| Finger Training | `src/content/finger_generator.rs` | ✅ Phase 3.2 |
| Two-Level Menu | `src/content/category.rs`, `app.rs` | ✅ Phase 3.3 |
| Statistics Dashboard | `src/ui/render.rs` | ✅ Phase 3.5 |
| Custom Lessons | `src/content/custom.rs` | ✅ Phase 3.7 |

See [docs/README.md#features-overview](docs/README.md#features-overview) for detailed feature descriptions.

## Quick Commands

```bash
# Development
cargo run              # Launch application
cargo test             # Run test suite (146 tests)
cargo check            # Fast compilation check

# Testing Adaptive Mode
cargo run --example create_test_stats  # Generate test data
cargo run --example verify_adaptive     # Verify adaptive configuration

# Quality
cargo clippy           # Linting
cargo fmt              # Code formatting

# Release
cargo build --release  # Optimized build

# Release (use automated script)
./scripts/release.sh 0.8.0                           # Bumps version, updates changelog, creates tag
git push origin main && git push origin v0.8.0       # Push to trigger automated release workflow
```

## CI/CD Workflows

### Continuous Integration
Runs on every push to `main` and all PRs: formatting, linting, tests (146 passing), security audit.

### Release Automation
Triggers on git tag `v*.*.*`:
- Generates changelog with git-cliff (conventional commits)
- Builds cross-platform binaries (Linux, macOS Intel/ARM64)
- Creates GitHub Release with binaries and checksums

**Creating a Release:**
```bash
# Use the automated release script
./scripts/release.sh 0.8.0

# Then push to trigger the release workflow
git push origin main && git push origin v0.8.0
```

The script handles:
- Version validation and duplicate tag checking
- Cargo.toml and Cargo.lock updates
- Complete CHANGELOG.md regeneration
- Commit with conventional message
- Tag creation

See [docs/steering/release-process.md](docs/steering/release-process.md) for detailed instructions.

See `.github/workflows/` for complete workflow details.

## Project Structure

```
src/
├── main.rs          # Entry point
├── app.rs           # App state machine, two-level navigation, event loop
├── ui/              # TUI rendering
│   ├── render.rs    # Category menu, lesson menu, layout rendering
│   └── keyboard.rs  # Visual keyboard display
├── engine/          # Session logic, scoring, analytics
│   ├── analytics.rs # Per-key/bigram statistics tracking
│   ├── adaptive.rs  # Weakness detection, spaced repetition
│   ├── scoring.rs   # WPM and accuracy calculations
│   └── types.rs     # TypingSession and CharInput
├── content/         # Lesson generation and organization
│   ├── category.rs            # Lesson categories, filtering
│   ├── adaptive_generator.rs  # Personalized content
│   ├── bigram_generator.rs    # Bigram practice
│   ├── code_generator.rs      # Code symbols
│   ├── custom.rs              # User-provided markdown lessons
│   ├── finger_generator.rs    # Finger-based drills
│   ├── lesson.rs              # Lesson types, definitions
│   └── generator.rs           # Home row drills
├── data/            # Stats persistence (with adaptive analytics)
└── keyboard/        # AZERTY layout and data model
```

## Stack Summary

- **Language**: Rust 2021
- **TUI**: ratatui + crossterm
- **Persistence**: serde + serde_json
- **Randomization**: rand (for adaptive content variety)
- **CLI**: clap (prepared for future phases)

## Key Constraints

- AZERTY keyboard only (Phase 1-2)
- French language only (Phase 1-2)
- Backspace support enabled (Phase 1+)
- Enter/newline support with `↵` icon visualization
- Random newlines in practice content (25% probability)
- No sound effects (all phases)
- Terminal-only (no GUI)

## Data Location

Stats saved to: `~/.config/typer-cli/stats.json`

Contains session history and adaptive analytics (per-key stats, bigram stats, mastery levels).

See [docs/features/session-storage/](docs/features/session-storage/) for complete schema.

## Current Status

**Phase**: 3.7 Complete (Custom Lessons)
**Next**: Phase 3+ (Analytics visualization, data export, gamification)
**Tests**: 146 passing
**Lessons**: 77 built-in + user custom lessons

See [docs/README.md#project-status](docs/README.md#project-status) for complete roadmap.
