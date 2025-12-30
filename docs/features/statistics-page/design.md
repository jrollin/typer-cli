# Statistics Page - Design

## Architecture

### State Machine
- Add `Statistics` variant to `AppState` enum
- Navigation: LessonTypeMenu ↔ Statistics
- Render statistics via `ui::render_statistics()`

### Layout Structure
Two-column layout (40% / 60%):
- Left: Overall stats, Mastery breakdown, Weaknesses, Mistypes
- Right: Keyboard heatmap visualization

### Data Flow
```
Stats (loaded from disk)
  ├─→ adaptive_analytics: Option<AdaptiveAnalytics>
      ├─→ key_stats: HashMap<char, KeyStats>
      ├─→ total_sessions, total_keystrokes
      └─→ session_history
  └─→ sessions: Vec<SessionRecord>
```

### Component Breakdown
1. `render_statistics()` - Main orchestrator
2. `render_overall_stats_block()` - Session summary
3. `render_mastery_breakdown()` - Key classification counts
4. `render_weaknesses_list()` - Top 10 weak keys
5. `render_common_mistypes()` - Error pattern analysis
6. `render_keyboard_with_heatmap()` - Keyboard visualization with heatmap always enabled

### Statistics-Specific Behavior
- Heatmap is **always enabled** in statistics context (cannot be toggled off)
- Finger color hints are **disabled** (heatmap takes priority)
- Footer keyboard shortcuts are **hidden** (not needed, we have our own instructions)
- This creates a cleaner, more focused analytics view

### Color Scheme
- Headers: Cyan + Bold
- Metric values: Yellow
- Mastery levels: Green/Yellow/LightRed/Red
- Instructions: Gray

### Edge Cases
- No analytics data → Show placeholder
- No weak keys → Congratulatory message
- No mistypes → Perfect accuracy message
- Terminal < 25 lines → Compact layout

## Technical Decisions

### Why Two-Column Layout?
- Efficient use of horizontal space
- All info visible at once (no scrolling)
- Keyboard is central visual feature

### Why Show 's' Option Always?
- Simplifies menu logic
- Statistics page handles no-data case internally
- Encourages users to complete sessions

### Why Filter < 5 Attempts?
- Prevents noise from newly introduced keys
- Ensures statistically meaningful weakness identification

## Dependencies
- Existing: `src/ui/keyboard.rs` (render_keyboard)
- Existing: `src/data/stats.rs` (all analytics methods)
- Existing: `src/engine/analytics.rs` (KeyStats, MasteryLevel)
