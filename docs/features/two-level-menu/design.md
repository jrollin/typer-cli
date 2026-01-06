# Two-Level Menu System - Design

## Architecture Overview

The two-level menu system introduces hierarchical navigation by adding a category selection layer before lesson selection.

### State Machine Changes

```
Previous Flow:
LessonMenu → DurationMenu → Running → Completed → LessonMenu

New Flow:
LessonTypeMenu → LessonMenu (filtered) → DurationMenu → Running → Completed → LessonMenu (filtered)
```

**New State**: `LessonTypeMenu` - First screen for category selection

**Modified State**: `LessonMenu` - Now displays filtered lessons for selected category

## Data Structures

### LessonCategoryType Enum
```rust
pub enum LessonCategoryType {
    Adaptive,
    FingerTraining,
    RowTraining,
    Languages,
    Code,
    Custom,
}
```

### LessonCategory Struct
```rust
pub struct LessonCategory {
    pub category_type: LessonCategoryType,
    pub name: &'static str,
    pub description: &'static str,
    pub color: Color,
}
```

### App State Fields
```rust
// New fields in App struct
selected_category: usize,           // Index into categories vec
categories: Vec<LessonCategory>,    // Available categories
current_category: Option<LessonCategoryType>, // Active filter
```

## Category Definitions

| Category | Name | Description | Color | Filter Logic |
|----------|------|-------------|-------|--------------|
| Adaptive | "Adaptive" | "Personalized training based on your weak areas" | Cyan | `LessonType::Adaptive` |
| FingerTraining | "Finger Training" | "Bilateral finger-based drills (24 lessons)" | Green | `LessonType::FingerPair { .. }` |
| RowTraining | "Row Training" | "Progressive row-based exercises (8 lessons)" | Cyan | `LessonType::RowProgression { .. }` |
| Languages | "Languages" | "French & English bigrams, trigrams, and words" | Yellow | `BigramType::Natural \| Trigram \| CommonWords` |
| Code | "Code" | "Programming symbols for TypeScript, Rust, Python" | Magenta | `CodeSymbols \| BigramType::Code` |
| Custom | "Custom" | "User-provided markdown lessons" | Blue | `LessonType::Custom { .. }` |

## Filtering Algorithm

### Lesson Filtering
```rust
fn filtered_lessons(&self) -> Vec<&Lesson> {
    if let Some(category_type) = self.current_category {
        let category = find_category(category_type);
        lessons.filter(|l| category.contains_lesson(l))
    } else {
        Vec::new()
    }
}
```

### Index Conversion
Since lesson selection works with relative indices (0-N within filtered view), we need to convert back to absolute indices for lesson execution:

```rust
fn absolute_lesson_index(&self, relative_index: usize) -> Option<usize> {
    let filtered = self.filtered_lessons();
    filtered.get(relative_index)
        .and_then(|lesson| lessons.position(|l| ptr::eq(*lesson, l)))
}
```

## UI Design

### Category Menu Layout
```
┌─────────────────────────────────────┐
│  TYPER CLI - Select Lesson Type    │  ← Header
├─────────────────────────────────────┤
│  ▶ 1. Adaptive                      │  ← Selected (Yellow)
│     Personalized training based...  │  ← Description (Gray)
│                                     │
│    2. Finger Training               │  ← Unselected (Green)
│     Bilateral finger-based drills...│
│                                     │
│    3. Row Training                  │  ← Unselected (Cyan)
│     Progressive row-based exercises.│
│  ...                                │
├─────────────────────────────────────┤
│  ↑/↓ or j/k • Enter/1-5 • ESC quit  │  ← Instructions
└─────────────────────────────────────┘
```

