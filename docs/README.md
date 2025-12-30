# Typer CLI Documentation

## Documentation Structure

This documentation follows a **feature-based organization** aligned with the global CLAUDE.md workflow. Each feature has its complete lifecycle tracked in its own folder.

## Workflow: Requirements → Design → Tasks

For each feature, we maintain three documents following this progression:

1. **requirements.md** - **WHAT** to build (EARS format)
   - Structured as: `WHEN [condition] THE SYSTEM SHALL [behavior]`
   - Clear, testable requirements
   - Future phase requirements marked as FUTURE

2. **design.md** - **HOW** to build it (technical architecture)
   - System architecture and data flows
   - Implementation details and algorithms
   - Design decisions and rationale
   - Testing strategy

3. **tasks.md** - **Progress tracking** (discrete tasks)
   - Implementation status (completed/pending)
   - Phase organization (Phase 1 MVP, Phase 2+, Phase 3+)
   - Implementation notes and decisions
   - Test coverage details

## Documentation Categories

### Steering Documents (Persistent Knowledge)

Located in `docs/steering/`:

- **[product.md](steering/product.md)** - Product vision, target users, key features, success metrics
- **[tech.md](steering/tech.md)** - Technology stack decisions, dependencies, build tools, rationale
- **[structure.md](steering/structure.md)** - Architecture patterns, module design, file organization

These documents represent persistent knowledge about the project that doesn't change with individual features.

### Feature Documentation

Located in `docs/features/<feature-name>/`:

Each feature folder contains the complete Requirements → Design → Tasks workflow.

## Features Overview

### Core Features (Phase 1 - MVP) ✓ COMPLETED

#### [typing-session/](features/typing-session/)
Core typing engine, scoring algorithms, and session management.

**Module**: `src/engine/`

- Character-by-character input validation
- Real-time WPM and accuracy calculation
- Session lifecycle management
- Performance: <50ms input latency

#### [home-row-lessons/](features/home-row-lessons/)
Content generation for AZERTY home row practice.

**Module**: `src/content/`

- Progressive difficulty (Level 1: f,j → Level 6: French words)
- Deterministic content generation
- Integration with keyboard layout
- Lesson selection menu with keyboard navigation
- All 6 levels with cumulative progression ✓ Completed

#### [session-storage/](features/session-storage/)
Statistics persistence for cross-session tracking.

**Module**: `src/data/`

- JSON storage at `~/.config/typer-cli/stats.json`
- SessionRecord: timestamp, lesson type, WPM, accuracy, duration
- XDG Base Directory compliance
- Graceful error handling

#### [tui-interface/](features/tui-interface/)
Terminal UI rendering with ratatui.

**Module**: `src/ui/`

- Minimal, distraction-free design
- Real-time visual feedback (green/red/gray)
- Live statistics panel
- Results screen with session summary

#### [keyboard-layout/](features/keyboard-layout/)
AZERTY layout definitions and finger mappings.

**Module**: `src/keyboard/`

- Home row: q, s, d, f, g, h, j, k, l, m
- Finger-to-key mapping
- Progressive key groups
- Extensible for future layouts (BÉPO, Dvorak)

### Planned Features (Phase 2+)

Comprehensive requirements and design documented in feature folders:

#### [bigram-training/](features/bigram-training/) ✅ COMPLETED
Practice common two-letter combinations for increased fluency.

**Module**: `src/content/bigram.rs`, `src/content/bigram_generator.rs`

- **3 languages**: French, English, Code
- **3 levels per language**: Drill → Word context → Realistic text
- **9 total lessons** (3 languages × 3 levels)
- French bigrams: qu, ou, en, on, an, es, ai, er, re, de
- English bigrams: th, he, in, er, an, re, on, at, en, ed
- Code bigrams: ->, ::, =>, !=, ==, <=, >=, &&, ||, //
- Frequency-ordered (most common first)
- Deterministic generation for consistent practice

