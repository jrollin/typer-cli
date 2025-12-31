# TUI Interface - Design Document

> **Purpose**: Technical design for the terminal user interface
> **Module**: `src/ui/`
> **Previous Step**: See `requirements.md` for UI requirements
> **Related**: See `../../steering/tech.md` for ratatui choice

## Overview

The TUI interface provides a distraction-free typing practice experience using ratatui for declarative terminal rendering. The design emphasizes immediate visual feedback and minimal cognitive load.

## Layout Structure

### Active Session Screen

```
┌─────────────────────────────────────────┐
│  TYPER CLI - Home Row Level 1           │ ← Header
├─────────────────────────────────────────┤
│                                         │
│  Text to type:                          │ ← Expected text
│  ff jj ff jj fj jf fj jf               │   (gray for untyped)
│                                         │
│  Your input:                            │ ← User input
│  ff jj f█                               │   (green/red + cursor)
│                                         │
├─────────────────────────────────────────┤
│  WPM: 45    │  Accuracy: 98%           │ ← Live metrics
│  Time: 00:15                            │
└─────────────────────────────────────────┘
```

**Implementation:**
```rust
use ratatui::{
    layout::{Constraint, Direction, Layout},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

pub fn render_session(f: &mut Frame, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),  // Header
            Constraint::Min(8),     // Main content
            Constraint::Length(3),  // Stats
        ])
        .split(f.size());

    render_header(f, chunks[0], app);
    render_content(f, chunks[1], app);
    render_stats(f, chunks[2], app);
}
```

### Results Screen

```
┌─────────────────────────────────────────┐
│  Session Complete!                      │
├─────────────────────────────────────────┤
│                                         │
│  Final Results:                         │
│                                         │
│  WPM:         45.2                      │
│  Accuracy:    96.5%                     │
│  Duration:    01:23                     │
│  Characters:  187                       │
│  Errors:      7                         │
│                                         │
│  Press 'r' to restart                   │
│  Press 'q' to quit                      │
│                                         │
└─────────────────────────────────────────┘
```

**Implementation:**
```rust
pub fn render_results(f: &mut Frame, app: &App) {
    let block = Block::default()
        .title("Session Complete!")
        .borders(Borders::ALL);

    let results = app.session.result();
    let text = format!(
        "\n  Final Results:\n\n  WPM:         {:.1}\n  Accuracy:    {:.1}%\n  Duration:    {}:{:02}\n  Characters:  {}\n  Errors:      {}\n\n  Press 'r' to restart\n  Press 'q' to quit",
        results.wpm,
        results.accuracy,
        results.duration.as_secs() / 60,
        results.duration.as_secs() % 60,
        results.char_count,
        results.error_count,
    );

    let paragraph = Paragraph::new(text).block(block);
    f.render_widget(paragraph, f.size());
}
```

## Color Scheme

### Character States

**Color mapping:**
```rust
pub enum CharState {
    Correct,
    Incorrect,
    Pending,
}

fn render_character(ch: char, state: CharState) -> Span {
    let style = match state {
        CharState::Correct => Style::default().fg(Color::Green),
        CharState::Incorrect => Style::default().fg(Color::Red),
        CharState::Pending => Style::default().fg(Color::Gray),
    };
    Span::styled(ch.to_string(), style)
}
```

**Design rationale:**
- **Green**: Universal "correct" indicator
- **Red**: Universal "error" indicator
- **Gray**: Low-contrast for untyped text (reduces distraction)

### Cursor Rendering

**Block cursor with reverse video:**
```rust
fn render_cursor() -> Span {
    Span::styled(
        "█",
        Style::default()
            .bg(Color::White)
            .fg(Color::Black)
    )
}
```

**Why block cursor:**
- Highly visible
- Doesn't require special terminal support
- Works consistently across terminals

### Special Character Visualization

**Non-printable character icons:**
```rust
fn display_char(ch: char) -> char {
    match ch {
        ' ' => '·',      // U+00B7 Middle Dot for spaces
        '\n' => '↵',     // U+21B5 Downwards Arrow for newlines
        c => c,
    }
}
```

**Design rationale:**
- **Space (·)**: Makes whitespace visible without being distracting
- **Newline (↵)**: Clearly indicates line breaks in multi-line content
- Icons shown in both expected text and typed input for consistency
- Preserves color coding (green/red/gray) to maintain feedback

**Implementation notes:**
- Icons are display-only; actual characters stored remain unchanged
- Enter key input is validated against '\n' character
- Newline support enables practice with code snippets and structured content

## Real-time Updates

### Render Loop

```rust
loop {
    terminal.draw(|f| {
        if app.show_results {
            render_results(f, &app);
        } else {
            render_session(f, &app);
        }
    })?;

    if event::poll(Duration::from_millis(100))? {
        handle_event(&mut app)?;
    }
}
```

**Design decisions:**
- **Poll timeout**: 100ms keeps UI responsive while limiting CPU usage
- **Conditional rendering**: Different screens based on app state
- **Declarative UI**: ratatui rebuilds from state each frame (no manual diffing)

### Input Rendering

