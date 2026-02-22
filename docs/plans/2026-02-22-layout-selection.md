# Layout Selection via Settings Screen Implementation Plan

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** Allow users to pick and persist their keyboard layout (Mac/PC) from a Settings screen inside the app, removing the need for a CLI flag.

**Architecture:** Add a `Config` struct in `src/data/` persisted to `~/.config/typer-cli/config.json`. Extend `Storage` with `load_config`/`save_config`. Add a new `AppState::Settings` with a simple two-item layout picker. Remove the `--layout` CLI argument.

**Tech Stack:** Rust, serde_json (already used for stats), ratatui (already used for UI)

---

### Task 1: Add `Serialize`/`Deserialize` to `LayoutVariant`

**Files:**
- Modify: `src/keyboard/azerty.rs:68-72`

**Step 1: Add serde derives to `LayoutVariant`**

In `src/keyboard/azerty.rs`, change:
```rust
#[derive(Debug, Clone, Copy, PartialEq, clap::ValueEnum)]
pub enum LayoutVariant {
    Pc,
    Mac,
}
```
to:
```rust
#[derive(Debug, Clone, Copy, PartialEq, clap::ValueEnum, serde::Serialize, serde::Deserialize)]
pub enum LayoutVariant {
    Pc,
    Mac,
}
```

**Step 2: Verify it compiles**

Run: `cargo check`
Expected: no errors

**Step 3: Commit**

```bash
git add src/keyboard/azerty.rs
git commit -m "feat(keyboard): derive Serialize/Deserialize for LayoutVariant"
```

---

### Task 2: Create `Config` struct and add `load_config`/`save_config` to `Storage`

**Files:**
- Create: `src/data/config.rs`
- Modify: `src/data/storage.rs`
- Modify: `src/data/mod.rs`

**Step 1: Write the failing test**

Add to the bottom of `src/data/storage.rs` (inside `#[cfg(test)] mod tests`):
```rust
#[test]
fn test_load_config_defaults_when_missing() {
    let (storage, _temp_dir) = create_test_storage();
    let config = storage.load_config().unwrap();
    assert_eq!(config.layout_variant, crate::keyboard::LayoutVariant::Mac);
}

#[test]
fn test_save_and_load_config() {
    let (storage, _temp_dir) = create_test_storage();
    let config = crate::data::Config {
        layout_variant: crate::keyboard::LayoutVariant::Pc,
    };
    storage.save_config(&config).unwrap();
    let loaded = storage.load_config().unwrap();
    assert_eq!(loaded.layout_variant, crate::keyboard::LayoutVariant::Pc);
}
```

**Step 2: Run tests to verify they fail**

Run: `cargo test test_load_config`
Expected: compile error — `Config`, `load_config`, `save_config` not yet defined

**Step 3: Create `src/data/config.rs`**

```rust
use crate::keyboard::LayoutVariant;

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct Config {
    pub layout_variant: LayoutVariant,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            layout_variant: LayoutVariant::Mac,
        }
    }
}
```

**Step 4: Add `load_config` and `save_config` to `Storage`**

In `src/data/storage.rs`, add after the `save` method:
```rust
pub fn load_config(&self) -> io::Result<crate::data::Config> {
    let config_path = self.config_path();
    if !config_path.exists() {
        return Ok(crate::data::Config::default());
    }
    let content = fs::read_to_string(&config_path)?;
    serde_json::from_str(&content).map_err(|e| {
        io::Error::new(io::ErrorKind::InvalidData, format!("Failed to parse config: {}", e))
    })
}

pub fn save_config(&self, config: &crate::data::Config) -> io::Result<()> {
    let content = serde_json::to_string_pretty(config).map_err(|e| {
        io::Error::new(io::ErrorKind::InvalidData, format!("Failed to serialize config: {}", e))
    })?;
    fs::write(self.config_path(), content)?;
    Ok(())
}

fn config_path(&self) -> std::path::PathBuf {
    self.file_path
        .parent()
        .expect("stats path always has a parent dir")
        .join("config.json")
}
```

**Step 5: Export `Config` from `src/data/mod.rs`**

```rust
pub mod config;
pub mod stats;
pub mod storage;

pub use config::Config;
pub use stats::{SessionRecord, Stats};
pub use storage::Storage;
```

**Step 6: Run tests to verify they pass**

Run: `cargo test test_load_config test_save_and_load_config`
Expected: both PASS

**Step 7: Commit**

```bash
git add src/data/config.rs src/data/storage.rs src/data/mod.rs
git commit -m "feat(data): add Config struct and load_config/save_config to Storage"
```

---

### Task 3: Load config in `App::new()` and remove the CLI `--layout` argument