### Lesson Menu Layout (Filtered)
```
┌─────────────────────────────────────┐
│  TYPER CLI - Finger Training Lessons│  ← Header with category
├─────────────────────────────────────┤
│  ─── PINKY FINGERS ───              │  ← Group separator (green)
│  ▶ 1. Pinky Fingers - Home Row     │
│    2. Pinky Fingers - Home Row + Shift│
│    3. Pinky Fingers - Extended     │
│    4. Pinky Fingers - Extended + Shift│
│    5. Pinky Fingers - All Keys     │
│    6. Pinky Fingers - All Keys + Shift│
│                                     │
│  ─── RING FINGERS ───               │  ← Group separator (green)
│    7. Ring Fingers - Home Row      │
│  ...                                │
├─────────────────────────────────────┤
│  ↑/↓ or j/k • Enter/1-9 • ESC back │  ← "go back" not "quit"
└─────────────────────────────────────┘
```

**Visual Grouping** (Phase 3.4):
- Visual separators group lessons within categories by logical type
- Languages: Grouped by language (French, English) - cyan separators
- Finger Training: Grouped by finger pair (Pinky, Ring, Middle, Index) - green separators
- Code: Grouped by type/language (Code Patterns, TypeScript, Rust, Python) - magenta separators
- Separators use category colors for visual consistency
- Blank line spacing between groups for readability
- Row Training and Adaptive categories use standard rendering (no grouping)

## Event Handling

### LessonTypeMenu State
```rust
KeyCode::Up/Down/j/k     → Navigate categories
KeyCode::Enter/Space     → Open selected category
KeyCode::Char('1'-'5')   → Direct category selection
KeyCode::Esc/q           → Quit application
```

### LessonMenu State (Modified)
```rust
KeyCode::Up/Down/j/k     → Navigate filtered lessons
KeyCode::Enter/Space     → Select lesson → DurationMenu
KeyCode::Char('1'-'9')   → Direct lesson selection
KeyCode::Esc/q           → Return to LessonTypeMenu (CHANGED)
```

### DurationMenu State (Modified)
```rust
KeyCode::Enter/Space     → Convert relative→absolute index, start lesson
KeyCode::Esc/q           → Return to LessonMenu
```

## Navigation Flow Matrix

| From State | Action | To State | Notes |
|------------|--------|----------|-------|
| LessonTypeMenu | Select category | LessonMenu | Sets `current_category`, resets `selected_lesson=0` |
| LessonTypeMenu | ESC | Quit | Application exit |
| LessonMenu | Select lesson | DurationMenu | Uses filtered index |
| LessonMenu | ESC | LessonTypeMenu | Clears `current_category` |
| DurationMenu | Start | Running | Converts filtered→absolute index |
| DurationMenu | ESC | LessonMenu | Maintains category filter |
| Running | ESC | LessonMenu | Resets to first filtered lesson |
| Completed | ESC/q | LessonMenu | Resets to first filtered lesson |
| Completed | r | DurationMenu | Restart same lesson |

## Implementation Considerations

### Session Persistence
- `selected_lesson` stores absolute index during active session
- After session, reset to 0 for filtered view
- `current_category` preserved across session lifecycle

### Backwards Compatibility
- All existing lesson generation logic unchanged
- Lesson data structure unchanged
- Session tracking unchanged
- Analytics integration unchanged

### Performance
- Filtering is O(n) where n = total lessons (~60)
- Category lookup is O(1) with enum matching
- Index conversion is O(n) but only on lesson start
- No performance impact on typing session

### Edge Cases
- Empty categories: Not possible with current lesson set
- Adaptive unavailable: Category not shown, list shifts
- Single lesson in category: Still navigable
- All categories available: Maximum 5 options

## Testing Strategy

### Unit Tests
- Category filtering logic
- Index conversion (relative ↔ absolute)
- Category conditional display (adaptive)

### Integration Tests
- Navigation flow (all state transitions)
- ESC behavior at each level
- Number key shortcuts
- Session completion flow

### Manual Tests
- Visual presentation of categories
- Color coding correctness
- Description display
- Scroll behavior with long lesson lists
- Category context preservation
