# Keyboard Layout - Task Tracking

> **Purpose**: Implementation progress for keyboard layout definitions
> **Module**: `src/keyboard/`
> **Status**: ✓ COMPLETED (Phase 1 - AZERTY home row)

## Phase 1: MVP Implementation

### AZERTY Layout (src/keyboard/) ✓
- [x] `azerty.rs` - AZERTY home row definition
- [x] `mod.rs` - Module exports
- [x] Home row constant: "qsdfghjklm"
- [x] Finger-key mapping
- [x] Key groups by difficulty
- [x] Unit tests (2 tests passing)

### Integration ✓
- [x] Used by content generator for lesson creation
- [x] Home row keys validated in tests
- [x] Finger grouping for progressive lessons

### Testing ✓
- [x] Home row length and uniqueness
- [x] Finger mapping completeness
- [x] Key group validation

## Phase 2: Extended AZERTY Support

### Full Keyboard Layout
- [ ] Top row (AZERTY number/symbol row)
- [ ] Upper row (a, z, e, r, t, y, u, i, o, p)
- [ ] Home row (q, s, d, f, g, h, j, k, l, m) ✓
- [ ] Bottom row (w, x, c, v, b, n, comma, period, etc.)
- [ ] Shift characters mapping

### French-Specific Keys
- [ ] Accented characters (é, è, à, ù, etc.)
- [ ] Special symbols (ç, ê, ô, etc.)
- [ ] AltGr combinations
- [ ] Dead keys handling

### Lesson Integration
- [ ] Top row practice lessons
- [ ] Full keyboard practice lessons
- [ ] Symbol-focused lessons
- [ ] French text with accents

## Phase 3: Multi-Layout Support

### Additional Layouts
- [ ] BÉPO layout support
  - [ ] Home row: "auietsrn"
  - [ ] Finger mapping
  - [ ] Key groups
- [ ] Dvorak layout support
- [ ] Colemak layout support
- [ ] Custom layout definition

### Layout Abstraction
- [ ] KeyboardLayout trait
- [ ] Generic content generator using trait
- [ ] Layout selection in settings
- [ ] Layout-specific lessons

### Configuration
- [ ] JSON layout definition format
- [ ] Load custom layouts from file
- [ ] Layout validation
- [ ] Layout switching in UI

### Visualization
- [ ] ASCII keyboard layout display
- [ ] Finger position indicators
- [ ] Key highlighting for current lesson
- [ ] Heat map overlay for error rates

## Phase 4: Advanced Features

### Analytics Integration
- [ ] Per-key statistics tracking
- [ ] Finger accuracy analysis
- [ ] Key pair difficulty analysis
- [ ] Hand balance metrics

### Adaptive Learning
- [ ] Identify weak keys based on layout
- [ ] Generate targeted practice
- [ ] Progressive difficulty per finger
- [ ] Hand coordination exercises

### Physical Keyboard Variants
- [ ] ISO vs ANSI keyboard support
- [ ] Laptop keyboard variations
- [ ] Ergonomic keyboard layouts
- [ ] Split keyboard support

## Implementation Notes

### Completed Features
- AZERTY home row: q, s, d, f, g, h, j, k, l, m
- Finger-to-key mapping (touch typing standard)
- Progressive key groups (Level 1-5)
- Integration with content generator

### Technical Decisions
- Constants for simple data (home row string)
- Functions for derived data (key groups, finger mapping)
- Separation from lesson logic (layout is data, not logic)
- Extensible design for future layouts

### AZERTY Home Row Details
- Left hand: q (pinky), s (ring), d (middle), f+g (index)
- Right hand: h+j (index), k (middle), l (ring), m (pinky)
- 10 keys total in home row position

### Test Coverage
- 2 unit tests in `src/keyboard/`
- Basic validation of constants
- Future: comprehensive tests for full layouts
