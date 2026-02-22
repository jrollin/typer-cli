# Layout Selection via Settings Screen

**Date**: 2026-02-22
**Status**: Approved

## Problem

Layout variant (Mac/PC) must currently be passed as a CLI argument at launch (`--layout`). Users cannot change it without restarting the application.

## Solution

Add a persistent Settings screen accessible from the main menu. Layout choice is saved to a local config file and applied immediately on change.

## Data Layer

- New `Config { layout_variant: LayoutVariant }` struct in `src/data/`
- Default: `LayoutVariant::Mac`
- Persisted to `~/.config/typer-cli/config.json` (separate from `stats.json`)
- `LayoutVariant` gains `serde::Serialize` / `serde::Deserialize` derives
- `Storage` gains `load_config() -> Config` and `save_config(&Config) -> io::Result<()>`

## App Startup

- `App::new()` loads config on startup; falls back to `Config::default()` if file is missing
- CLI `--layout` argument is removed from `main.rs` entirely
- `run_app()` signature simplified (no `layout_variant` param)

## Settings Screen

- New `AppState::Settings`
- Access: press `p` from `LessonTypeMenu`
- UI: two items — `AZERTY Mac` and `AZERTY PC`, current selection marked `[✓]`
- `↑/↓` or `j/k` to navigate, `Enter` to apply + persist, `Esc` to cancel
- On confirm: updates `self.keyboard_layout` and saves config immediately

## Navigation Changes

- `LessonTypeMenu` footer: add `[p] Preferences` hint and `Layout: Mac` indicator
- `render_lesson_type_menu()` receives current `LayoutVariant` to display active layout
