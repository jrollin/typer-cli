# Typer CLI - Technology Stack

> **Purpose**: Documents chosen frameworks, libraries, development tools, and technical constraints
> **Type**: Steering Document - Technology decisions and rationale
> **Related**: See `product.md` for product vision, `structure.md` for how we organize code

## Language Choice

### Rust (2021 Edition)

**Why Rust:**
- **Performance**: Near-zero overhead for real-time input processing (<50ms latency requirement)
- **Terminal Control**: Excellent ecosystem for TUI applications
- **Type Safety**: Prevents entire classes of bugs at compile time
- **Memory Safety**: No garbage collection pauses that could affect typing feedback
- **Cross-platform**: Works on Linux, macOS, Windows with same codebase
- **Developer Experience**: Cargo provides excellent build, test, and dependency management

**Alternatives Considered:**
- **Go**: Good performance, but less mature TUI ecosystem
- **Python**: Rapid development, but performance concerns for real-time feedback
- **C/C++**: Maximum performance, but unsafe and complex for this use case

## Core Dependencies

### ratatui (0.29)
**Purpose**: Terminal UI framework

**Why ratatui:**
- Modern, actively maintained (fork of archived tui-rs)
- Declarative UI approach (similar to React)
- Excellent widget system for layouts
- Strong community and documentation
- Supports all major terminal backends

**What it provides:**
- Layout system (blocks, borders, panels)
- Text styling and coloring
- Frame rendering optimization
- Widget composition

**Alternatives Considered:**
- **cursive**: Higher-level but less flexible for custom layouts
- **termion**: Lower-level, more manual work required
- **tui-rs**: Archived, ratatui is the successor

### crossterm (0.28)
**Purpose**: Cross-platform terminal manipulation

**Why crossterm:**
- Cross-platform (Windows, macOS, Linux)
- Event handling (keyboard, mouse, terminal resize)
- Raw mode control
- Direct cursor manipulation
- ANSI color support

**What it provides:**
- Keyboard event capture (character-by-character input)
- Terminal mode switching (raw vs cooked)
- Screen clearing and cursor positioning
- Event polling for responsive UI

**Alternatives Considered:**
- **termion**: Unix-only, not cross-platform
- **pancurses**: C bindings, less Rust-idiomatic

### serde (1.0) + serde_json
**Purpose**: Serialization and deserialization

**Why serde:**
- Zero-cost serialization framework
- Type-safe JSON handling
- Derive macros reduce boilerplate
- Industry standard in Rust ecosystem

**What it provides:**
- Struct ↔ JSON conversion
- Automatic field serialization
- Human-readable stats files

**Usage:**
```rust
#[derive(Serialize, Deserialize)]
struct Stats {
    sessions: Vec<SessionRecord>,
}
```

### chrono (0.4)
**Purpose**: Date and time handling

**Why chrono:**
- Comprehensive time/date library
- Timezone support
- Duration calculations
- RFC 3339 timestamp formatting

**What it provides:**
- Session timestamps
- Duration tracking
- Human-readable time formatting

**Alternatives Considered:**
- **time**: Simpler, but less feature-complete
- **std::time**: Insufficient for timestamp formatting needs

### clap (4.5)
**Purpose**: Command-line argument parsing

**Why clap:**
- Derive-based API (ergonomic)
- Automatic help generation
- Subcommand support
- Shell completion generation

**Current Usage:**
- Prepared for future phases (Phase 1 has no CLI args)

**Future Usage:**
```bash
typer-cli --lesson homerow-3
typer-cli --stats
typer-cli --mode code
```

**Alternatives Considered:**
- **structopt**: Merged into clap 3.0
- **argh**: Simpler but less feature-complete

## Development Tools

### cargo
**Purpose**: Build system and package manager

**Usage:**
```bash
cargo build          # Development build
cargo build --release # Optimized build
cargo test           # Run test suite
cargo clippy         # Linting
cargo fmt            # Code formatting
```

### clippy
**Purpose**: Rust linter

**What it catches:**
- Common mistakes and anti-patterns
- Performance issues
- Idiomatic Rust violations
- Potential bugs

### rustfmt
**Purpose**: Code formatter

**Configuration**: Default settings (Rust style guide)

## Testing Strategy

### Unit Tests
**Framework**: Built-in `#[cfg(test)]` and `#[test]`

**Coverage:**
- `engine/scoring.rs`: WPM and accuracy calculations
- `engine/types.rs`: TypingSession logic
- `content/generator.rs`: Lesson generation
- `data/storage.rs`: JSON persistence
- `keyboard/azerty.rs`: Layout definitions

