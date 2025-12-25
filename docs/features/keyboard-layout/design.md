# Keyboard Layout - Design Document

> **Purpose**: Technical design for keyboard layout definitions
> **Module**: `src/keyboard/`
> **Previous Step**: See `requirements.md` for layout requirements
> **Related**: See `../home-row-lessons/design.md` for content generation integration

## Overview

The keyboard layout module provides AZERTY layout definitions, particularly the home row keys used for progressive practice lessons. The design supports future extensibility to additional keyboard layouts.

## AZERTY Layout Definition

### Home Row Keys

**Key positions (AZERTY):**

```
┌───┬───┬───┬───┬───┬───┬───┬───┬───┬───┐
│ q │ s │ d │ f │ g │ h │ j │ k │ l │ m │
└───┴───┴───┴───┴───┴───┴───┴───┴───┴───┘
```

**Implementation:**

```rust
pub const AZERTY_HOME_ROW: &str = "qsdfghjklm";

pub fn get_home_row_keys() -> Vec<char> {
    AZERTY_HOME_ROW.chars().collect()
}
```

### Finger-Key Mapping

**Finger assignment for touch typing:**

```rust
use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Finger {
    LeftPinky,
    LeftRing,
    LeftMiddle,
    LeftIndex,
    RightIndex,
    RightMiddle,
    RightRing,
    RightPinky,
}

pub fn get_finger_mapping() -> HashMap<char, Finger> {
    let mut mapping = HashMap::new();

    // Left hand
    mapping.insert('q', Finger::LeftPinky);
    mapping.insert('s', Finger::LeftRing);
    mapping.insert('d', Finger::LeftMiddle);
    mapping.insert('f', Finger::LeftIndex);
    mapping.insert('g', Finger::LeftIndex);  // Stretch for left index

    // Right hand
    mapping.insert('h', Finger::RightIndex);
    mapping.insert('j', Finger::RightIndex);
    mapping.insert('k', Finger::RightMiddle);
    mapping.insert('l', Finger::RightRing);
    mapping.insert('m', Finger::RightPinky);

    mapping
}
```

### Key Groups by Difficulty

**Progressive learning groups:**

```rust
pub struct KeyGroup {
    pub name: &'static str,
    pub keys: Vec<char>,
    pub difficulty: u8,
}

pub fn get_key_groups() -> Vec<KeyGroup> {
    vec![
        KeyGroup {
            name: "Index fingers",
            keys: vec!['f', 'j'],
            difficulty: 1,
        },
        KeyGroup {
            name: "Middle fingers",
            keys: vec!['d', 'k'],
            difficulty: 2,
        },
        KeyGroup {
            name: "Ring fingers",
            keys: vec!['s', 'l'],
            difficulty: 3,
        },
        KeyGroup {
            name: "Pinkies",
            keys: vec!['q', 'm'],
            difficulty: 4,
        },
        KeyGroup {
            name: "All home row",
            keys: AZERTY_HOME_ROW.chars().collect(),
            difficulty: 5,
        },
    ]
}
```

## Design Patterns

### Separation from Lesson Logic

**Keyboard layout is data, not logic:**

```rust
// Good: Keyboard provides data
pub const AZERTY_HOME_ROW: &str = "qsdfghjklm";

// Content generator uses the data
impl HomeRowGenerator {
    fn generate(&self, level: u8) -> String {
        let keys = keyboard::get_home_row_keys();
        // Generate content using keys
    }
}
```

**Why separate:**

- Easy to add new layouts without changing lesson logic
- Clear responsibility boundaries
- Testable independently

### Constants vs Functions

**For simple data, use constants:**

```rust
pub const AZERTY_HOME_ROW: &str = "qsdfghjklm";
```

**For derived data, use functions:**

```rust
pub fn get_home_row_keys() -> Vec<char> {
    AZERTY_HOME_ROW.chars().collect()
}

pub fn is_home_row_key(c: char) -> bool {
    AZERTY_HOME_ROW.contains(c)
}
```

## Future Extensibility (Phase 2+)

### Multi-Layout Support

**Trait-based design:**

```rust
pub trait KeyboardLayout {
    fn name(&self) -> &str;
    fn home_row(&self) -> &str;
    fn finger_mapping(&self) -> HashMap<char, Finger>;
    fn key_groups(&self) -> Vec<KeyGroup>;
}

pub struct AzertyLayout;

impl KeyboardLayout for AzertyLayout {
    fn name(&self) -> &str {
        "AZERTY"
    }

    fn home_row(&self) -> &str {
        "qsdfghjklm"
    }

    fn finger_mapping(&self) -> HashMap<char, Finger> {
        get_finger_mapping()  // Current implementation
    }

    fn key_groups(&self) -> Vec<KeyGroup> {
        get_key_groups()  // Current implementation
    }
}

pub struct BepoLayout;

impl KeyboardLayout for BepoLayout {
    fn name(&self) -> &str {
        "BÉPO"
    }

    fn home_row(&self) -> &str {
        "auietsrn"  // BÉPO home row
    }

    fn finger_mapping(&self) -> HashMap<char, Finger> {
        // BÉPO-specific mapping
    }

    fn key_groups(&self) -> Vec<KeyGroup> {
        // BÉPO-specific groups
    }
}
```

