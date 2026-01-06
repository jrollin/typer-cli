# Typer CLI - Structure and Architecture

> **Purpose**: Outlines file organization, naming conventions, import patterns, and architectural decisions
> **Type**: Steering Document - How the codebase is organized
> **Related**: See `product.md` for product goals, `tech.md` for technology choices

## Project Structure

```
typer-cli/
├── src/
│   ├── main.rs              # Entry point, terminal initialization
│   ├── app.rs               # Application state machine, two-level navigation, event loop
│   ├── ui/
│   │   ├── mod.rs           # UI module exports
│   │   ├── render.rs        # Category menu, lesson menu, TUI rendering
│   │   └── keyboard.rs      # Visual keyboard display with highlighting
│   ├── engine/
│   │   ├── mod.rs           # Engine module exports
│   │   ├── types.rs         # Core types: TypingSession, CharInput, SessionResult
│   │   ├── scoring.rs       # WPM and accuracy calculation algorithms
│   │   ├── analytics.rs     # Per-key/bigram statistics tracking
│   │   └── adaptive.rs      # Weakness detection, spaced repetition
│   ├── content/
│   │   ├── mod.rs           # Content module exports
│   │   ├── category.rs      # Lesson categories, filtering logic
│   │   ├── lesson.rs        # Lesson type definitions and enums
│   │   ├── generator.rs     # Home row content generation
│   │   ├── bigram_generator.rs    # Bigram practice
│   │   ├── trigram_generator.rs   # Trigram practice
│   │   ├── code_generator.rs      # Code symbols
│   │   ├── finger_generator.rs    # Finger-based drills
│   │   ├── common_word_generator.rs # Common words practice
│   │   └── adaptive_generator.rs  # Personalized content
│   ├── data/
│   │   ├── mod.rs           # Data module exports
│   │   ├── stats.rs         # Stats and SessionRecord structures (with analytics)
│   │   └── storage.rs       # JSON persistence (load/save)
│   └── keyboard/
│       ├── mod.rs           # Keyboard module exports
│       └── azerty.rs        # AZERTY layout definition with finger mapping
├── docs/
│   ├── README.md            # Documentation index
│   ├── features/            # Feature-based documentation
│   │   ├── two-level-menu/
│   │   │   ├── requirements.md  # EARS format requirements
│   │   │   ├── design.md        # Technical architecture
│   │   │   └── tasks.md         # Implementation tracking
│   │   └── [other features]/
│   └── steering/            # Persistent knowledge
│       ├── product.md       # Product vision and goals
│       ├── tech.md          # Technology stack decisions
│       └── structure.md     # This file
├── Cargo.toml               # Dependencies and project metadata
├── CLAUDE.md                # Lightweight AI assistant context
└── README.md                # User-facing documentation
```

## Module Architecture

### Dependency Graph
```
main.rs
  └─> app.rs
       ├─> ui/
       │    ├─> render.rs (category menu, lesson menu, session rendering)
       │    │    ├─> engine/types.rs
       │    │    └─> content/category.rs
       │    └─> keyboard.rs (visual keyboard display)
       │         └─> keyboard/azerty.rs
       ├─> engine/
       │    ├─> types.rs (TypingSession, CharInput, SessionResult)
       │    ├─> scoring.rs (uses types.rs)
       │    ├─> analytics.rs (key/bigram statistics)
       │    └─> adaptive.rs (weakness detection, spaced repetition)
       ├─> content/
       │    ├─> category.rs (LessonCategory, filtering)
       │    ├─> lesson.rs (LessonType enum, definitions)
       │    ├─> generator.rs (home row generation)
       │    ├─> bigram_generator.rs (uses lesson.rs)
       │    ├─> trigram_generator.rs
       │    ├─> code_generator.rs
       │    ├─> finger_generator.rs (uses keyboard/azerty.rs)
       │    ├─> common_word_generator.rs
       │    └─> adaptive_generator.rs (uses analytics.rs)
       ├─> data/
       │    ├─> stats.rs (SessionRecord, AdaptiveAnalytics)
       │    └─> storage.rs (uses stats.rs)
       └─> keyboard/
            └─> azerty.rs (layout definition, finger mapping)
```

### Module Responsibilities

#### `main.rs`
**Responsibility**: Application entry point

**What it does:**
- Initializes terminal (raw mode, alternate screen)
- Creates App instance
- Starts main event loop
- Handles terminal cleanup on exit