**Current Status**: 29 passing tests

### Integration Tests
**Approach**: End-to-end session tests

**What's tested:**
- Complete typing session flow
- Stats persistence across restarts
- Input validation and feedback

### Test Data
- Deterministic test cases
- Edge cases (empty sessions, 100% accuracy, 0% accuracy)
- Performance benchmarks for input processing

## Build Configuration

### Debug Profile
```toml
[profile.dev]
opt-level = 0
debug = true
```
**Use for**: Development, testing, debugging

### Release Profile
```toml
[profile.release]
opt-level = 3
lto = true
codegen-units = 1
```
**Use for**: Production builds, benchmarking

## File Structure Rationale

### Modular Architecture
```
src/
├── main.rs          # Minimal entry point
├── app.rs           # Application state and event loop
├── ui/              # UI rendering (separation of concerns)
├── engine/          # Core business logic (testable)
├── content/         # Lesson generation (extensible)
├── data/            # Persistence (swappable backend)
└── keyboard/        # Layout definitions (multi-layout support)
```

**Benefits:**
- **Testability**: Business logic isolated from UI
- **Extensibility**: Easy to add new lesson types or layouts
- **Maintainability**: Clear boundaries and responsibilities
- **Reusability**: Engine can be used in other contexts

## Performance Considerations

### Input Latency Target: <50ms
**Approach:**
- Direct terminal input (no buffering)
- Minimal processing per keystroke
- Pre-generated lesson content
- Efficient ratatui rendering

### Memory Usage
**Target**: <10MB resident memory

**Approach:**
- No large data structures
- Streaming stats to disk
- Minimal history retention in memory

### Binary Size
**Target**: <5MB release binary

**Current**: ~2.8MB (stripped release build)

## Platform Support

### Tested Platforms
- **Linux**: Primary development platform (Manjaro, Ubuntu)
- **macOS**: Secondary testing
- **Windows**: Supported via crossterm, but lower priority

### Terminal Compatibility
- **Minimum**: ANSI color support, cursor control
- **Tested**: kitty, alacritty, gnome-terminal, iTerm2, Windows Terminal
- **Not supported**: Very old terminals without ANSI escape codes

## Data Storage

### Configuration Directory
**Location**: `~/.config/typer-cli/` (XDG Base Directory standard)

**Why XDG:**
- Standard on Linux
- Clean separation from home directory
- Easy to backup/reset

### Stats File Format
**Format**: JSON (human-readable)
**File**: `~/.config/typer-cli/stats.json`

**Why JSON:**
- Human-readable for debugging
- Easy to parse with external tools
- Serde provides automatic serialization
- Future-proof for schema evolution

**Alternatives Considered:**
- **SQLite**: Overkill for simple append-only data
- **Binary format**: Not human-readable, harder to debug
- **TOML/YAML**: No significant advantage over JSON

## Future Technology Considerations

### Phase 2+
- **Graphing**: Consider `plotters` for terminal-based graphs
- **Colors**: May add `colored` crate for more sophisticated color schemes
- **Fuzzy matching**: If adding lesson search/selection

### Extensibility
- **Plugin system**: Consider `libloading` for dynamic lesson loading
- **Themes**: JSON-based theme definitions
- **Custom layouts**: JSON keyboard layout definitions

## Security Considerations

### Input Validation
- Terminal input is inherently safe (limited to printable chars)
- No external input beyond keyboard
- Stats file written to user's own config directory

### Dependencies
- All dependencies from crates.io (trusted source)
- Regular `cargo audit` for known vulnerabilities
- Minimal dependency tree to reduce attack surface

## Build and Release Process

### Development Workflow
```bash
cargo check           # Fast compilation check
cargo test            # Run test suite
cargo clippy          # Lint
cargo run             # Run in debug mode
```

### Release Workflow
```bash
cargo build --release          # Optimized build
strip target/release/typer-cli # Remove debug symbols
cargo test --release           # Test optimized build
```

### Distribution (Future)
- **Source**: GitHub repository
- **Binary releases**: GitHub Releases
- **Package managers**: AUR (Arch), cargo install, Homebrew (macOS)

## Development Environment

### Recommended IDE/Editor
- **VS Code** + rust-analyzer
- **Neovim/Vim** + rust-analyzer (LSP)
- **IntelliJ IDEA** + Rust plugin

### Required Tools
- **Rust**: 1.70+ (via rustup)
- **Cargo**: Included with Rust
- **Git**: Version control

### Optional Tools
- **cargo-watch**: Auto-rebuild on file changes
- **cargo-audit**: Security vulnerability scanning
- **hyperfine**: Benchmarking startup time
