# Typer CLI

A Rust CLI application for learning and improving touch typing on AZERTY keyboards.

## Description

Typer CLI is a terminal-based keyboard typing training tool. It provides an interactive TUI (Terminal User Interface) for practicing touch typing with immediate visual feedback and real-time statistics.

## Features (MVP - Phase 1)

- ✅ **Home Row Mode**: Training on the home row (f, j)
- ✅ **Real-time Feedback**: Green/red coloring for each character
- ✅ **Live Metrics**: WPM (words per minute) and accuracy
- ✅ **Persistence**: Automatic session saving
- ✅ **Minimal Interface**: Clean TUI with ratatui
- ✅ **AZERTY Keyboard**: Optimized for French layout

## Installation

### Prerequisites

- Rust 1.70+ (install via [rustup](https://rustup.rs/))

### Build

```bash
# Clone the repo (if applicable)
cd typer-cli

# Build in release mode
cargo build --release

# Executable will be in target/release/typer-cli
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

- **Typing**: Simply type the displayed characters
- **ESC**: Quit at any time
- **q**: Quit after a completed session
- **r**: Restart a new session after completion

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
- **CLI args**: [clap](https://github.com/clap-rs/clap) (prepared for future)

## Architecture

```
src/
├── main.rs           # Entry point + terminal init
├── app.rs            # App state + event loop
├── ui/
│   └── render.rs     # Rendu TUI avec ratatui
├── engine/
│   ├── types.rs      # TypingSession, CharInput, SessionResult
│   └── scoring.rs    # Calcul WPM et accuracy
├── content/
│   ├── lesson.rs     # Définition des leçons
│   └── generator.rs  # Génération de contenu
├── data/
│   ├── stats.rs      # Structures de stats
│   └── storage.rs    # Persistence JSON
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
      "lesson_type": "HomeRow-1",
      "wpm": 45.0,
      "accuracy": 95.5,
      "duration": 60
    }
  ]
}
```

## Development

### Tests

```bash
# Run all tests
cargo test

# With output
cargo test -- --nocapture
```

29 unit tests cover:
- Engine (WPM calculation, accuracy, session)
- Content (lesson generation)
- Data (stats, persistence)
- Keyboard (AZERTY layout)

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

### Phase 1: MVP ✅ (Completed)
- [x] Home row basics
- [x] WPM + Accuracy
- [x] Minimal UI
- [x] Persistence

### Phase 2: Extension (Future)
- [ ] All home row lessons (levels 2-6)
- [ ] Bigrams mode (FR/EN)
- [ ] Code mode (programming symbols)
- [ ] Keyboard visualization
- [ ] English support

### Phase 3: Advanced (Future)
- [ ] Adaptive mode (focus on errors)
- [ ] Progress graphs
- [ ] Detailed history
- [ ] Backspace support
- [ ] CLI args for lesson selection

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