**What it doesn't do:**
- Business logic
- UI rendering
- Event handling (delegated to App)

#### `app.rs`
**Responsibility**: Application state machine and event orchestration

**Core struct:**
```rust
pub struct App {
    session: Option<TypingSession>,
    state: AppState,  // LessonTypeMenu, LessonMenu, DurationMenu, Running, Completed, Quit
    selected_category: usize,
    categories: Vec<LessonCategory>,
    current_category: Option<LessonCategoryType>,
    selected_lesson: usize,
    lessons: Vec<Lesson>,
    // ... stats, storage, keyboard config
}
```

**State Machine:**
```
LessonTypeMenu → LessonMenu (filtered) → DurationMenu → Running → Completed
     ↑                ↑                      ↑
     └─ ESC ──────────┘────── ESC ──────────┘
```

**What it does:**
- Manages two-level navigation (category → lesson)
- Filters lessons by selected category
- Routes keyboard events to appropriate handlers
- Converts relative lesson indices to absolute for execution
- Manages application lifecycle and state transitions
- Preserves category context after sessions

**What it doesn't do:**
- Rendering (delegates to ui/render.rs)
- Scoring calculations (delegates to engine/scoring.rs)
- Content generation (delegates to content generators)

#### `ui/render.rs`
**Responsibility**: Terminal UI rendering

**What it does:**
- Renders category selection menu with descriptions and colors
- Renders filtered lesson menu for selected category
- Renders duration selection menu
- Renders active typing session layout
- Renders results screen
- Applies color coding (green/red/gray, category colors)
- Formats statistics display
- Creates TUI widgets (blocks, paragraphs, spans, lists)

**What it doesn't do:**
- State management
- Event handling
- Scoring calculations
- Lesson filtering (receives filtered list from app)

**Key functions:**
- `render_lesson_type_menu()`: Category selection screen
- `render_menu()`: Filtered lesson selection (accepts category name)
- `render_duration_menu()`: Duration selection
- `render()`: Main typing interface
- `render_results()`: End-of-session results screen
- Color scheme: Green (correct), Red (incorrect), Gray (pending), category-specific colors

#### `ui/keyboard.rs`
**Responsibility**: Visual keyboard display

**What it does:**
- Renders full AZERTY keyboard layout (5 rows)
- Highlights next key to press (cyan background)
- Indicates shift state on both shift keys
- Displays finger color hints (toggle with Ctrl+F)
- Shows accuracy heatmap overlay (toggle with Ctrl+H)
- Supports compact and full keyboard modes

**What it doesn't do:**
- Input handling
- Key mapping logic (uses keyboard/azerty.rs)
- State management

#### `engine/types.rs`
**Responsibility**: Core typing session domain model

**Key types:**
```rust
pub struct TypingSession {
    content: String,
    current_index: usize,
    inputs: Vec<CharInput>,
    start_time: Instant,
    end_time: Option<Instant>,
}

pub struct CharInput {
    expected: char,
    typed: char,
    timestamp: Duration,
    is_correct: bool,
}

pub struct SessionResult {
    wpm: f64,
    accuracy: f64,
    duration: Duration,
    char_count: usize,
    error_count: usize,
}
```

**What it does:**
- Manages typing session lifecycle
- Records each character input
- Determines session completion
- Validates character input
- Provides session results

**Design decisions:**
- Immutable session content (set at creation)
- Append-only inputs (no backspace in Phase 1)
- Lazy result calculation (only when requested)

#### `engine/scoring.rs`
**Responsibility**: Metric calculations

**What it does:**
- Calculates WPM: `(char_count / 5) / (duration_seconds / 60)`
- Calculates accuracy: `(correct_chars / total_chars) × 100`
- Provides real-time and final metrics

**Design decisions:**
- Pure functions (no side effects)
- Independent of UI
- Well-tested with edge cases

#### `content/category.rs`
**Responsibility**: Lesson categorization and filtering

**Key types:**
```rust
pub enum LessonCategoryType {
    Adaptive,
    FingerTraining,
    RowTraining,
    Languages,
    Code,
    Custom,
}

pub struct LessonCategory {
    pub category_type: LessonCategoryType,
    pub name: &'static str,
    pub description: &'static str,
    pub color: Color,
}
```

**What it does:**
- Defines 6 lesson categories with metadata
- Provides filtering logic via `contains_lesson()`
- Generates category list with `all(has_adaptive: bool)`
- Maps lesson types to categories