**Files:**
- Modify: `src/app.rs:52-133`
- Modify: `src/main.rs`

**Step 1: Update `App::new()` to load config and drop `layout_variant` param**

In `src/app.rs`, change the signature and startup logic:

```rust
// Add field to App struct (after keyboard_config):
layout_variant: LayoutVariant,

// Change App::new signature:
pub fn new() -> io::Result<Self> {
    let storage = Storage::new()?;
    let config = storage.load_config()?;
    let stats = storage.load()?;
    // ... rest unchanged, replace layout_variant usage:
    keyboard_layout: match config.layout_variant {
        LayoutVariant::Mac => AzertyLayout::new_mac(),
        LayoutVariant::Pc => AzertyLayout::new(),
    },
    layout_variant: config.layout_variant,
    // ... rest unchanged
}
```

**Step 2: Update `src/main.rs` to remove `--layout` CLI arg**

Replace the full file with:
```rust
mod app;
mod content;
mod data;
mod engine;
mod keyboard;
mod ui;

use app::App;

fn main() -> std::io::Result<()> {
    let mut terminal = ratatui::init();
    let result = run_app(&mut terminal);
    ratatui::restore();
    result
}

fn run_app(terminal: &mut ratatui::DefaultTerminal) -> std::io::Result<()> {
    let mut app = App::new()?;
    app.run(terminal)
}
```

**Step 3: Verify it compiles**

Run: `cargo check`
Expected: no errors

**Step 4: Run all tests**

Run: `cargo test`
Expected: all passing (same count as before)

**Step 5: Commit**

```bash
git add src/app.rs src/main.rs
git commit -m "refactor(app): load layout from config file, remove --layout CLI arg"
```

---

### Task 4: Add `AppState::Settings` and wire navigation

**Files:**
- Modify: `src/app.rs`

**Step 1: Add `Settings` to the `AppState` enum**

In `src/app.rs`, add `Settings` to the enum:
```rust
enum AppState {
    LessonTypeMenu,
    Settings,          // new
    Statistics,
    // ...
}
```

**Step 2: Add `selected_layout` field to `App` struct**

```rust
selected_layout: usize,  // 0 = Mac, 1 = PC
```
Initialize in `App::new()`:
```rust
selected_layout: match config.layout_variant {
    LayoutVariant::Mac => 0,
    LayoutVariant::Pc => 1,
},
```

**Step 3: Add key handler for `p` in `LessonTypeMenu`**

Inside `AppState::LessonTypeMenu => match key.code { ... }`, add:
```rust
KeyCode::Char('p') | KeyCode::Char('P') => {
    self.state = AppState::Settings;
}
```

**Step 4: Add key handler for `AppState::Settings`**

```rust
AppState::Settings => match key.code {
    KeyCode::Esc | KeyCode::Char('q') => {
        self.state = AppState::LessonTypeMenu;
    }
    KeyCode::Up | KeyCode::Char('k') => {
        if self.selected_layout > 0 {
            self.selected_layout -= 1;
        }
    }
    KeyCode::Down | KeyCode::Char('j') => {
        if self.selected_layout < 1 {
            self.selected_layout += 1;
        }
    }
    KeyCode::Enter | KeyCode::Char(' ') => {
        let variant = if self.selected_layout == 0 {
            LayoutVariant::Mac
        } else {
            LayoutVariant::Pc
        };
        self.keyboard_layout = match variant {
            LayoutVariant::Mac => AzertyLayout::new_mac(),
            LayoutVariant::Pc => AzertyLayout::new(),
        };
        self.layout_variant = variant;
        let config = crate::data::Config { layout_variant: variant };
        self.storage.save_config(&config)?;
        self.state = AppState::LessonTypeMenu;
    }
    _ => {}
},
```

**Step 5: Add `Settings` render arm in the `run()` draw loop**

```rust
AppState::Settings => {
    ui::render_settings(f, self.selected_layout, self.layout_variant);
}
```

**Step 6: Verify it compiles**

Run: `cargo check`
Expected: compile error on `ui::render_settings` (not yet implemented — expected)

**Step 7: Commit the app logic (before UI)**

```bash
git add src/app.rs
git commit -m "feat(app): add Settings state, layout selection and persistence"
```

---

### Task 5: Implement `render_settings` UI function

**Files:**
- Modify: `src/ui/render.rs`
- Modify: `src/ui/mod.rs`

**Step 1: Add `render_settings` to `src/ui/render.rs`**

