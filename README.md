# Typer CLI

[![CI](https://github.com/jrollin/typer-cli/workflows/CI/badge.svg)](https://github.com/jrollin/typer-cli/actions/workflows/ci.yml)
[![Release](https://img.shields.io/github/v/release/jrollin/typer-cli)](https://github.com/jrollin/typer-cli/releases)
[![License](https://img.shields.io/badge/license-Personal-blue.svg)](LICENSE)

A Rust CLI application for learning and improving touch typing on AZERTY keyboards.

## Description

Typer CLI is a terminal-based keyboard typing training tool. It provides an interactive TUI (Terminal User Interface) for practicing touch typing with immediate visual feedback and real-time statistics.

## Features

### Core Features (Phase 1-2) ✅
- ✅ **Home Row Lessons**: 6 progressive levels for AZERTY home row
- ✅ **Bigram Training**: French, English, and Code patterns (9 lessons)
- ✅ **Code Symbols**: TypeScript, Rust, Python practice (18 lessons)
- ✅ **Real-time Feedback**: Green/red coloring for each character
- ✅ **Live Metrics**: WPM (words per minute) and accuracy
- ✅ **Session Persistence**: Automatic saving to JSON
- ✅ **Minimal TUI**: Clean interface with ratatui
- ✅ **AZERTY Keyboard**: Optimized for French layout

### Adaptive Mode (Phase 2+) ✅ NEW
- ✅ **Analytics Tracking**: Automatic per-key and per-bigram statistics
- ✅ **Weakness Detection**: Identifies keys < 80% accuracy or slow speed
- ✅ **Personalized Content**: 60% weak, 30% moderate, 10% strong key distribution
- ✅ **Spaced Repetition**: Practice intervals based on mastery level
- ✅ **Smart Appearance**: Shows in menu after 10+ sessions
- ✅ **Privacy-Focused**: All data stored locally

**Total Lessons**: 34 (33 standard + 1 adaptive)

## Installation

### Download Pre-built Binary (Recommended)

Download the latest release for your platform from [GitHub Releases](https://github.com/jrollin/typer-cli/releases):

```bash
# Linux (x86_64)
wget https://github.com/jrollin/typer-cli/releases/latest/download/typer-cli-linux-x86_64
chmod +x typer-cli-linux-x86_64
./typer-cli-linux-x86_64

# macOS (Intel)
wget https://github.com/jrollin/typer-cli/releases/latest/download/typer-cli-macos-x86_64
chmod +x typer-cli-macos-x86_64
./typer-cli-macos-x86_64

# macOS (Apple Silicon M1/M2/M3)
wget https://github.com/jrollin/typer-cli/releases/latest/download/typer-cli-macos-arm64
chmod +x typer-cli-macos-arm64
./typer-cli-macos-arm64
```

**Verify Checksum** (optional but recommended):
```bash
# Download checksum file
wget https://github.com/jrollin/typer-cli/releases/latest/download/typer-cli-linux-x86_64.sha256

# Verify
shasum -a 256 -c typer-cli-linux-x86_64.sha256
```

### Build from Source

Prerequisites:
- Rust 1.70+ (install via [rustup](https://rustup.rs/))

```bash
# Clone the repository
git clone https://github.com/jrollin/typer-cli.git
cd typer-cli

# Build in release mode
cargo build --release

# Run
./target/release/typer-cli
```

## Usage

### Launch the application

```bash
# Development mode
cargo run

# Or with release executable
./target/release/typer-cli
```

### Controls

**In Menu:**
- **↑/↓ or j/k**: Navigate lessons
- **Enter or Space**: Start selected lesson
- **1-9**: Quick select lesson by number
- **ESC or q**: Quit

**During Practice:**
- **Typing**: Simply type the displayed characters
- **Backspace**: Correct mistakes
- **ESC**: Return to menu

**After Completion:**
- **q or ESC**: Return to menu
- **r**: Restart same lesson

### Interface

```
┌─────────────────────────────────┐
│  TYPER CLI - Home Row Practice  │
├─────────────────────────────────┤
│  Text to type:                  │
│  ff jj ff jj dd kk              │
│                                 │
│  Your input:                    │
│  ff jj f█                       │
│                                 │
├─────────────────────────────────┤
│  WPM: 45  │  Accuracy: 98%     │
│  Time: 00:15                    │
└─────────────────────────────────┘
```

- **Green**: Correct character
- **Red**: Incorrect character
- **Gray**: Not yet typed
- **Blinking cursor**: Current position

## Technical Stack

- **Language**: Rust 2021
- **TUI**: [ratatui](https://github.com/ratatui-org/ratatui) + [crossterm](https://github.com/crossterm-rs/crossterm)
- **Serialization**: [serde](https://serde.rs/) + serde_json
- **Timestamps**: [chrono](https://github.com/chronotope/chrono)
- **Randomization**: [rand](https://github.com/rust-random/rand) (for adaptive content)
- **CLI args**: [clap](https://github.com/clap-rs/clap) (prepared for future)

## Architecture

```
src/
├── main.rs           # Entry point + terminal init
├── app.rs            # App state + event loop
├── ui/
│   └── render.rs     # TUI rendering with ratatui
├── engine/
│   ├── types.rs      # TypingSession, CharInput
│   ├── scoring.rs    # WPM and accuracy calculation
│   ├── analytics.rs  # Per-key/bigram statistics tracking (NEW)
│   └── adaptive.rs   # Weakness detection, spaced repetition (NEW)
├── content/
│   ├── lesson.rs            # Lesson definitions
│   ├── generator.rs         # Home row content generation
│   ├── bigram_generator.rs  # Bigram practice content
│   ├── code_generator.rs    # Code symbols content
│   └── adaptive_generator.rs # Personalized content (NEW)
├── data/
│   ├── stats.rs      # Stats structures (with adaptive analytics)
│   └── storage.rs    # JSON persistence
└── keyboard/
    └── azerty.rs     # Layout AZERTY
```

## Data

Session statistics are saved to:
```
~/.config/typer-cli/stats.json
```

JSON format:
```json
{
  "sessions": [
    {
      "timestamp": "2024-01-01T12:00:00Z",
      "lesson_type": "Home Row - Level 1",
      "wpm": 45.0,
      "accuracy": 95.5,
      "duration": 60000
    }
  ],
  "adaptive_analytics": {
    "key_stats": {
      "f": {
        "key": "f",
        "total_attempts": 100,
        "correct_attempts": 97,
        "error_count": 3,
        "total_time_ms": 10000,
        "mistype_map": {},
        "last_practiced": "2024-01-01T12:00:00Z",
        "mastery_level": "Mastered"
      }
    },
    "bigram_stats": {},
    "session_history": [],
    "total_sessions": 15,
    "total_keystrokes": 850
  }
}
```

**Note**: `adaptive_analytics` appears after your first session and enables personalized training after 10+ sessions.

## Development

### Tests

```bash
# Run all tests
cargo test

# With output
cargo test -- --nocapture

# Test adaptive mode
cargo run --example create_test_stats  # Generate test data
cargo run --example verify_adaptive     # Verify configuration
```

**81 unit tests** cover:
- Engine (typing session, scoring, analytics, adaptive algorithms)
- Content (lesson generation, bigrams, code symbols, adaptive content)
- Data (stats persistence with analytics)
- Keyboard (AZERTY layout)

**Test Categories:**
- 13 tests: typing session logic
- 9 tests: analytics tracking
- 9 tests: adaptive algorithms (weakness detection, spaced repetition)
- 6 tests: adaptive content generation
- 12 tests: bigram training
- 12 tests: code symbols
- 7 tests: home row lessons
- 7 tests: session storage
- 6 tests: additional coverage

### Code Quality

```bash
# Linter
cargo clippy

# Formatting
cargo fmt

# Check compilation
cargo check
```

### Commit Structure

See `tasks.md` for detailed tracking of implemented tasks.

## Roadmap

### Phase 1: MVP ✅ Completed
- [x] Home row basics (Level 1: f, j)
- [x] WPM + Accuracy calculations
- [x] Minimal TUI interface
- [x] Session persistence

### Phase 2: Content Expansion ✅ Completed
- [x] All home row lessons (Levels 1-6)
- [x] Bigram training (French, English, Code)
- [x] Code symbols (TypeScript, Rust, Python)
- [x] Lesson selection menu
- [x] Backspace support

### Phase 2+: Adaptive Mode ✅ Completed
- [x] Analytics engine (per-key/bigram tracking)
- [x] Weakness detection (accuracy & speed)
- [x] Spaced repetition algorithm
- [x] Adaptive content generation (60/30/10 distribution)
- [x] Automatic analytics after each session
- [x] Conditional menu appearance (≥10 sessions)

### Phase 3: Enhancements (Future)
- [ ] Enhanced adaptive UI (pre/post-session feedback)
- [ ] Progress visualization (heat maps, graphs)
- [ ] Data export (JSON/CSV)
- [ ] Keyboard visualization
- [ ] Gamification (achievements, streaks)
- [ ] Theme customization
- [ ] Multi-layout support (BÉPO, Dvorak)

## Documentation

### Workflow Documents
- `requirements.md` - EARS format requirements (what to build)
- `design.md` - Technical design and implementation details
- `tasks.md` - Detailed task tracking and progress

### Steering Documents
- `product.md` - Product vision, target users, and goals
- `tech.md` - Technology stack decisions and rationale
- `structure.md` - Architecture and module design patterns

### Project Context
- `CLAUDE.md` - Quick reference for AI assistants
- `README.md` - This file (user-facing documentation)

## License

This project is a personal learning tool.

## Author

Developed with the help of Claude (Anthropic).