**Category Filtering:**
- Adaptive: `LessonType::Adaptive`
- FingerTraining: `LessonType::FingerPair { .. }`
- RowTraining: `LessonType::RowProgression { .. }`
- Languages: `BigramType::Natural | Trigram | CommonWords`
- Code: `CodeSymbols | BigramType::Code`
- Custom: `LessonType::Custom { .. }`

#### `content/lesson.rs`
**Responsibility**: Lesson type definitions

**Key types:**
```rust
pub enum LessonType {
    KeyPair { lesson_id: u8 },
    KeyPairGroup { group_id: u8, with_shift: bool },
    Bigram { bigram_type: BigramType, language: Option<Language>, level: u8 },
    Trigram { language: Language, level: u8 },
    CommonWords { language: Language, level: u8 },
    CodeSymbols { language: ProgrammingLanguage, level: u8 },
    FingerPair { finger_pair: FingerPair, level: u8, with_shift: bool },
    Adaptive,
}

pub struct Lesson {
    pub lesson_type: LessonType,
    pub title: String,
}
```

**What it does:**
- Defines all available lesson types
- Provides lesson metadata (title, type)
- Factory methods for creating lesson collections
- Implements ContentGenerator trait for lesson execution

#### `content/generator.rs`
**Responsibility**: Practice content generation

**What it does:**
- Generates character sequences for lessons
- Implements progressive difficulty
- Uses keyboard layout for key selection

**Lesson generation strategies:**
- Level 1: Alternating f/j patterns
- Level 2-4: Progressive finger addition
- Level 5: All home row keys
- Level 6: French words using home row

**Design decisions:**
- Deterministic generation (same lesson = same content)
- Appropriate length (~50-100 chars per lesson)
- Space-separated for WPM calculation

#### `data/stats.rs`
**Responsibility**: Statistics data structures

**Key types:**
```rust
pub struct Stats {
    pub sessions: Vec<SessionRecord>,
}

pub struct SessionRecord {
    pub timestamp: DateTime<Utc>,
    pub lesson_type: String,
    pub wpm: f64,
    pub accuracy: f64,
    pub duration: u64,
}
```

**What it does:**
- Defines serializable stats format
- Provides helper methods for stats access

#### `data/storage.rs`
**Responsibility**: File system persistence

**What it does:**
- Loads stats from `~/.config/typer-cli/stats.json`
- Saves stats to JSON file
- Creates config directory if needed
- Handles file I/O errors gracefully

**Design decisions:**
- XDG Base Directory compliance
- Human-readable JSON format
- Create directory on first run
- Fail gracefully if stats can't be loaded

#### `keyboard/azerty.rs`
**Responsibility**: Keyboard layout definitions

**What it does:**
- Defines AZERTY home row: `qsdfghjklm`
- Provides key grouping by finger

**Design decisions:**
- Extensible to full keyboard layout
- Separated from lesson logic
- Easy to add BÉPO, Dvorak, etc. in future

## Data Flow

### Session Lifecycle

```
1. Application Start
   └─> main.rs initializes terminal
       └─> app.rs creates TypingSession
           └─> content/generator.rs generates lesson content
               └─> keyboard/azerty.rs provides key set

2. User Types Character
   └─> main.rs captures keyboard event (crossterm)
       └─> app.rs handles_key_event()
           └─> TypingSession.process_input()
               ├─> Validates character
               ├─> Records CharInput
               └─> Checks if complete

3. Render Frame
   └─> main.rs triggers render
       └─> app.rs provides current state
           └─> ui/render.rs renders UI
               └─> engine/scoring.rs calculates metrics
                   └─> Displayed to user

4. Session Complete
   └─> TypingSession marks complete
       └─> app.rs sets show_results = true
           └─> ui/render.rs renders results screen
               └─> engine/scoring.rs provides final metrics

5. Save Stats
   └─> app.rs creates SessionRecord
       └─> data/storage.rs saves to JSON
           └─> data/stats.rs serializes

6. Restart or Quit
   └─> User presses 'r' (restart) or 'q' (quit)
       └─> app.rs resets state or sets should_quit
```

## Design Patterns

### Separation of Concerns
**Pattern**: Clear boundaries between modules
- **UI**: Only rendering, no business logic
- **Engine**: Business logic, no I/O or rendering
- **Data**: Persistence only, no business logic
- **Content**: Generation only, no session management

