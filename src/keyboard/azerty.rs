use ratatui::style::Color;
use std::collections::HashMap;

/// Finger assignment for touch typing
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Finger {
    LeftPinky,
    LeftRing,
    LeftMiddle,
    LeftIndex,  // Covers 2 columns (e.g., f and g on AZERTY home row)
    RightIndex, // Covers 2 columns (e.g., h and j on AZERTY home row)
    RightMiddle,
    RightRing,
    RightPinky,
    Thumb, // Spacebar
}

/// Hand classification for shift key selection
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Hand {
    Left,
    Right,
    Either, // For spacebar - either shift works
}

impl Finger {
    /// Get the terminal color for this finger
    pub fn color(&self) -> Color {
        match self {
            Finger::LeftPinky => Color::Magenta,
            Finger::LeftRing => Color::LightBlue,
            Finger::LeftMiddle => Color::Blue,
            Finger::LeftIndex => Color::Cyan,
            Finger::RightIndex => Color::Green,
            Finger::RightMiddle => Color::Yellow,
            Finger::RightRing => Color::LightRed,
            Finger::RightPinky => Color::Red,
            Finger::Thumb => Color::Gray,
        }
    }

    /// Determine which hand uses this finger (for smart shift highlighting)
    pub fn hand(&self) -> Hand {
        match self {
            Finger::LeftPinky | Finger::LeftRing | Finger::LeftMiddle | Finger::LeftIndex => {
                Hand::Left
            }
            Finger::RightPinky | Finger::RightRing | Finger::RightMiddle | Finger::RightIndex => {
                Hand::Right
            }
            Finger::Thumb => Hand::Either,
        }
    }
}

/// Row type classification for keyboard layout
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum RowType {
    Number,   // 1234567890°=+
    Top,      // azertyuiop^$
    Home,     // qsdfghjklmù*
    Bottom,   // <wxcvbn,;:!
    Space,    // Space bar
    Modifier, // Ctrl, Cmd, Option, Space, Alt, Fn1, Fn2
}

/// Single key representation
#[derive(Debug, Clone)]
pub struct Key {
    pub base: char,
    pub shift_variant: Option<char>,
    #[allow(dead_code)]
    pub display_width: u8,
    pub finger: Finger,
}

impl Key {
    pub fn new(base: char, shift_variant: Option<char>, finger: Finger) -> Self {
        Self {
            base,
            shift_variant,
            display_width: 1,
            finger,
        }
    }
}

/// Keyboard row with keys and type
#[derive(Debug, Clone)]
pub struct KeyboardRow {
    pub keys: Vec<Key>,
    pub row_type: RowType,
}

/// Layout clavier AZERTY
/// Phase 3+: Keyboard layout abstraction for future QWERTY/other layout support
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct AzertyLayout {
    pub home_row: Vec<char>,
    pub rows: Vec<KeyboardRow>,
    pub shift_mappings: HashMap<char, char>,
}

/// Phase 3+: Keyboard layout abstraction for future QWERTY/other layout support
#[allow(dead_code)]
impl AzertyLayout {
    pub fn new() -> Self {
        let shift_mappings = Self::build_shift_mappings();

        Self {
            home_row: vec!['q', 's', 'd', 'f', 'g', 'h', 'j', 'k', 'l', 'm'], // Keep original for backward compatibility
            rows: vec![
                Self::number_row(),
                Self::top_row(),
                Self::home_row_keys(),
                Self::bottom_row(),
                Self::modifier_row(), // Replace space_row with modifier_row
            ],
            shift_mappings,
        }
    }

    pub fn get_home_row(&self) -> &[char] {
        &self.home_row
    }

    pub fn is_home_row_key(&self, c: char) -> bool {
        self.home_row.contains(&c)
    }

    /// Build number row - French AZERTY (symbols are base, numbers are shift)
    fn number_row() -> KeyboardRow {
        KeyboardRow {
            row_type: RowType::Number,
            keys: vec![
                Key::new('²', Some('³'), Finger::LeftPinky), // Superscript 2/3 (first key)
                Key::new('&', Some('1'), Finger::LeftPinky),
                Key::new('é', Some('2'), Finger::LeftPinky),
                Key::new('"', Some('3'), Finger::LeftRing),
                Key::new('\'', Some('4'), Finger::LeftMiddle),
                Key::new('(', Some('5'), Finger::LeftIndex),
                Key::new('-', Some('6'), Finger::LeftIndex),
                Key::new('è', Some('7'), Finger::RightIndex),
                Key::new('_', Some('8'), Finger::RightMiddle),
                Key::new('ç', Some('9'), Finger::RightRing),
                Key::new('à', Some('0'), Finger::RightPinky),
                Key::new(')', Some('°'), Finger::RightPinky),
                Key::new('=', Some('+'), Finger::RightPinky),
            ],
        }
    }

