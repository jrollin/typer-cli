# Typer CLI

[![CI](https://github.com/jrollin/typer-cli/workflows/CI/badge.svg)](https://github.com/jrollin/typer-cli/actions/workflows/ci.yml)
[![Release](https://img.shields.io/github/v/release/jrollin/typer-cli)](https://github.com/jrollin/typer-cli/releases)
[![License](https://img.shields.io/badge/license-Personal-blue.svg)](LICENSE)

A Rust CLI application for learning and improving touch typing on AZERTY keyboards.

## Description

Typer CLI is a terminal-based keyboard typing training tool. It provides an interactive TUI (Terminal User Interface) for practicing touch typing with immediate visual feedback and real-time statistics.

## Features

- **Progressive Lessons**: 77 lessons from home row basics to advanced code symbols
- **Real-time Feedback**: Instant visual feedback (green/red) and live WPM/accuracy metrics
- **Adaptive Mode**: Personalized training targeting your weaknesses
- **Visual Keyboard**: Full AZERTY layout with next-key highlighting, AltGr support, and heatmaps
- **AltGr Support**: Full support for code symbols ({}[]@#|`\^~) on AZERTY keyboards
- **Comprehensive Practice**: French/English bigrams, trigrams, TypeScript/Rust/Python symbols
- **Statistics Dashboard**: Track progress with performance analytics
- **Finger Training**: Targeted drills for each finger pair
- **Session Persistence**: All data saved locally for privacy

See [docs/README.md#features-overview](docs/README.md#features-overview) for complete feature list and implementation details.

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

**In Category Menu:**
- **↑/↓ or j/k**: Navigate categories
- **Enter or Space**: Open category
- **1-5**: Quick select category by number
- **ESC or q**: Quit application

**In Lesson Menu:**
- **↑/↓ or j/k**: Navigate lessons
- **Enter or Space**: Select lesson
- **1-9**: Quick select lesson by number
- **ESC or q**: Return to category menu

**During Practice:**
- **Typing**: Simply type the displayed characters
- **Backspace**: Correct mistakes
- **Tab**: Toggle keyboard display visibility
- **Ctrl+F**: Toggle finger color hints on keyboard
- **Ctrl+H**: Toggle accuracy heatmap overlay
- **ESC**: Return to menu

**In Duration Menu:**
- **↑/↓ or j/k**: Navigate durations
- **Enter or Space**: Start lesson
- **ESC or q**: Return to lesson menu

**After Completion:**
- **q or ESC**: Return to lesson menu
- **r**: Restart same lesson

### Interface

The interface features a clean, organized layout with consistent margins:

```
  ┌─────────────────────────────────┐
  │  TYPER CLI - Home Row Practice  │  ← Header
  ├─────────────────────────────────┤
  │ WPM: 45 │ Accuracy: 98% │ 00:15 │  ← Stats
  ├─────────────────────────────────┤
  │  Text to type:                  │
  │  ff jj ff jj dd kk              │  ← Content
  │  Your input:                    │
  │  ff jj f█                       │
  ├─────────────────────────────────┤
  │  [AZERTY Keyboard Layout]       │  ← Keyboard (Tab to toggle)
  │  Next key highlighted           │
  │  Tab │ Ctrl+H │ Ctrl+F          │
  │                                 │
  │         ESC to quit             │  ← Instructions
  └─────────────────────────────────┘
```

**Visual Feedback:**
- **Green**: Correct character
- **Red**: Incorrect character
- **White**: Not yet typed
- **Blinking cursor**: Current position
- **Cyan highlight**: Next key to press (on keyboard)

**Keyboard Display Features:**
- Full AZERTY layout visualization
- Real-time next-key highlighting
- Shift state indicators
- Optional finger color hints (Ctrl+F)
- Optional accuracy heatmap overlay (Ctrl+H)
- Toggle visibility with Tab

## Technical Stack

Built with **Rust** using [ratatui](https://github.com/ratatui-org/ratatui) for the terminal UI.

See [docs/steering/tech.md](docs/steering/tech.md) for complete stack details.

## Architecture

Modular Rust codebase organized by feature:
- `src/engine/` - Core typing engine, analytics, adaptive algorithms
- `src/content/` - Lesson generation and categorization
- `src/ui/` - Terminal interface rendering
- `src/data/` - Statistics persistence
- `src/keyboard/` - AZERTY layout model

See [docs/steering/structure.md](docs/steering/structure.md) for detailed architecture.

## Data

Session statistics are saved to `~/.config/typer-cli/stats.json`.

The file contains session history and adaptive analytics for personalized training.
See [docs/features/session-storage/](docs/features/session-storage/) for data schema details.

## Development

### Build and Run
```bash
cargo run              # Launch application
cargo build --release  # Build optimized binary
```

### Tests
```bash
cargo test             # Run test suite (129 passing tests)
```

### Code Quality
```bash
cargo clippy           # Linting
cargo fmt              # Formatting
cargo check            # Fast compilation check
```

See [docs/README.md](docs/README.md) for contributor workflow and detailed test coverage.

## Status

**Current Version**: Phase 3.5 Complete (Statistics Dashboard)
**Total Lessons**: 77
**Test Coverage**: 129 passing tests

See [docs/README.md#project-status](docs/README.md#project-status) for detailed roadmap and planned features.

## Documentation

- **User Guide**: You're reading it (installation, usage, controls)
- **Contributing**: See [docs/README.md](docs/README.md) for development workflow
- **Features**: See [docs/README.md#features-overview](docs/README.md#features-overview)
- **AI Context**: See [CLAUDE.md](CLAUDE.md)

Feature-specific documentation follows the Requirements → Design → Tasks workflow.
See [docs/features/](docs/features/) for individual feature documentation.

## License

This project is a personal learning tool.

## Author

Developed with the help of Claude (Anthropic).
