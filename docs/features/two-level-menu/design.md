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
    KeyTraining,
    Languages,
    Code,
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
| KeyTraining | "Key Training" | "Progressive key pair exercises (25 lessons)" | Cyan | `LessonType::KeyPair { .. } \| LessonType::KeyPairGroup { .. }` |
| Languages | "Languages" | "French & English bigrams, trigrams, and words" | Yellow | `BigramType::Natural \| Trigram \| CommonWords` |
| Code | "Code" | "Programming symbols for TypeScript, Rust, Python" | Magenta | `CodeSymbols \| BigramType::Code` |

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
│    3. Key Training                  │  ← Unselected (Cyan)
│     Progressive key pair exercises..│
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
│  ▶ 1. Pinky fingers - Home row     │
│    2. Pinky fingers - Home + Shift │
│    3. Pinky fingers - Extended     │
│  ...                                │
├─────────────────────────────────────┤
│  ↑/↓ or j/k • Enter/1-9 • ESC back │  ← "go back" not "quit"
└─────────────────────────────────────┘
```

**Key Changes**:
- No category separators (handled by separate screen)
- Simpler lesson list rendering
- Category name in header
- "ESC to go back" instruction

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
- Filtering is O(n) where n = total lessons (~77)
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
