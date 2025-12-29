# Finger Training - Design

## Architecture

### Data Model

#### New Types (src/content/lesson.rs)

```rust
/// Finger pair combinations for bilateral training
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum FingerPairType {
    Pinky,   // Left pinky + Right pinky
    Ring,    // Left ring + Right ring
    Middle,  // Left middle + Right middle
    Index,   // Left index + Right index
}

/// Extended LessonType enum
pub enum LessonType {
    // ... existing variants ...
    FingerPair {
        finger_pair: FingerPairType,
        level: u8,        // 1=Home Row, 2=Extended, 3=All Keys
        with_shift: bool, // false=base chars, true=mixed case+symbols
    },
}
```

### Content Generation

#### Key Extraction Algorithm (src/content/finger_generator.rs)

**Function**: `get_finger_pair_keys(layout, finger_pair, level, with_shift) -> Vec<char>`

**Input**:
- `layout`: AzertyLayout reference
- `finger_pair`: Which finger pair (Pinky/Ring/Middle/Index)
- `level`: Difficulty (1/2/3)
- `with_shift`: Include shift variants (true/false)

**Process**:
1. Map finger pair to (left_finger, right_finger) tuple
2. Determine allowed rows based on level:
   - Level 1: Home row only
   - Level 2: Home + Top + Bottom
   - Level 3: All rows including Number
3. Iterate through layout rows and keys
4. For each key matching either finger:
   - Add base character
   - If with_shift=true, add shift variant
5. Filter out placeholders (\0, \n)
6. Sort and deduplicate

**Output**: Vector of characters assigned to the finger pair

#### Drill Generation Algorithm

**Base Drills** (3-phase pattern):

**Phase 1**: Single key repetitions
```
ff dd jj kk
```

**Phase 2**: Adjacent pairs and reversals
```
fd df fj jf dk kd
```

**Phase 3**: Triplets with permutations (if ≥3 keys)
```
fdk dfk kfd dkf fkd kdf
```

**Shift Drills** (weighted random):

**Distribution**:
- 50% lowercase characters
- 40% uppercase characters
- 10% symbols

**Pool Building**:
1. Separate keys into: lowercase, uppercase, symbols
2. Create weighted pool:
   - Add lowercase × 50
   - Add uppercase × 40
   - Add symbols × 10

**Pattern Generation**:
- Phase 1: 20 repetitions (random selection)
- Phase 2: 30 pairs (random combinations)
- Phase 3: 50 triplets (random combinations)

**Content Assembly**:
- Cycle through patterns until length reached
- Check length before adding each pattern to prevent overflow

### French AZERTY Corrections

#### Number Row Mappings (src/keyboard/azerty.rs)

**Corrected mappings** (symbols are base, numbers are shift):

| Key | Base | Shift | Finger |
|-----|------|-------|--------|
| 1 | & | 1 | LeftPinky |
| 2 | é | 2 | LeftPinky |
| 3 | " | 3 | LeftRing |
| 4 | ' | 4 | LeftMiddle |
| 5 | ( | 5 | LeftIndex |
| 6 | - | 6 | LeftIndex |
| 7 | è | 7 | RightIndex |
| 8 | _ | 8 | RightMiddle |
| 9 | ç | 9 | RightRing |

#### Bottom Row Corrections

| Key | Was | Now |
|-----|-----|-----|
| w | LeftRing | LeftPinky |
| x | LeftMiddle | LeftRing |
| c | LeftIndex | LeftMiddle |
| b | RightIndex | LeftIndex |
| , | RightMiddle | RightIndex |
| ; | RightRing | RightMiddle |
| : | RightPinky | RightRing |

**Total**: 16 finger mapping errors corrected

### Menu Integration

#### Lesson Ordering (src/app.rs)

**Build sequence**:
1. ADAPTIVE (1 lesson, if ≥10 sessions)
2. FINGER TRAINING (24 lessons)
3. PRIMARY (25 lessons)
4. SECONDARY (27 lessons)

#### Dynamic Separator Positioning (src/ui/render.rs)

**Logic**:
```rust
let has_adaptive = first lesson is Adaptive type
let finger_index = if has_adaptive { 1 } else { 0 }
let primary_index = if has_adaptive { 25 } else { 24 }
let secondary_index = if has_adaptive { 50 } else { 49 }
```

**Separators**:
- ADAPTIVE (cyan) - if present, before lesson 1
- FINGER TRAINING (green) - before finger_index
- PRIMARY (cyan) - before primary_index
- SECONDARY (cyan) - before secondary_index

### Lesson Manifest

#### Complete Set (24 lessons)

**Pinky Fingers** (6 lessons):
- L1: q, m, ù, * (4 keys)
- L1+Shift: + Q, M, Ù, %, µ
- L2: + a, <, w, p, ^, $, ! (11 keys)
- L2+Shift: + uppercase + symbols
- L3: + &, é, à, ), = (16 keys)
- L3+Shift: + 1, 2, 0, °, +

