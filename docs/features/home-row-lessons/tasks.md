# Home Row Lessons - Task Tracking

> **Purpose**: Implementation progress for home row lesson content
> **Module**: `src/content/`
> **Status**: ✓ Level 1 COMPLETED, Levels 2-6 PLANNED (Phase 2)

## Phase 1: MVP Implementation

### Content Generation (src/content/) ✓
- [x] `lesson.rs` - Lesson enum and type definitions
- [x] `generator.rs` - HomeRowGenerator implementation
- [x] `mod.rs` - Module exports
- [x] Level 1 generation (f and j keys)
- [x] Deterministic content generation
- [x] Unit tests (7 tests passing)

### Integration ✓
- [x] App starts with Level 1 lesson
- [x] Content provided to TypingSession
- [x] Lesson name displayed in UI header

### Testing ✓
- [x] Content generation tests
- [x] Character validation (only expected chars)
- [x] Length validation (50-150 chars)
- [x] Deterministic generation test
- [x] Space-separated format verification

## Phase 2: Progressive Levels ✓ COMPLETED

### Additional Home Row Levels ✓
- [x] Level 2: Middle fingers (f, j, d, k) - cumulative
- [x] Level 3: Ring fingers (f, j, d, k, s, l) - cumulative
- [x] Level 4: Pinkies (f, j, d, k, s, l, q, m) - cumulative
- [x] Level 5: All home row keys combined (q, s, d, f, g, h, j, k, l, m)
- [x] Level 6: Simple French words using home row

### Lesson Selection UI ✓
- [x] Menu to select lesson level (1-6)
- [x] Keyboard navigation (↑/↓, j/k)
- [x] Direct selection with number keys (1-6)
- [x] Visual feedback for selected lesson
- [ ] Progress tracking (which levels completed) - Phase 3
- [ ] Recommendation system (suggest next level) - Phase 3

### Content Refinement ✓
- [x] Balanced key distribution per level
- [x] Varied patterns: repetitions, alternations, triplets
- [x] Progressive complexity within each level
- [x] French words for Level 6 (la, le, de, se, me, je, mal, sel, etc.)

## Phase 3: Extended Training Modes

### Bigram Mode
- [ ] French bigrams: "qu", "ou", "en", "on", "an"
- [ ] English bigrams: "th", "er", "he", "an", "in"
- [ ] Common programming bigrams: "->", "::", "=>", "!="

### Code Mode
- [ ] TypeScript patterns: `const x: number = 42;`
- [ ] Rust patterns: `fn main() { let x = 42; }`
- [ ] Python patterns: `def main(): x = 42`
- [ ] Symbols practice: `{}`, `[]`, `()`, `<>`, `;`, `:`

### Word Lists
- [ ] French common words (frequency-based)
- [ ] English common words
- [ ] Programming keywords
- [ ] Technical terms

## Implementation Notes

### Completed Features (Phase 1)
- Home row level 1 (f and j)
- Deterministic generation (same level = same content)
- AZERTY layout integration
- Appropriate content length (~50-100 chars)

### Completed Features (Phase 2)
- All 6 home row levels with progressive cumulative key addition
- Lesson selection menu with keyboard navigation
- Direct lesson selection with number keys (1-6)
- Improved content generation with varied patterns:
  - Phase 1: Repetitions (ff, jj, dd, kk)
  - Phase 2: Alternations (fj, dk, sl, qm)
  - Phase 3: Triplets (fjd, jdk, dks, ksl)
- French words for Level 6
- Session stats saved with lesson title

### Technical Decisions
- Deterministic for reproducible testing
- Space-separated for accurate WPM calculation
- ContentGenerator trait for extensibility
- Lesson enum supports future lesson types
- **Cumulative progression**: Each level includes all previous keys plus new ones
- **Option<TypingSession>**: App state supports menu mode without active session
- **ESC returns to menu**: Better navigation flow than quitting

### Test Coverage
- 7 unit tests in `src/content/`
- Character set validation
- Length validation
- Deterministic generation
- Integration with keyboard layout module
- All 32 tests passing

### Code Changes (Phase 2)
- **src/content/lesson.rs**: Updated lesson definitions to be cumulative (Level 2: f,j,d,k instead of just d,k)
- **src/content/generator.rs**: Added `generate_progressive_drills()` for levels 2-5 with varied patterns
- **src/app.rs**:
  - Added `AppState::Menu` state
  - Changed `session` to `Option<TypingSession>`
  - Added `selected_lesson` and `lessons` fields
  - Added menu navigation (↑/↓, j/k, Enter, Space, 1-6)
  - ESC returns to menu instead of quitting
- **src/ui/render.rs**: Added `render_menu()` function with lesson list and navigation instructions
- **src/ui/mod.rs**: Exported `render_menu`
