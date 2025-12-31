# Keyboard Layout - Requirements

> **Purpose**: Captures requirements for keyboard layout definitions
> **Module**: `src/keyboard/`
> **Next Step**: See `design.md` for layout implementation details

## Layout Requirements

### US-1 AZERTY Layout Support
THE SYSTEM SHALL use AZERTY keyboard layout as the default and only layout in Phase 1

### US-2 AZERTY Home Row Definition
THE SYSTEM SHALL define AZERTY home row keys as: q, s, d, f, g, h, j, k, l, m

### US-3 Finger-Key Mapping
THE SYSTEM SHALL group home row keys by finger position:
- Left pinky: q
- Left ring: s
- Left middle: d
- Left index: f, g
- Right index: h, j
- Right middle: k
- Right ring: l
- Right pinky: m

## Future Phase Requirements (Out of MVP Scope)

### US-4 Layout Extensibility (Phase 2+)
FUTURE: Support additional keyboard layouts (BÉPO, Dvorak)
FUTURE: Allow user-configurable custom layouts