#### [code-symbols/](features/code-symbols/) ✅ COMPLETED
Master programming symbols across different languages.

**Module**: `src/content/code_symbols.rs`, `src/content/code_generator.rs`

- **3 programming languages**: TypeScript, Rust, Python
- **6 levels per language**: Brackets → Operators → Comparisons → Arrows → Compound → Realistic
- **18 total lessons** (3 languages × 6 levels)
- TypeScript: Arrow functions (=>), type annotations (:), const/let
- Rust: Function arrows (->), match arms (=>), path separator (::), closures
- Python: List/dict comprehensions, f-strings, decorators (@), type hints
- Syntactically valid code snippets for each language
- Progressive difficulty from simple brackets to complex code

#### [adaptive-mode/](features/adaptive-mode/) ✅ COMPLETED (MVP)
Personalized training based on individual weaknesses.

**Modules**: `src/engine/analytics.rs`, `src/engine/adaptive.rs`, `src/content/adaptive_generator.rs`

- Per-key and per-bigram statistics tracking ✓
- Weakness detection (accuracy < 80%, speed > 75th percentile) ✓
- Spaced repetition algorithm (intervals by mastery level) ✓
- Adaptive content generation (60% weak, 30% moderate, 10% mastered) ✓
- Recommendation engine for next practice ✓
- Mastery level progression (Beginner → Learning → Proficient → Mastered) ✓
- Local data storage with backward compatibility ✓
- Automatic analytics tracking after each session ✓
- Conditional menu appearance (>= 10 sessions, >= 100 keystrokes) ✓

### Phase 3 Features (Completed) ✅

#### [keyboard-display/](features/keyboard-display/) ✅ COMPLETED
Visual AZERTY keyboard layout with real-time highlighting.

**Module**: `src/ui/keyboard.rs`, `src/keyboard/azerty.rs`

- Full 5-row keyboard rendering (Number, Top, Home, Bottom, Modifier rows)
- Real-time next-key highlighting (cyan background)
- Shift state indication (both shift keys highlight)
- Finger color hints toggle (Ctrl+F)
- Accuracy heatmap overlay toggle (Ctrl+H)
- Keyboard visibility toggle (Tab)

#### [finger-training/](features/finger-training/) ✅ COMPLETED
Targeted finger-pair practice with bilateral drills.

**Module**: `src/content/finger_generator.rs`

- 24 lessons: 4 finger pairs × 6 variants
- 3 difficulty levels: Home Row, Extended, All Keys
- Base and shift variants for each level
- Corrected French AZERTY finger mappings (16 fixes)
- Auto-generated drills with 3-phase pattern
- Prioritized menu placement (after adaptive mode)

#### [two-level-menu/](features/two-level-menu/) ✅ COMPLETED
Hierarchical navigation system for improved lesson discovery.

**Module**: `src/content/category.rs`, `src/app.rs`, `src/ui/render.rs`

- Two-screen navigation: Category selection → Lesson selection
- 5 lesson categories: Adaptive, Finger Training, Key Training, Languages, Code
- Category-based lesson filtering
- Visual descriptions and color coding
- Number shortcuts (1-5 for categories, 1-9 for lessons)
- ESC navigation: Lessons → Categories → Quit
- Context preservation after session completion

### Future Features (Phase 3+)

- **Analytics visualization** - Heat maps, trend graphs, progress charts
- **Enhanced adaptive UI** - Pre/post-session feedback, progress indicators
- **Data export** - JSON/CSV export for external analysis
- **Themes** - Multiple color schemes, high contrast options
- **Multi-layout** - BÉPO, Dvorak, custom keyboard layouts
- **Gamification** - Optional achievements, streaks, challenges
- **Advanced adaptive** - Machine learning, optimal schedules, fatigue detection

## Quick Navigation

### For New Contributors