**Benefit**: Testability, maintainability, extensibility

### Domain-Driven Design
**Pattern**: Types model the problem domain
- `TypingSession` represents a practice session
- `Lesson` represents lesson types
- `SessionResult` represents outcomes

**Benefit**: Code reads like the problem domain

### Dependency Injection
**Pattern**: High-level modules don't depend on low-level details
- App doesn't know about JSON storage
- Engine doesn't know about ratatui
- Content generator receives keyboard layout

**Benefit**: Easy to swap implementations, test with mocks

### Pure Functions
**Pattern**: Scoring functions have no side effects
```rust
pub fn calculate_wpm(char_count: usize, duration: Duration) -> f64
```

**Benefit**: Predictable, testable, parallelizable

## File Naming Conventions

### Module Files
- `mod.rs`: Module exports and public API
- Descriptive names: `render.rs`, `scoring.rs`, `generator.rs`

### Types
- PascalCase for structs/enums: `TypingSession`, `Lesson`
- snake_case for functions/variables: `calculate_wpm`, `current_index`

### Test Files
- Tests in same file as implementation (below `#[cfg(test)]`)
- Test function names: `test_<what_is_tested>`

## Configuration and Settings

### Current: No Configuration File
Phase 1 has no user-configurable settings

### Future Configuration (Phase 2+)
Potential `~/.config/typer-cli/config.toml`:
```toml
[display]
theme = "default"  # or "high-contrast"

[lessons]
default_length = 100  # characters

[keyboard]
layout = "azerty"  # future: "bepo", "qwerty"
```

## Error Handling Strategy

### Current Approach
- **Terminal errors**: Propagate to main, clean exit
- **Stats I/O errors**: Log and continue (graceful degradation)
- **User input**: Validate and ignore invalid input

### Error Types
- Terminal initialization failures: Fatal (can't run without terminal)
- Stats file issues: Non-fatal (can run without stats)
- Invalid lesson: Panic (programmer error, not user error)

## Extensibility Points

### Adding New Lesson Types
1. Add variant to `Lesson` enum in `content/lesson.rs`
2. Implement generation logic in `content/generator.rs`
3. Update UI to show new lesson type

### Adding New Keyboard Layouts
1. Create new file in `keyboard/` (e.g., `bepo.rs`)
2. Implement layout struct with key definitions
3. Update generator to accept layout parameter

### Adding New Metrics
1. Add field to `SessionResult` in `engine/types.rs`
2. Implement calculation in `engine/scoring.rs`
3. Update UI rendering in `ui/render.rs`
4. Update `SessionRecord` for persistence

### Adding Themes
1. Create theme struct in `ui/theme.rs`
2. Define color schemes
3. Pass theme to render functions
4. Add theme selection to config file

## Testing Architecture

### Unit Tests
- **Location**: Same file as implementation (`#[cfg(test)]` module)
- **Coverage**: All business logic in engine/, content/, data/
- **Strategy**: Test public API, mock dependencies where needed

### Integration Tests
- **Location**: `tests/` directory (future)
- **Coverage**: End-to-end session flows
- **Strategy**: Simulate user input, verify state changes

### Test Data
- **Location**: Inline in test functions or const data
- **Strategy**: Deterministic, edge cases, happy paths

## Performance Considerations

### Real-time Input Processing
- **Target**: <50ms latency from keypress to visual feedback
- **Approach**:
  - Minimal processing per keystroke
  - Pre-allocated data structures
  - Efficient ratatui rendering

### Memory Usage
- **Target**: <10MB resident memory
- **Approach**:
  - No large in-memory buffers
  - Streaming stats to disk
  - Bounded session history

### Startup Time
- **Target**: <100ms cold start
- **Achieved**: ~50ms (measured with hyperfine)
- **Approach**:
  - Minimal dependencies
  - Lazy loading where possible

## Future Architectural Considerations

### Plugin System (Phase 3+)
- Dynamic lesson loading from `~/.config/typer-cli/lessons/`
- JSON lesson definitions
- Custom content generators

### Multi-Language Support
- Separate language packs
- i18n for UI strings
- Language-specific content generators

### Analytics Engine
- Per-key tracking
- Heat map generation
- Trend analysis
- Weak point detection

### Export/Import
- Export stats to CSV
- Import custom word lists
- Backup/restore functionality