    /// Build top row (azertyuiop^$ + Enter)
    fn top_row() -> KeyboardRow {
        KeyboardRow {
            row_type: RowType::Top,
            keys: vec![
                Key::new('a', Some('A'), Finger::LeftPinky),
                Key::new('z', Some('Z'), Finger::LeftRing),
                Key::new('e', Some('E'), Finger::LeftMiddle),
                Key::new('r', Some('R'), Finger::LeftIndex),
                Key::new('t', Some('T'), Finger::LeftIndex),
                Key::new('y', Some('Y'), Finger::RightIndex),
                Key::new('u', Some('U'), Finger::RightIndex),
                Key::new('i', Some('I'), Finger::RightMiddle),
                Key::new('o', Some('O'), Finger::RightRing),
                Key::new('p', Some('P'), Finger::RightPinky),
                Key::new('^', Some('¨'), Finger::RightPinky),
                Key::new('$', Some('£'), Finger::RightPinky),
                Key::new('\n', None, Finger::RightPinky), // Enter key (newline character)
            ],
        }
    }

    /// Build home row (qsdfghjklmù* + Enter continuation)
    fn home_row_keys() -> KeyboardRow {
        KeyboardRow {
            row_type: RowType::Home,
            keys: vec![
                Key::new('q', Some('Q'), Finger::LeftPinky),
                Key::new('s', Some('S'), Finger::LeftRing),
                Key::new('d', Some('D'), Finger::LeftMiddle),
                Key::new('f', Some('F'), Finger::LeftIndex),
                Key::new('g', Some('G'), Finger::LeftIndex),
                Key::new('h', Some('H'), Finger::RightIndex),
                Key::new('j', Some('J'), Finger::RightIndex),
                Key::new('k', Some('K'), Finger::RightMiddle),
                Key::new('l', Some('L'), Finger::RightRing),
                Key::new('m', Some('M'), Finger::RightPinky),
                Key::new('ù', Some('%'), Finger::RightPinky),
                Key::new('*', Some('µ'), Finger::RightPinky),
                Key::new('\n', None, Finger::RightPinky), // Enter key continuation (same as top row Enter)
            ],
        }
    }

    /// Build bottom row (<wxcvbn,;:! + Right Shift placeholder)
    fn bottom_row() -> KeyboardRow {
        KeyboardRow {
            row_type: RowType::Bottom,
            keys: vec![
                Key::new('<', Some('>'), Finger::LeftPinky),
                Key::new('w', Some('W'), Finger::LeftPinky),
                Key::new('x', Some('X'), Finger::LeftRing),
                Key::new('c', Some('C'), Finger::LeftMiddle),
                Key::new('v', Some('V'), Finger::LeftIndex),
                Key::new('b', Some('B'), Finger::LeftIndex),
                Key::new('n', Some('N'), Finger::RightIndex),
                Key::new(',', Some('?'), Finger::RightIndex),
                Key::new(';', Some('.'), Finger::RightMiddle),
                Key::new(':', Some('/'), Finger::RightRing),
                Key::new('!', Some('§'), Finger::RightPinky),
                Key::new('\0', None, Finger::RightPinky), // Right Shift placeholder (null character)
            ],
        }
    }

    /// Build space bar row
    fn space_row() -> KeyboardRow {
        KeyboardRow {
            row_type: RowType::Space,
            keys: vec![
                Key::new(' ', None, Finger::Thumb), // Space bar - no shift variant
            ],
        }
    }

    /// Build modifier row (bottom row with Ctrl, Cmd, Option, etc.)
    fn modifier_row() -> KeyboardRow {
        KeyboardRow {
            row_type: RowType::Modifier,
            keys: vec![
                Key::new('\0', None, Finger::LeftPinky),  // Ctrl placeholder
                Key::new('⌘', None, Finger::RightPinky),  // Cmd
                Key::new('⌥', None, Finger::RightPinky),  // Option
                Key::new(' ', None, Finger::Thumb),       // Space
                Key::new('\0', None, Finger::RightPinky), // Alt placeholder
                Key::new('\0', None, Finger::RightPinky), // Fn1 placeholder
                Key::new('\0', None, Finger::RightPinky), // Fn2 placeholder
            ],
        }
    }

    /// Build shift mappings for all keys
    fn build_shift_mappings() -> HashMap<char, char> {
        let mut map = HashMap::new();

        // Letter mappings (a->A, b->B, etc.)
        for c in 'a'..='z' {
            map.insert(c, c.to_ascii_uppercase());
        }

        // Superscript keys
        map.insert('²', '³');

        // Number row symbols
        map.insert('1', '&');
        map.insert('2', 'é');
        map.insert('3', '"');
        map.insert('4', '\'');
        map.insert('5', '(');
        map.insert('6', '-');
        map.insert('7', 'è');
        map.insert('8', '_');
        map.insert('9', 'ç');
        map.insert('0', 'à');
        map.insert('°', ')');
        map.insert('=', '+');

        // Punctuation and special characters
        map.insert(',', '?');
        map.insert(';', '.');
        map.insert(':', '/');
        map.insert('!', '§');
        map.insert('<', '>');
        map.insert('^', '¨');
        map.insert('$', '£');
        map.insert('ù', '%');
        map.insert('*', 'µ');

        map
    }

