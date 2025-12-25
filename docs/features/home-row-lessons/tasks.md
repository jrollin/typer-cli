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

## Phase 2: Progressive Levels

### Additional Home Row Levels
- [ ] Level 2: Middle fingers (d, k) + existing
- [ ] Level 3: Ring fingers (s, l) + existing
- [ ] Level 4: Pinkies (q, m) + existing
- [ ] Level 5: All home row keys combined
- [ ] Level 6: Simple French words using home row

### Lesson Selection UI
- [ ] Menu to select lesson level
- [ ] Progress tracking (which levels completed)
- [ ] Recommendation system (suggest next level)

### Content Refinement
- [ ] Balanced key distribution per level
- [ ] Varied patterns (not just alternating)
- [ ] Progressive complexity within each level
- [ ] French word validation (common words only)

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

### Completed Features
- Home row level 1 (f and j)
- Deterministic generation (same level = same content)
- AZERTY layout integration
- Appropriate content length (~50-100 chars)

### Technical Decisions
- Deterministic for reproducible testing
- Space-separated for accurate WPM calculation
- ContentGenerator trait for extensibility
- Lesson enum supports future lesson types

### Test Coverage
- 7 unit tests in `src/content/`
- Character set validation
- Length validation
- Deterministic generation
- Integration with keyboard layout module