1. **Understand the product**: Start with [`docs/steering/product.md`](steering/product.md)
2. **Understand the tech stack**: Read [`docs/steering/tech.md`](steering/tech.md)
3. **Understand the architecture**: Review [`docs/steering/structure.md`](steering/structure.md)
4. **Pick a feature**: Browse `docs/features/` for areas of interest

### Adding a New Feature

1. Create folder: `docs/features/feature-name/`
2. Write requirements: `requirements.md` (EARS format)
3. Design solution: `design.md` (architecture and implementation)
4. Track progress: `tasks.md` (discrete, trackable tasks)
5. Update this README with feature description

### Understanding an Existing Feature

Each feature folder contains:
- **requirements.md** - What problem does it solve?
- **design.md** - How is it implemented?
- **tasks.md** - What's done and what's planned?

### Finding Code

Module locations are documented in each feature's design.md:

- `src/app.rs` - Application state machine and two-level navigation
- `src/engine/` - Typing session core, analytics, adaptive algorithms
- `src/content/` - Lesson generation and categorization
  - `category.rs` - Category filtering (see [two-level-menu/design.md](features/two-level-menu/design.md))
- `src/data/` - Statistics persistence (see [session-storage/design.md](features/session-storage/design.md))
- `src/ui/` - Terminal interface
  - `render.rs` - Category menu, lesson menu, session rendering
  - `keyboard.rs` - Visual keyboard display
- `src/keyboard/` - Layout definitions and finger mappings

## Development Workflow

### When Adding Requirements

1. Identify which feature it belongs to
2. Add to `docs/features/<feature>/requirements.md` in EARS format
3. Update `design.md` if architecture changes
4. Add tasks to `tasks.md`

### When Implementing a Feature

1. Check `requirements.md` for what to build
2. Review `design.md` for how to build it
3. Update `tasks.md` as you complete work
4. Mark tasks as completed with [x]

### When Reviewing Code

1. Find the feature folder: `docs/features/<feature>/`
2. Verify implementation matches `requirements.md`
3. Check design patterns match `design.md`
4. Confirm `tasks.md` is up to date

## Project Status

**Current Phase**: Phase 3.3 Complete (Two-Level Menu System) ✓

**Total Tests**: 129 passing
- 13 tests: typing-session
- 7 tests: home-row-lessons
- 12 tests: bigram-training
- 12 tests: code-symbols
- 9 tests: analytics (NEW)
- 9 tests: adaptive algorithms (NEW)
- 6 tests: adaptive generator (NEW)
- 7 tests: session-storage
- 2 tests: keyboard-layout
- 3 tests: content generation
- 1 test: data structures

**Completed Features**:
- Phase 1: Home row Level 1 ✓
- Phase 2: Home row Levels 2-6 ✓
- Phase 2: Bigram training (French, English, Code) ✓
- Phase 2: Trigram training (French, English) ✓
- Phase 2: Common words (French, English) ✓
- Phase 2: Code symbols (TypeScript, Rust, Python) ✓
- Phase 2+: Adaptive Mode ✓
- Phase 3: Visual keyboard display ✓
- Phase 3.1: Layout improvements ✓
- Phase 3.2: Finger training ✓
- Phase 3.3: Two-level menu system ✓

**Total Lessons**: 77 (52 standard + 24 finger training + 1 adaptive)

**Next Phase**: Analytics visualization and data export (Phase 3+)

## Additional Resources

- **User documentation**: [`README.md`](../README.md) at project root
- **AI assistant context**: [`CLAUDE.md`](../CLAUDE.md) at project root
- **Global workflow**: `~/.claude/CLAUDE.md` (defines Requirements → Design → Tasks pattern)

## Questions?

- **Product questions**: See [`docs/steering/product.md`](steering/product.md)
- **Technical questions**: See [`docs/steering/tech.md`](steering/tech.md)
- **Architecture questions**: See [`docs/steering/structure.md`](steering/structure.md)
- **Feature-specific**: Check the feature's folder in `docs/features/`