    /// Find the base key for a given character (handles shift variants)
    pub fn get_base_key(&self, c: char) -> Option<char> {
        // Direct match
        for row in &self.rows {
            for key in &row.keys {
                if key.base == c {
                    return Some(key.base);
                }
                if key.shift_variant == Some(c) {
                    return Some(key.base);
                }
            }
        }
        None
    }

    /// Check if character requires shift
    pub fn requires_shift(&self, c: char) -> bool {
        self.shift_mappings.values().any(|&v| v == c)
    }

    /// Find the Key object for a given base character (for smart shift highlighting)
    pub fn find_key(&self, base_char: char) -> Option<&Key> {
        for row in &self.rows {
            for key in &row.keys {
                if key.base == base_char {
                    return Some(key);
                }
            }
        }
        None
    }
}

impl Default for AzertyLayout {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_azerty_home_row() {
        let layout = AzertyLayout::new();
        assert_eq!(layout.home_row.len(), 10);
        assert_eq!(layout.home_row[0], 'q');
        assert_eq!(layout.home_row[9], 'm');
    }

    #[test]
    fn test_is_home_row_key() {
        let layout = AzertyLayout::new();
        assert!(layout.is_home_row_key('f'));
        assert!(layout.is_home_row_key('j'));
        assert!(layout.is_home_row_key('q'));
        assert!(!layout.is_home_row_key('a'));
        assert!(!layout.is_home_row_key('z'));
    }

    #[test]
    fn test_all_rows_populated() {
        let layout = AzertyLayout::new();
        assert_eq!(layout.rows.len(), 5);
        assert_eq!(layout.rows[0].row_type, RowType::Number);
        assert_eq!(layout.rows[1].row_type, RowType::Top);
        assert_eq!(layout.rows[2].row_type, RowType::Home);
        assert_eq!(layout.rows[3].row_type, RowType::Bottom);
        assert_eq!(layout.rows[4].row_type, RowType::Modifier);
    }

    #[test]
    fn test_number_row_has_13_keys() {
        let layout = AzertyLayout::new();
        assert_eq!(layout.rows[0].keys.len(), 13);
        // First key should be ²
        assert_eq!(layout.rows[0].keys[0].base, '²');
        assert_eq!(layout.rows[0].keys[0].shift_variant, Some('³'));
        // 12th key (index 11) - French AZERTY: ) is base, ° is shift
        assert_eq!(layout.rows[0].keys[11].base, ')');
        assert_eq!(layout.rows[0].keys[11].shift_variant, Some('°'));
        // Last key should be =
        assert_eq!(layout.rows[0].keys[12].base, '=');
        assert_eq!(layout.rows[0].keys[12].shift_variant, Some('+'));
    }

    #[test]
    fn test_shift_mapping_letters() {
        let layout = AzertyLayout::new();
        assert_eq!(layout.shift_mappings.get(&'a'), Some(&'A'));
        assert_eq!(layout.shift_mappings.get(&'z'), Some(&'Z'));
        assert_eq!(layout.shift_mappings.get(&'q'), Some(&'Q'));
        assert_eq!(layout.shift_mappings.get(&'m'), Some(&'M'));
    }

    #[test]
    fn test_shift_mapping_numbers() {
        let layout = AzertyLayout::new();
        assert_eq!(layout.shift_mappings.get(&'²'), Some(&'³'));
        assert_eq!(layout.shift_mappings.get(&'1'), Some(&'&'));
        assert_eq!(layout.shift_mappings.get(&'5'), Some(&'('));
        assert_eq!(layout.shift_mappings.get(&'0'), Some(&'à'));
    }

    #[test]
    fn test_shift_mapping_special_chars() {
        let layout = AzertyLayout::new();
        assert_eq!(layout.shift_mappings.get(&'<'), Some(&'>'));
        assert_eq!(layout.shift_mappings.get(&'^'), Some(&'¨'));
        assert_eq!(layout.shift_mappings.get(&'$'), Some(&'£'));
        assert_eq!(layout.shift_mappings.get(&'ù'), Some(&'%'));
        assert_eq!(layout.shift_mappings.get(&'*'), Some(&'µ'));
    }

