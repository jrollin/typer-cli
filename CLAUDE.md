# Typer CLI - AI Assistant Context

Quick reference for AI assistants working on this project.

## Current Phase

**Phase 1: MVP** - ✅ Completed
**Phase 2: Home Row Levels 2-6** - ✅ Completed
**Phase 2+: Adaptive Mode** - ✅ Completed
**Phase 3: Visual Keyboard Display** - ✅ Completed
**Phase 3.1: Layout Improvements** - ✅ Completed
**Phase 3.2: Finger Training** - ✅ Completed
**Phase 3.3: Two-Level Menu System** - ✅ Completed
**Phase 3.4: Menu Grouping** - ✅ Completed
**Current Focus**: Phase 3+ (Analytics visualization, data export, gamification)

## Dead Code Annotations

The codebase contains **28 intentional `#[allow(dead_code)]` annotations** marking Phase 3+ features that are:
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

See `~/.claude/plans/soft-wishing-mccarthy.md` for complete inventory.

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
- **bigram-training/** (`src/content/`) - French (40 with accents), English (30), Code (10) bigram practice
- **trigram-training/** (`src/content/`) - French (25 with accents), English (20) trigram practice
- **code-symbols/** (`src/content/`) - Programming symbols for TypeScript, Rust, Python

**Phase 2+ (Completed):**
- **adaptive-mode/** (`src/engine/analytics.rs`, `src/engine/adaptive.rs`, `src/content/adaptive_generator.rs`) - Personalized training with analytics
  - Per-key and per-bigram statistics tracking
  - Weakness detection (accuracy < 80%, speed > 75th percentile)
  - Spaced repetition algorithm
  - Adaptive content generation (60% weak, 30% moderate, 10% strong)
  - Automatic analytics tracking after each session
  - Appears in menu when ≥ 10 sessions completed

**Phase 3 (Completed):**
- **keyboard-display/** (`src/ui/keyboard.rs`, `src/keyboard/azerty.rs`) - Visual AZERTY keyboard layout
  - Full 5-row keyboard rendering (Number, Top, Home, Bottom, Modifier)
  - Real-time next-key highlighting (cyan background)
  - Shift state indication (both shift keys highlight)
  - Proper AZERTY layout with base characters
  - Modifier keys (Tab, Caps, Shift, Ctrl, Cmd, Option, Alt, Fn)
  - Enter key as arrow [←] on home row
  - Keyboard shortcuts:
    - Tab: Toggle keyboard visibility
    - Ctrl+F: Toggle finger color hints
    - Ctrl+H: Toggle accuracy heatmap overlay

**Phase 3.1 (Completed):**
- **layout-improvements/** (`src/ui/render.rs`) - Enhanced interface layout
  - Reorganized layout: Header → Stats → Content → Keyboard → Instructions
  - Stats block moved under header, before content
  - Keyboard positioned after content (not fixed to bottom)
  - Consistent margins (2 units) matching menu screens
  - "ESC to quit" instructions at bottom
  - Fixed keyboard shortcuts labels (Ctrl+F, Ctrl+H)
  - Smooth keyboard toggle without layout shifts

**Phase 3.2 (Completed):**
- **finger-training/** (`src/content/finger_generator.rs`) - Finger-specific practice lessons
  - 24 lessons: 4 finger pairs (Pinky, Ring, Middle, Index) × 6 variants
  - 3 difficulty levels: Home Row, Extended, All Keys
  - Base and shift variants for each level
  - Corrected French AZERTY finger mappings (16 fixes)
  - Auto-generated drills with 3-phase pattern
  - Shift drills use 50% lower, 40% upper, 10% symbols
  - Heatmap disabled by default (Ctrl+H to enable)

**Phase 3.3 (Completed):**
- **two-level-menu/** (`src/content/category.rs`, `src/app.rs`, `src/ui/render.rs`) - Hierarchical navigation system
  - Two-screen navigation: Category selection → Lesson selection
  - 5 lesson categories: Adaptive, Finger Training, Key Training, Languages, Code
  - Category-based lesson filtering (simplified menu rendering)
  - New `LessonCategoryType` enum and `LessonCategory` struct
  - Category metadata: name, description, color coding
  - Keyboard shortcuts: Numbers (1-5) for direct category selection
  - Navigation flow: Category menu → Filtered lessons → Duration → Session
  - ESC navigation: Lessons → Categories → Quit
  - Maintains category context after session completion

**Phase 3.4 (Completed):**
- **menu-grouping/** (`src/ui/render.rs`, `src/app.rs`) - Visual lesson grouping within categories
  - Category-aware lesson grouping with visual separators
  - **Languages category**: Grouped by language (French, English) with cyan separators
  - **Finger Training category**: Grouped by finger pair (Pinky, Ring, Middle, Index) with green separators
  - **Code category**: Grouped by type/language (Code Patterns, TypeScript, Rust, Python) with magenta separators
  - Separators use category colors for visual consistency
  - Automatic blank line spacing between groups
  - Lesson ordering optimized for logical grouping
  - Improves navigation and lesson discovery within large categories

**Phase 3.5 (Completed):**
- **statistics-page/** (`src/ui/render.rs`, `src/app.rs`) - Comprehensive performance dashboard ✅ NEW
  - Overall session statistics (sessions, keystrokes, WPM, accuracy)
  - Mastery level breakdown (counts per classification: Mastered, Proficient, Learning, Beginner)
  - Top 10 weaknesses list (< 80% accuracy with error details)
  - Common mistype patterns (top 5 error patterns showing which keys are confused)
  - Visual keyboard heatmap with accuracy-based color coding
  - Accessible from main menu with 's' key
  - Graceful placeholder when no analytics data exists
  - Two-column layout optimized for terminal display (40% stats / 60% heatmap)

**Complete documentation index**: See `docs/README.md` for navigation guide and feature details.

### User Documentation
- **README.md** (root) - User-facing documentation and setup

## Quick Commands

```bash
# Development
cargo run              # Launch application
cargo test             # Run test suite (129 tests)
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
- **Test Suite**: Runs 129 tests on Ubuntu and macOS
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
- **Phase 3**: Visual keyboard display ✅
  - Full AZERTY keyboard layout rendering (Number, Top, Home, Bottom, Modifier rows)
  - Real-time next-key highlighting and shift state indication
  - Keyboard shortcuts: Tab (toggle visibility), Ctrl+F (finger colors), Ctrl+H (heatmap)
  - Proper alignment and visual styling
- **Phase 3.1**: Layout improvements ✅
  - Reorganized interface layout (Header → Stats → Content → Keyboard → Instructions)
  - Stats repositioned under header
  - Keyboard flows after content (not fixed at bottom)
  - Consistent margins across all screens
  - Bottom instructions section ("ESC to quit")
  - Fixed keyboard shortcut labels
- **Phase 3.2**: Finger-based training ✅
  - 24 finger lessons (4 pairs × 6 variants: 3 levels × 2 modes)
  - Corrected French AZERTY finger mappings (16 fixes)
  - Auto-generated drills with 3-phase pattern
  - Shift variants with weighted distribution (50/40/10)
  - Heatmap disabled by default
- **Phase 3.3**: Two-level menu system ✅
  - Hierarchical navigation with category selection
  - 5 categories: Adaptive, Finger Training, Key Training, Languages, Code
  - Category-based lesson filtering
  - Improved UX with descriptions and color coding
  - ESC navigation flow: Lessons → Categories → Quit
- **Phase 3+**: ⏳ FUTURE
  - Enhanced adaptive UI (pre/post-session feedback, progress indicators)
  - Analytics visualization (performance graphs, trend charts)
  - Data export (JSON/CSV)
  - Gamification (achievements, streaks)
  - Themes and customization
