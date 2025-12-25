# Typing Session - Design Document

> **Purpose**: Technical design for the core typing session engine
> **Module**: `src/engine/`
> **Previous Step**: See `requirements.md` for what we're building
> **Related**: See `../../steering/structure.md` for overall architecture

## Overview

The typing session engine is the core business logic that manages practice sessions, validates input, and calculates performance metrics. It's designed to be independent of the UI layer for maximum testability.

## Component Design

### TypingSession Lifecycle

**State Machine:**
```
[Created] → [Active] → [Completed]
    ↓          ↓            ↓
 content   inputs      results
   set    recorded   calculated
```

**Implementation:**
```rust
pub struct TypingSession {
    content: String,           // Immutable after creation
    current_index: usize,      // Current position [0..content.len()]
    inputs: Vec<CharInput>,    // Append-only history
    start_time: Instant,       // Fixed at creation
    end_time: Option<Instant>, // None until completed
}

impl TypingSession {
    pub fn new(content: String) -> Self {
        Self {
            content,
            current_index: 0,
            inputs: Vec::new(),
            start_time: Instant::now(),
            end_time: None,
        }
    }

    pub fn process_input(&mut self, typed: char) {
        // Record input with timestamp
        // Advance index
        // Check completion
    }

    pub fn is_complete(&self) -> bool {
        self.current_index >= self.content.len()
    }
}
```

**Design Decisions:**
- **Immutable content**: Prevents mid-session changes, simpler reasoning
- **Append-only inputs**: No backspace in Phase 1, easy to replay
- **Instant for timing**: Monotonic, not affected by system clock changes
- **Option for end_time**: Models incomplete vs complete sessions

### CharInput Recording

**Structure:**
```rust
pub struct CharInput {
    expected: char,
    typed: char,
    timestamp: Duration,  // Relative to session start
    is_correct: bool,     // Pre-computed for efficiency
}
```

**Why record both expected and typed:**
- Error analysis (what mistakes are common)
- Replay capability
- Debugging and validation

**Why Duration instead of Instant:**
- Serializable
- Relative timing sufficient for metrics
- Smaller memory footprint

### Scoring Algorithms

**WPM Calculation:**
```rust
pub fn calculate_wpm(char_count: usize, duration: Duration) -> f64 {
    let chars = char_count as f64;
    let minutes = duration.as_secs_f64() / 60.0;

    if minutes == 0.0 {
        return 0.0;
    }

    // Standard: 5 chars = 1 word
    (chars / 5.0) / minutes
}
```

**Rationale:**
- Industry standard: 5 characters per word
- Handles edge case: 0 duration → 0 WPM (not divide by zero)
- Real-time: Can be called on partial sessions

**Accuracy Calculation:**
```rust
pub fn calculate_accuracy(inputs: &[CharInput]) -> f64 {
    if inputs.is_empty() {
        return 100.0;  // No input = perfect accuracy
    }

    let correct = inputs.iter().filter(|i| i.is_correct).count();
    (correct as f64 / inputs.len() as f64) * 100.0
}
```

**Rationale:**
- Simple: correct chars / total chars
- Edge case: Empty session = 100% (no errors made)
- Returns percentage (0-100) not fraction (0-1)

## Data Flow

### Session Lifecycle Flow

```
1. Session Creation
   ┌──────────────┐
   │ User starts  │
   └──────┬───────┘
          │
          ▼
   ┌─────────────────────┐
   │ ContentGenerator    │
   │ generates lesson    │
   └──────┬──────────────┘
          │
          ▼
   ┌─────────────────────┐
   │ TypingSession::new()│
   │ with content        │
   └─────────────────────┘

2. Character Processing
   ┌──────────────┐
   │ User types   │
   └──────┬───────┘
          │
          ▼
   ┌─────────────────────┐
   │ Event captured      │
   │ (crossterm)         │
   └──────┬──────────────┘
          │
          ▼
   ┌─────────────────────┐
   │ session.process()   │
   │ - Record input      │
   │ - Advance index     │
   │ - Check complete    │
   └──────┬──────────────┘
          │
          ▼
   ┌─────────────────────┐
   │ Render update       │
   │ - Show char color   │
   │ - Update metrics    │
   └─────────────────────┘

3. Session Completion
   ┌──────────────┐
   │ Last char    │
   └──────┬───────┘
          │
          ▼
   ┌─────────────────────┐
   │ session.complete()  │
   │ - Set end_time      │
   │ - Calculate results │
   └──────┬──────────────┘
          │
          ▼
   ┌─────────────────────┐
   │ Show results screen │
   └──────┬──────────────┘
          │
          ▼
   ┌─────────────────────┐
   │ Save to stats.json  │
   └─────────────────────┘
```

### Event Handling Flow

```rust
match event::read()? {
    Event::Key(key_event) => {
        match key_event.code {
            KeyCode::Esc => {
                app.should_quit = true;
            }
            KeyCode::Char(c) if !app.show_results => {
                app.session.process_input(c);
                if app.session.is_complete() {
                    save_session(&app.session)?;
                    app.show_results = true;
                }
            }
            KeyCode::Char('q') if app.show_results => {
                app.should_quit = true;
            }
            KeyCode::Char('r') if app.show_results => {
                app.restart();
            }
            _ => {}
        }
    }
    _ => {}
}
```

## Performance Considerations

### Input Processing Latency

**Target**: <50ms from keypress to screen update

**Optimization:**
- Minimal work in process_input()
- Pre-computed is_correct flag
- Efficient ratatui rendering (diffing)
- No disk I/O in hot path

### Memory Efficiency

**Current usage**: ~3MB resident memory

**Optimization:**
- Small data structures (no large buffers)
- Vec capacity pre-allocation for inputs
- Session content is single String (not char vec)

## Testing Strategy

### Unit Test Coverage

**engine/scoring.rs:**
```rust
#[cfg(test)]
mod tests {
    #[test]
    fn test_wpm_standard_case() {
        // 50 chars in 60 seconds = 10 WPM
        let wpm = calculate_wpm(50, Duration::from_secs(60));
        assert_eq!(wpm, 10.0);
    }

    #[test]
    fn test_wpm_zero_duration() {
        let wpm = calculate_wpm(50, Duration::from_secs(0));
        assert_eq!(wpm, 0.0);
    }

    #[test]
    fn test_accuracy_perfect() {
        let inputs = vec![
            CharInput { expected: 'a', typed: 'a', is_correct: true, ... },
            CharInput { expected: 'b', typed: 'b', is_correct: true, ... },
        ];
        assert_eq!(calculate_accuracy(&inputs), 100.0);
    }
}
```

**engine/types.rs:**
```rust
#[test]
fn test_session_completion() {
    let mut session = TypingSession::new("abc".to_string());
    assert!(!session.is_complete());

    session.process_input('a');
    session.process_input('b');
    assert!(!session.is_complete());

    session.process_input('c');
    assert!(session.is_complete());
}
```

### Integration Test Strategy

**Full session test:**
```rust
#[test]
fn test_complete_session() {
    let content = "test".to_string();
    let mut session = TypingSession::new(content);

    // Simulate typing
    session.process_input('t');
    session.process_input('e');
    session.process_input('s');
    session.process_input('t');

    assert!(session.is_complete());

    let result = session.result();
    assert_eq!(result.char_count, 4);
    assert_eq!(result.error_count, 0);
    assert_eq!(result.accuracy, 100.0);
}
```

## File Locations

- `src/engine/types.rs` - TypingSession, CharInput, SessionResult
- `src/engine/scoring.rs` - WPM and accuracy calculations
- `src/engine/mod.rs` - Module exports
