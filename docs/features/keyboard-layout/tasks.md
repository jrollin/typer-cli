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
- [x] AltGr combinations (Phase 3.6 ✓)
- [ ] Dead keys handling

### Lesson Integration
- [ ] Top row practice lessons
- [ ] Full keyboard practice lessons
- [ ] Symbol-focused lessons
- [ ] French text with accents

## Phase 3: Visual Keyboard Display ✓

### AZERTY Visual Layout (src/ui/keyboard.rs)
- [x] Number row display ([²] [&] [é] ["] [']... with ² key as first key)
- [x] Top row with Tab key box ([Tab] [a] [z] [e]...)
- [x] Home row with Caps Lock key box ([Caps] [q] [s] [d]...)
- [x] Enter key as arrow ([←]) on home row right end
- [x] Bottom row with Shift key boxes ([ ⇧ ] [<] [w]... [ ⇧ ])
- [x] Modifier row ([Ctrl] [⌘] [⌥] [Space] [Alt] [Fn1] [Fn2])
- [x] Next key highlighting (cyan background)
- [x] Shift key highlighting (both shift keys when shift required)
- [x] Non-typeable keys in grey (Tab, Caps, Ctrl, Cmd, Option, Alt, Fn)
- [x] Proper AZERTY alignment and spacing

### Layout Data Model (src/keyboard/azerty.rs)
- [x] Extended RowType enum with Modifier variant
- [x] Complete number row (12 keys, starting with &)
- [x] Shift variant mappings (letters, symbols, numbers)
- [x] Base key lookup (handles shift variants)
- [x] Modifier row definition
- [x] Unit tests (91 tests passing)

### Integration
- [x] Keyboard rendering in main UI layout
- [x] Tab key toggle (show/hide keyboard)
- [x] Real-time next key indication
- [x] Shift state detection and display

## Phase 3.6: AltGr Modifier Support ✓

Standard French AZERTY AltGr layout implemented

### Data Model (src/keyboard/azerty.rs)
- [x] Add missing ² key (replaces Esc) with ³ shift variant
- [x] Extend Key struct with altgr_variant field
- [x] Add altgr_mappings HashMap (11 mappings)
- [x] Implement build_altgr_mappings() function
- [x] Add requires_altgr() detection method
- [x] Update get_base_key() to handle AltGr variants
- [x] Add 7 comprehensive tests (18 total keyboard tests)

### UI Integration (src/ui/keyboard.rs)
- [x] Highlight AltGr key when needed (cyan background)
- [x] Add should_highlight_altgr() function
- [x] Update render_key() to check AltGr variants
- [x] Update render_keyboard_row() with requires_altgr parameter
- [x] Update render_keyboard_compact() for AltGr display

### AltGr Mappings (Standard French AZERTY)
- é (2) + AltGr = ~
- " (3) + AltGr = #
- ' (4) + AltGr = {
- ( (5) + AltGr = [
- - (6) + AltGr = |
- è (7) + AltGr = `
- _ (8) + AltGr = \
- ç (9) + AltGr = ^
- à (0) + AltGr = @
- ) (-) + AltGr = ]
- = (=) + AltGr = }

### Test Results
- All 132 tests passing (was 129, added 7 new tests, updated 4 existing)
- Number row now has 13 keys (was 12)

## Phase 4: Multi-Layout Support (FUTURE)

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

### Advanced Visualization
- [ ] Finger position indicators
- [ ] Heat map overlay for error rates (accuracy colors)
- [ ] Adaptive analytics integration
- [ ] Compact mode for small terminals

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
