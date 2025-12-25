# Home Row Lessons - Requirements

> **Purpose**: Captures requirements for home row practice content generation
> **Module**: `src/content/`
> **Next Step**: See `design.md` for lesson generation strategy

## Lesson Content Requirements

### R10: Home Row Practice
THE SYSTEM SHALL provide home row lessons focusing on AZERTY layout keys: q, s, d, f, g, h, j, k, l, m

### R11: Progressive Difficulty
THE SYSTEM SHALL organize home row lessons in progressive difficulty:
- Level 1: f and j (index fingers)
- Level 2: d and k (middle fingers)
- Level 3: s and l (ring fingers)
- Level 4: q and m (pinkies)
- Level 5: All home row keys combined
- Level 6: Simple French words using home row

### R12: AZERTY Layout Support
THE SYSTEM SHALL use AZERTY keyboard layout as the default and only layout in Phase 1

### R13: French Language Support
THE SYSTEM SHALL generate practice content in French language for Phase 1

## Future Phase Requirements (Out of MVP Scope)

### R38: Additional Training Modes (Phase 2+)
FUTURE: Bigram mode for common French (qu, ou, en) and English (th, er, on) patterns
FUTURE: Code mode for programming symbols ({}, [], (), <>, ->, ::, =>)
FUTURE: Support for TypeScript, Rust, Python specific patterns

### R40: English Language Support (Phase 2+)
FUTURE: Support English language practice content