    #[test]
    fn test_get_base_key() {
        let layout = AzertyLayout::new();
        // Base characters (French AZERTY: symbols are base, numbers are shift)
        assert_eq!(layout.get_base_key('²'), Some('²'));
        assert_eq!(layout.get_base_key('a'), Some('a'));
        assert_eq!(layout.get_base_key('q'), Some('q'));
        assert_eq!(layout.get_base_key('&'), Some('&')); // Base character
        assert_eq!(layout.get_base_key('é'), Some('é')); // Base character
                                                         // Shift variants (numbers require shift on French AZERTY)
        assert_eq!(layout.get_base_key('³'), Some('²'));
        assert_eq!(layout.get_base_key('A'), Some('a'));
        assert_eq!(layout.get_base_key('Q'), Some('q'));
        assert_eq!(layout.get_base_key('1'), Some('&')); // 1 is shift of &
        assert_eq!(layout.get_base_key('2'), Some('é')); // 2 is shift of é
                                                         // Special characters
        assert_eq!(layout.get_base_key('<'), Some('<'));
        assert_eq!(layout.get_base_key('>'), Some('<'));
        assert_eq!(layout.get_base_key('%'), Some('ù'));
        assert_eq!(layout.get_base_key('µ'), Some('*'));
    }

    #[test]
    fn test_requires_shift() {
        let layout = AzertyLayout::new();
        // Uppercase letters require shift
        assert!(layout.requires_shift('A'));
        assert!(layout.requires_shift('Z'));
        assert!(layout.requires_shift('Q'));
        // Number symbols require shift
        assert!(layout.requires_shift('³'));
        assert!(layout.requires_shift('&'));
        assert!(layout.requires_shift('('));
        assert!(layout.requires_shift('é'));
        // Special character shift variants
        assert!(layout.requires_shift('>'));
        assert!(layout.requires_shift('%'));
        assert!(layout.requires_shift('µ'));
        // Base characters don't require shift
        assert!(!layout.requires_shift('²'));
        assert!(!layout.requires_shift('a'));
        assert!(!layout.requires_shift('1'));
        assert!(!layout.requires_shift('q'));
        assert!(!layout.requires_shift('<'));
        assert!(!layout.requires_shift('ù'));
        assert!(!layout.requires_shift('*'));
        assert!(!layout.requires_shift('°'));
    }

    #[test]
    fn test_finger_hand_detection() {
        // Test left hand fingers
        assert_eq!(Finger::LeftPinky.hand(), Hand::Left);
        assert_eq!(Finger::LeftRing.hand(), Hand::Left);
        assert_eq!(Finger::LeftMiddle.hand(), Hand::Left);
        assert_eq!(Finger::LeftIndex.hand(), Hand::Left);
        // Test right hand fingers
        assert_eq!(Finger::RightPinky.hand(), Hand::Right);
        assert_eq!(Finger::RightRing.hand(), Hand::Right);
        assert_eq!(Finger::RightMiddle.hand(), Hand::Right);
        assert_eq!(Finger::RightIndex.hand(), Hand::Right);
        // Test thumb (either hand)
        assert_eq!(Finger::Thumb.hand(), Hand::Either);
    }

    #[test]
    fn test_find_key() {
        let layout = AzertyLayout::new();

        // Test finding left hand keys
        let key_q = layout.find_key('q');
        assert!(key_q.is_some());
        assert_eq!(key_q.unwrap().base, 'q');
        assert_eq!(key_q.unwrap().finger, Finger::LeftPinky);

        let key_s = layout.find_key('s');
        assert!(key_s.is_some());
        assert_eq!(key_s.unwrap().finger, Finger::LeftRing);

        let key_d = layout.find_key('d');
        assert!(key_d.is_some());
        assert_eq!(key_d.unwrap().finger, Finger::LeftMiddle);

        let key_f = layout.find_key('f');
        assert!(key_f.is_some());
        assert_eq!(key_f.unwrap().finger, Finger::LeftIndex);

        // Test finding right hand keys
        let key_j = layout.find_key('j');
        assert!(key_j.is_some());
        assert_eq!(key_j.unwrap().finger, Finger::RightIndex);

        let key_k = layout.find_key('k');
        assert!(key_k.is_some());
        assert_eq!(key_k.unwrap().finger, Finger::RightMiddle);

        let key_l = layout.find_key('l');
        assert!(key_l.is_some());
        assert_eq!(key_l.unwrap().finger, Finger::RightRing);

        let key_m = layout.find_key('m');
        assert!(key_m.is_some());
        assert_eq!(key_m.unwrap().finger, Finger::RightPinky);

        // Test finding spacebar (thumb)
        let key_space = layout.find_key(' ');
        assert!(key_space.is_some());
        assert_eq!(key_space.unwrap().finger, Finger::Thumb);

        // Test non-existent key
        let key_none = layout.find_key('€');
        assert!(key_none.is_none());
    }
}