Add at the bottom of the file:
```rust
/// Render the settings screen for layout selection
pub fn render_settings(f: &mut Frame, selected_layout: usize, current_variant: crate::keyboard::LayoutVariant) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(2)
        .constraints([
            Constraint::Length(3), // Header
            Constraint::Min(6),    // Options
            Constraint::Length(3), // Instructions
        ])
        .split(f.area());

    let header = Paragraph::new("TYPER CLI - Settings")
        .style(Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD))
        .alignment(Alignment::Center)
        .block(Block::default().borders(Borders::ALL).padding(ratatui::widgets::Padding::horizontal(1)));
    f.render_widget(header, chunks[0]);

    let layouts = [
        ("AZERTY Mac", crate::keyboard::LayoutVariant::Mac),
        ("AZERTY PC",  crate::keyboard::LayoutVariant::Pc),
    ];

    let items: Vec<ListItem> = layouts
        .iter()
        .enumerate()
        .map(|(i, (name, variant))| {
            let is_selected = i == selected_layout;
            let is_active = *variant == current_variant;
            let prefix = if is_selected { "▶ " } else { "  " };
            let check = if is_active { " [✓]" } else { "" };
            let label = format!("{}{}{}", prefix, name, check);
            let style = if is_selected {
                Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)
            } else {
                Style::default().fg(Color::White)
            };
            ListItem::new(Line::from(Span::styled(label, style)))
        })
        .collect();

    let list = List::new(items).block(
        Block::default()
            .title("Keyboard Layout")
            .borders(Borders::ALL)
            .padding(ratatui::widgets::Padding::new(1, 1, 1, 0)),
    );
    f.render_widget(list, chunks[1]);

    let instructions = vec![
        Line::from(""),
        Line::from(Span::styled(
            "Use ↑/↓ or j/k to navigate  •  Enter to apply and save  •  ESC to cancel",
            Style::default().fg(Color::Gray),
        )),
    ];
    f.render_widget(Paragraph::new(instructions).alignment(Alignment::Center), chunks[2]);
}
```

**Step 2: Export `render_settings` from `src/ui/mod.rs`**

```rust
pub use render::{
    render, render_analytics_details, render_analytics_export, render_analytics_history,
    render_duration_menu, render_lesson_type_menu, render_menu, render_results, render_settings,
    render_statistics,
};
```

**Step 3: Verify it compiles**

Run: `cargo check`
Expected: no errors

**Step 4: Run all tests**

Run: `cargo test`
Expected: all passing

**Step 5: Commit**

```bash
git add src/ui/render.rs src/ui/mod.rs
git commit -m "feat(ui): add render_settings screen for layout selection"
```

---

### Task 6: Update `LessonTypeMenu` footer to show active layout and `[p]` hint

**Files:**
- Modify: `src/ui/render.rs` (the `render_lesson_type_menu` function)
- Modify: `src/app.rs` (pass `layout_variant` to render call)

**Step 1: Update `render_lesson_type_menu` signature to accept `LayoutVariant`**

Change:
```rust
pub fn render_lesson_type_menu(
    f: &mut Frame,
    categories: &[crate::content::LessonCategory],
    selected: usize,
)
```
To:
```rust
pub fn render_lesson_type_menu(
    f: &mut Frame,
    categories: &[crate::content::LessonCategory],
    selected: usize,
    layout_variant: crate::keyboard::LayoutVariant,
)
```

**Step 2: Update the instructions string inside `render_lesson_type_menu`**

Replace:
```rust
"Use ↑/↓ or j/k to navigate  •  Press Enter/Space or 1-5 to select  •  Press 's' for Statistics  •  ESC to quit"
```
With a two-line version:
```rust
let layout_name = match layout_variant {
    crate::keyboard::LayoutVariant::Mac => "Mac",
    crate::keyboard::LayoutVariant::Pc => "PC",
};
let hint = format!(
    "↑/↓ j/k navigate  •  Enter/1-5 select  •  [s] Statistics  •  [p] Preferences (Layout: {})  •  ESC quit",
    layout_name
);
```
Then use `hint.as_str()` in the `Span::styled` call instead of the old literal.

**Step 3: Update the call site in `src/app.rs`**

Change:
```rust
ui::render_lesson_type_menu(f, &self.categories, self.selected_category);
```
To:
```rust
ui::render_lesson_type_menu(f, &self.categories, self.selected_category, self.layout_variant);
```

**Step 4: Verify it compiles and run tests**

Run: `cargo check && cargo test`
Expected: no errors, all tests passing

**Step 5: Smoke test manually**

Run: `cargo run`
- Verify footer shows `Layout: Mac` by default
- Press `p` → Settings screen opens
- Select `AZERTY PC`, press Enter → returns to main menu, footer now shows `Layout: PC`
- Quit and relaunch → layout is still PC

**Step 6: Commit**

```bash
git add src/ui/render.rs src/app.rs
git commit -m "feat(ui): show active layout and preferences hint in main menu footer"
```