**Character-by-character display:**
```rust
fn render_input_line(session: &TypingSession) -> Line {
    let mut spans = Vec::new();

    for input in &session.inputs {
        let state = if input.is_correct {
            CharState::Correct
        } else {
            CharState::Incorrect
        };
        spans.push(render_character(input.typed, state));
    }

    // Add cursor at current position
    spans.push(render_cursor());

    Line::from(spans)
}
```

### Expected Text Rendering

**Show remaining text in gray:**
```rust
fn render_expected_line(session: &TypingSession) -> Line {
    let remaining: String = session.content
        .chars()
        .skip(session.current_index)
        .collect();

    Line::from(Span::styled(remaining, Style::default().fg(Color::Gray)))
}
```

## Statistics Panel

### Live Metric Display

```rust
fn render_stats(f: &mut Frame, chunk: Rect, app: &App) {
    let session = &app.session;
    let elapsed = session.elapsed();

    let wpm = calculate_wpm(session.inputs.len(), elapsed);
    let accuracy = calculate_accuracy(&session.inputs);

    let stats_text = format!(
        "  WPM: {:.0}    Accuracy: {:.1}%    Time: {:02}:{:02}",
        wpm,
        accuracy,
        elapsed.as_secs() / 60,
        elapsed.as_secs() % 60,
    );

    let paragraph = Paragraph::new(stats_text)
        .block(Block::default().borders(Borders::ALL));

    f.render_widget(paragraph, chunk);
}
```

**Update frequency:** Every render cycle (~100ms)

## Event Handling

### Keyboard Input Processing

```rust
use crossterm::event::{self, Event, KeyCode};

pub fn handle_event(app: &mut App) -> Result<()> {
    match event::read()? {
        Event::Key(key_event) => {
            match key_event.code {
                KeyCode::Esc => {
                    app.should_quit = true;
                }
                KeyCode::Char(c) if !app.show_results => {
                    app.session.process_input(c);
                    if app.session.is_complete() {
                        app.show_results = true;
                    }
                }
                KeyCode::Char('q') if app.show_results => {
                    app.should_quit = true;
                }
                KeyCode::Char('r') if app.show_results => {
                    app.restart();
                    app.show_results = false;
                }
                _ => {}
            }
        }
        _ => {}
    }
    Ok(())
}
```

**Key behavior:**
- **ESC**: Immediate quit (always available)
- **Printable chars**: Only during active session
- **'q'**: Only on results screen
- **'r'**: Only on results screen

## Terminal Setup and Cleanup

### Initialization

```rust
use crossterm::{
    execute,
    terminal::{enable_raw_mode, EnterAlternateScreen},
};
use ratatui::backend::CrosstermBackend;

pub fn setup_terminal() -> Result<Terminal<CrosstermBackend<Stdout>>> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let terminal = Terminal::new(backend)?;
    Ok(terminal)
}
```

### Cleanup

```rust
use crossterm::terminal::{disable_raw_mode, LeaveAlternateScreen};

pub fn restore_terminal(mut terminal: Terminal<CrosstermBackend<Stdout>>) -> Result<()> {
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;
    Ok(())
}
```

**Critical:** Cleanup must run even on panic/error to restore terminal state.

## Performance Considerations

### Rendering Optimization

**ratatui's built-in diffing:**
- Only changed cells are updated
- Minimal ANSI escape sequences sent
- Sub-millisecond render times

**Our optimizations:**
- Pre-compute styled spans (don't recreate every frame)
- Limit string allocations
- Use references where possible

### Memory Usage

**Minimal state:**
- UI holds no state, only renders from App
- No render buffers or caches
- All state in App struct

## Future Enhancements (Phase 2+)

### Keyboard Visualization

```
┌─────────────────────────────────────────┐
│  q  s  d  f  g    h  j  k  l  m        │
│  │  │  │  ●  │    │  ●  │  │  │        │  ← Highlight current target keys
└─────────────────────────────────────────┘
```

### Theme System

```rust
pub struct Theme {
    correct: Color,
    incorrect: Color,
    pending: Color,
    background: Color,
    cursor: Color,
}

const DEFAULT_THEME: Theme = Theme {
    correct: Color::Green,
    incorrect: Color::Red,
    pending: Color::Gray,
    background: Color::Black,
    cursor: Color::White,
};

const HIGH_CONTRAST_THEME: Theme = Theme {
    correct: Color::Cyan,
    incorrect: Color::Magenta,
    pending: Color::DarkGray,
    background: Color::Black,
    cursor: Color::Yellow,
};
```

### Progress Graphs

```rust
// Using ratatui's Chart widget
use ratatui::widgets::{Chart, Dataset};

fn render_progress_graph(f: &mut Frame, stats: &Stats) {
    let data: Vec<(f64, f64)> = stats.sessions
        .iter()
        .enumerate()
        .map(|(i, s)| (i as f64, s.wpm))
        .collect();

    let dataset = Dataset::default()
        .marker(symbols::Marker::Braille)
        .data(&data);

    let chart = Chart::new(vec![dataset])
        .x_axis(Axis::default().title("Sessions"))
        .y_axis(Axis::default().title("WPM"));

    f.render_widget(chart, area);
}
```

## File Locations

- `src/ui/render.rs` - All rendering logic
- `src/ui/mod.rs` - Module exports
- `src/app.rs` - Event handling and state management
- `src/main.rs` - Terminal setup/cleanup