**Ring Fingers** (6 lessons):
- L1: s, l (2 keys)
- L2: + z, o, x, ; (6 keys)
- L3: + ", ç (8 keys) + shift: 3, 9

**Middle Fingers** (6 lessons):
- L1: d, k (2 keys)
- L2: + e, i, c, ; (6 keys)
- L3: + ', _ (8 keys) + shift: 4, 8

**Index Fingers** (6 lessons):
- L1: f, g, h, j (4 keys)
- L2: + r, t, y, u, v, c, b, n (12 keys)
- L3: + (, -, è (16 keys) + shift: 5, 6, 7

### Configuration

#### Keyboard Display Defaults (src/ui/keyboard.rs)

```rust
impl Default for KeyboardConfig {
    fn default() -> Self {
        Self {
            _show_shift_indicators: true,
            show_heatmap: false,        // Disabled by default
            show_finger_colors: true,   // Enabled by default
            _compact_mode: false,
        }
    }
}
```

**Rationale**: Finger colors more useful for learning; heatmap available on demand (Ctrl+H)

## Implementation Files

### Modified Files
1. `src/keyboard/azerty.rs` - Corrected finger mappings (16 fixes)
2. `src/content/lesson.rs` - Added FingerPairType enum and FingerPair variant
3. `src/content/generator.rs` - Added FingerPair content generation
4. `src/content/mod.rs` - Exported finger_generator module
5. `src/app.rs` - Reordered lesson building
6. `src/ui/render.rs` - Dynamic separator positioning
7. `src/ui/keyboard.rs` - Disabled heatmap by default

### New Files
1. `src/content/finger_generator.rs` - Key extraction and drill generation (12 unit tests)

## Testing Strategy

### Unit Tests (finger_generator.rs)
- Key extraction correctness (middle fingers home row, extended, all keys)
- Shift variant inclusion
- Level progression (L3 > L2 > L1 key count)
- Placeholder filtering (\0, \n excluded)
- Content generation length limits
- All 24 lessons generate valid content

### Updated Tests
- `keyboard::azerty::tests::test_get_base_key` - French AZERTY expectations
- `keyboard::azerty::tests::test_number_row_has_13_keys` - Corrected base/shift

### Integration Testing
- Menu displays correct separators with/without adaptive
- Lesson numbering sequential
- Content generation for all 24 lessons
- Keyboard shortcuts work (Ctrl+F, Ctrl+H, Tab)

## Performance Considerations

- Lesson generation: O(n) where n = number of keys in layout
- Content generation: O(m) where m = target content length
- No runtime overhead during typing session (content pre-generated)
- Memory: ~24KB for all finger lesson metadata

## Future Enhancements

1. **Per-finger analytics**: Track accuracy/speed by finger
2. **Adaptive finger recommendations**: Suggest finger lessons based on weak keys
3. **Cross-hand patterns**: Explicit left-right alternation drills
4. **Finger strength visualization**: Heatmap overlay by finger performance
5. **Thumb timing drills**: Spacebar rhythm and spacing practice
