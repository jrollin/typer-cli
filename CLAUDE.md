# Typer CLI - AI Assistant Context

Quick reference for AI assistants working on this project.

## Current Phase

**Phase 1: MVP** - ✅ Completed
**Current Focus**: Polish (clippy, formatting, documentation)

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
- **home-row-lessons/** (`src/content/`) - Content generation for AZERTY home row
- **session-storage/** (`src/data/`) - Stats persistence to JSON
- **tui-interface/** (`src/ui/`) - Terminal UI with ratatui
- **keyboard-layout/** (`src/keyboard/`) - AZERTY layout definitions

**Complete documentation index**: See `docs/README.md` for navigation guide and feature details.

### User Documentation
- **README.md** (root) - User-facing documentation and setup

## Quick Commands

```bash
# Development
cargo run              # Launch application
cargo test             # Run test suite (29 tests)
cargo check            # Fast compilation check

# Quality
cargo clippy           # Linting
cargo fmt              # Code formatting

# Release
cargo build --release  # Optimized build
```

## Project Structure

```
src/
├── main.rs          # Entry point
├── app.rs           # App state, event loop
├── ui/render.rs     # TUI rendering
├── engine/          # Session logic, scoring
├── content/         # Lesson generation
├── data/            # Stats persistence
└── keyboard/        # AZERTY layout
```

## Stack Summary

- **Language**: Rust 2021
- **TUI**: ratatui + crossterm
- **Persistence**: serde + serde_json
- **CLI**: clap (prepared for Phase 2)

## Key Constraints

- AZERTY keyboard only (Phase 1)
- French language only (Phase 1)
- No backspace support (Phase 1)
- No sound effects (all phases)
- Terminal-only (no GUI)

## Data Location

Stats saved to: `~/.config/typer-cli/stats.json`

## Roadmap

- **Phase 1**: Home row practice ✅
- **Phase 2**: Bigrams, code symbols, all home row levels
- **Phase 3**: Adaptive mode, analytics, themes
