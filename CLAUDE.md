# Typer CLI - AI Assistant Context

Quick reference for AI assistants working on this project.

## Current Phase

**Phase 1: MVP** - ✅ Completed
**Phase 2: Home Row Levels 2-6** - ✅ Completed
**Phase 2+: Adaptive Mode** - ✅ Completed
**Current Focus**: Phase 3 (UI enhancements, analytics visualization)

## Project Overview

Terminal-based typing trainer for AZERTY keyboards with real-time feedback and code-focused practice.

## Documentation Structure

This project uses **feature-based documentation** organized in `docs/`:

### Workflow: Requirements → Design → Tasks

Each feature in `docs/features/<feature-name>/` contains:
- **requirements.md** - WHAT to build (EARS format: `WHEN [condition] THE SYSTEM SHALL [behavior]`)
- **design.md** - HOW to build it (technical architecture, algorithms, data flows)
- **tasks.md** - Implementation tracking (completed/pending tasks, organized by phase)

### Steering Documents (Persistent Knowledge)

Located in `docs/steering/`:
- **product.md** - Product vision, target users, key features, success metrics
- **tech.md** - Technology stack decisions, dependencies, rationale
- **structure.md** - Architecture patterns, module organization, design patterns

### Features Overview

**Phase 1 (Completed):**
- **typing-session/** (`src/engine/`) - Core typing engine, scoring, session management
- **session-storage/** (`src/data/`) - Stats persistence to JSON
- **tui-interface/** (`src/ui/`) - Terminal UI with ratatui
- **keyboard-layout/** (`src/keyboard/`) - AZERTY layout definitions

**Phase 2 (Completed):**
- **home-row-lessons/** (`src/content/`) - All 6 progressive home row levels with menu selection
- **bigram-training/** (`src/content/`) - French, English, and Code bigram practice
- **code-symbols/** (`src/content/`) - Programming symbols for TypeScript, Rust, Python

**Phase 2+ (Completed):**
- **adaptive-mode/** (`src/engine/analytics.rs`, `src/engine/adaptive.rs`, `src/content/adaptive_generator.rs`) - Personalized training with analytics ✅ NEW
  - Per-key and per-bigram statistics tracking
  - Weakness detection (accuracy < 80%, speed > 75th percentile)
  - Spaced repetition algorithm
  - Adaptive content generation (60% weak, 30% moderate, 10% strong)
  - Automatic analytics tracking after each session
  - Appears in menu when ≥ 10 sessions completed

**Complete documentation index**: See `docs/README.md` for navigation guide and feature details.

### User Documentation
- **README.md** (root) - User-facing documentation and setup

## Quick Commands

```bash
# Development
cargo run              # Launch application
cargo test             # Run test suite (81 tests)
cargo check            # Fast compilation check

# Testing Adaptive Mode
cargo run --example create_test_stats  # Generate test data
cargo run --example verify_adaptive     # Verify adaptive configuration

# Quality
cargo clippy           # Linting
cargo fmt              # Code formatting

# Release
cargo build --release  # Optimized build

# CI/CD
git cliff --latest     # Preview changelog for next release
git tag v0.2.0         # Create version tag (triggers release workflow)
git push origin v0.2.0 # Push tag to trigger automated release
```

## CI/CD Workflows

### Continuous Integration (`.github/workflows/ci.yml`)

Runs automatically on every push to `main` and all pull requests.

**Jobs:**
- **Quality Checks**: Formatting (cargo fmt), linting (cargo clippy -D warnings), compilation (cargo check)
- **Test Suite**: Runs 81 tests on Ubuntu and macOS
- **Security Audit**: Scans dependencies for known vulnerabilities (non-blocking)

**Status**: [![CI](https://github.com/jrollin/typer-cli/workflows/CI/badge.svg)](https://github.com/jrollin/typer-cli/actions/workflows/ci.yml)

### Release Automation (`.github/workflows/release.yml`)

Triggers on git tag push matching `v*.*.*` pattern.

**Process:**
1. Generate changelog using git-cliff (conventional commits)
2. Create GitHub Release with changelog as release notes
3. Build cross-platform binaries:
   - Linux x86_64 (`typer-cli-linux-x86_64`)
   - macOS Intel (`typer-cli-macos-x86_64`)
   - macOS ARM64 (`typer-cli-macos-arm64`)
4. Generate SHA256 checksums for all binaries
5. Upload binaries and checksums as release assets

**Creating a Release:**
```bash
# 1. Ensure all changes are committed and CI passes
git add . && git commit -m "feat: your feature description"

# 2. Create and push version tag
git tag v0.2.0
git push origin v0.2.0

# 3. Monitor release workflow at:
# https://github.com/jrollin/typer-cli/actions

# 4. Release will be available at:
# https://github.com/jrollin/typer-cli/releases
```

### Changelog Generation (`cliff.toml`)

Uses [git-cliff](https://git-cliff.org) to generate conventional commit-based changelogs.

**Commit Types:**
- `feat:` → Features section
- `fix:` → Bug Fixes section
- `docs:` → Documentation section
- `perf:` → Performance section
- `refactor:` → Refactoring section
- `test:` → Testing section
- `chore:`, `ci:` → Miscellaneous Tasks section

**Preview changelog locally:**
```bash
# Install git-cliff
cargo install git-cliff

# Preview next release
git cliff --latest --strip header

# Generate full changelog
git cliff -o CHANGELOG.md
```

## Project Structure

```
src/
├── main.rs          # Entry point
├── app.rs           # App state, event loop
├── ui/render.rs     # TUI rendering
├── engine/          # Session logic, scoring, analytics
│   ├── analytics.rs # Per-key/bigram statistics tracking
│   ├── adaptive.rs  # Weakness detection, spaced repetition
│   ├── scoring.rs   # WPM and accuracy calculations
│   └── types.rs     # TypingSession and CharInput
├── content/         # Lesson generation
│   ├── adaptive_generator.rs  # Personalized content
│   ├── bigram_generator.rs    # Bigram practice
│   ├── code_generator.rs      # Code symbols
│   └── generator.rs           # Home row drills
├── data/            # Stats persistence (with adaptive analytics)
└── keyboard/        # AZERTY layout
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
- No sound effects (all phases)
- Terminal-only (no GUI)

## Data Location

Stats saved to: `~/.config/typer-cli/stats.json`

**Stats Structure:**
- `sessions`: Array of completed session records
- `adaptive_analytics`: Optional analytics data (appears after first session)
  - `key_stats`: Per-key performance (accuracy, speed, mistypes, mastery level)
  - `bigram_stats`: Per-bigram performance
  - `total_sessions`: Session counter
  - `total_keystrokes`: Total keystrokes tracked

## Roadmap

- **Phase 1**: Home row practice (Level 1) ✅
- **Phase 2**: All home row levels (Levels 1-6) with lesson selection menu ✅
- **Phase 2**: Bigram training (French, English, Code patterns) ✅
- **Phase 2**: Code symbols (TypeScript, Rust, Python) ✅
- **Phase 2+**: Adaptive mode (personalized weak-key training) ✅
  - Analytics engine (per-key and per-bigram tracking)
  - Weakness detection (accuracy < 80%, speed > 75th percentile)
  - Spaced repetition algorithm (practice intervals by mastery level)
  - Adaptive content generation (60/30/10 distribution)
  - Automatic tracking after each session
  - Conditional menu appearance (≥ 10 sessions, ≥ 100 keystrokes)
- **Phase 3**: ⏳ FUTURE
  - Enhanced adaptive UI (pre/post-session feedback, progress indicators)
  - Analytics visualization (heat maps, trend graphs)
  - Data export (JSON/CSV)
  - Gamification (achievements, streaks)
  - Themes and customization