### Full Keyboard Layout

**Beyond home row:**

```rust
pub struct FullKeyboard {
    pub rows: Vec<KeyRow>,
}

pub struct KeyRow {
    pub name: &'static str,
    pub keys: Vec<Key>,
}

pub struct Key {
    pub char: char,
    pub shift_char: Option<char>,
    pub finger: Finger,
}

// Example: Full AZERTY layout
pub fn get_full_azerty() -> FullKeyboard {
    FullKeyboard {
        rows: vec![
            KeyRow {
                name: "Number row",
                keys: vec![
                    Key { char: '&', shift_char: Some('1'), finger: Finger::LeftPinky },
                    // ... rest of row
                ],
            },
            KeyRow {
                name: "Top row",
                keys: vec![
                    Key { char: 'a', shift_char: Some('A'), finger: Finger::LeftPinky },
                    // ... rest of row
                ],
            },
            KeyRow {
                name: "Home row",
                keys: vec![
                    Key { char: 'q', shift_char: Some('Q'), finger: Finger::LeftPinky },
                    // ... rest of row
                ],
            },
            // ... bottom row
        ],
    }
}
```

### Configuration-Based Layouts

**JSON layout definitions:**

```json
{
  "name": "Custom AZERTY",
  "home_row": "qsdfghjklm",
  "finger_mapping": {
    "q": "left_pinky",
    "s": "left_ring",
    "d": "left_middle",
    "f": "left_index",
    "g": "left_index",
    "h": "right_index",
    "j": "right_index",
    "k": "right_middle",
    "l": "right_ring",
    "m": "right_pinky"
  }
}
```

**Loading custom layouts:**

```rust
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct LayoutConfig {
    pub name: String,
    pub home_row: String,
    pub finger_mapping: HashMap<char, String>,
}

pub fn load_layout_from_file(path: &Path) -> Result<LayoutConfig> {
    let json = fs::read_to_string(path)?;
    let layout: LayoutConfig = serde_json::from_str(&json)?;
    Ok(layout)
}
```

## Testing Strategy

### Unit Tests

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_home_row_length() {
        assert_eq!(AZERTY_HOME_ROW.len(), 10);
    }

    #[test]
    fn test_home_row_unique_chars() {
        let chars: HashSet<char> = AZERTY_HOME_ROW.chars().collect();
        assert_eq!(chars.len(), 10);  // All unique
    }

    #[test]
    fn test_finger_mapping_complete() {
        let mapping = get_finger_mapping();
        for c in AZERTY_HOME_ROW.chars() {
            assert!(mapping.contains_key(&c));
        }
    }

    #[test]
    fn test_key_groups_progressive() {
        let groups = get_key_groups();

        // Level 1: 2 keys
        assert_eq!(groups[0].keys.len(), 2);
        assert_eq!(groups[0].difficulty, 1);

        // Level 2: 2 keys (cumulative 4)
        assert_eq!(groups[1].keys.len(), 2);
        assert_eq!(groups[1].difficulty, 2);

        // Final level: all 10 keys
        assert_eq!(groups[4].keys.len(), 10);
        assert_eq!(groups[4].difficulty, 5);
    }

    #[test]
    fn test_is_home_row_key() {
        assert!(is_home_row_key('q'));
        assert!(is_home_row_key('m'));
        assert!(!is_home_row_key('a'));
        assert!(!is_home_row_key('z'));
    }
}
```

## Integration with Other Modules

### Content Generation

```rust
// In src/content/generator.rs
use crate::keyboard::azerty::{get_key_groups, get_home_row_keys};

impl HomeRowGenerator {
    pub fn generate(&self, level: u8) -> String {
        let groups = get_key_groups();
        let group = &groups[(level - 1) as usize];

        self.generate_from_keys(&group.keys)
    }
}
```

### Future Analytics

```rust
// Phase 3: Per-key statistics
use crate::keyboard::azerty::{get_finger_mapping, Finger};

pub fn analyze_finger_performance(sessions: &[SessionRecord]) -> HashMap<Finger, f64> {
    let finger_map = get_finger_mapping();

    // Calculate accuracy per finger
    let mut finger_stats: HashMap<Finger, (u32, u32)> = HashMap::new();

    for session in sessions {
        for input in &session.inputs {
            if let Some(finger) = finger_map.get(&input.expected) {
                let (correct, total) = finger_stats.entry(*finger).or_insert((0, 0));
                *total += 1;
                if input.is_correct {
                    *correct += 1;
                }
            }
        }
    }

    // Convert to accuracy percentages
    finger_stats.iter()
        .map(|(finger, (correct, total))| {
            let accuracy = (*correct as f64 / *total as f64) * 100.0;
            (*finger, accuracy)
        })
        .collect()
}
```

## File Locations

- `src/keyboard/azerty.rs` - AZERTY layout definitions
- `src/keyboard/mod.rs` - Module exports
- (Future) `src/keyboard/bepo.rs` - BÉPO layout
- (Future) `src/keyboard/trait.rs` - KeyboardLayout trait
