use std::collections::HashMap;

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
}

impl Key {
    pub fn new(base: char, shift_variant: Option<char>) -> Self {
        Self {
            base,
            shift_variant,
            display_width: 1,
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

    /// Build number row (²1234567890°=) - displays shift variants by default
    fn number_row() -> KeyboardRow {
        KeyboardRow {
            row_type: RowType::Number,
            keys: vec![
                Key::new('²', Some('³')), // Superscript 2/3 (first key)
                Key::new('1', Some('&')),
                Key::new('2', Some('é')),
                Key::new('3', Some('"')),
                Key::new('4', Some('\'')),
                Key::new('5', Some('(')),
                Key::new('6', Some('-')),
                Key::new('7', Some('è')),
                Key::new('8', Some('_')),
                Key::new('9', Some('ç')),
                Key::new('0', Some('à')),
                Key::new('°', Some(')')),
                Key::new('=', Some('+')),
            ],
        }
    }

    /// Build top row (azertyuiop^$ + Enter)
    fn top_row() -> KeyboardRow {
        KeyboardRow {
            row_type: RowType::Top,
            keys: vec![
                Key::new('a', Some('A')),
                Key::new('z', Some('Z')),
                Key::new('e', Some('E')),
                Key::new('r', Some('R')),
                Key::new('t', Some('T')),
                Key::new('y', Some('Y')),
                Key::new('u', Some('U')),
                Key::new('i', Some('I')),
                Key::new('o', Some('O')),
                Key::new('p', Some('P')),
                Key::new('^', Some('¨')),
                Key::new('$', Some('£')),
                Key::new('\n', None), // Enter key (newline character)
            ],
        }
    }

    /// Build home row (qsdfghjklmù* + Enter continuation)
    fn home_row_keys() -> KeyboardRow {
        KeyboardRow {
            row_type: RowType::Home,
            keys: vec![
                Key::new('q', Some('Q')),
                Key::new('s', Some('S')),
                Key::new('d', Some('D')),
                Key::new('f', Some('F')),
                Key::new('g', Some('G')),
                Key::new('h', Some('H')),
                Key::new('j', Some('J')),
                Key::new('k', Some('K')),
                Key::new('l', Some('L')),
                Key::new('m', Some('M')),
                Key::new('ù', Some('%')),
                Key::new('*', Some('µ')),
                Key::new('\n', None), // Enter key continuation (same as top row Enter)
            ],
        }
    }

    /// Build bottom row (<wxcvbn,;:! + Right Shift placeholder)
    fn bottom_row() -> KeyboardRow {
        KeyboardRow {
            row_type: RowType::Bottom,
            keys: vec![
                Key::new('<', Some('>')),
                Key::new('w', Some('W')),
                Key::new('x', Some('X')),
                Key::new('c', Some('C')),
                Key::new('v', Some('V')),
                Key::new('b', Some('B')),
                Key::new('n', Some('N')),
                Key::new(',', Some('?')),
                Key::new(';', Some('.')),
                Key::new(':', Some('/')),
                Key::new('!', Some('§')),
                Key::new('\0', None), // Right Shift placeholder (null character)
            ],
        }
    }

    /// Build space bar row
    fn space_row() -> KeyboardRow {
        KeyboardRow {
            row_type: RowType::Space,
            keys: vec![
                Key::new(' ', None), // Space bar - no shift variant
            ],
        }
    }

    /// Build modifier row (bottom row with Ctrl, Cmd, Option, etc.)
    fn modifier_row() -> KeyboardRow {
        KeyboardRow {
            row_type: RowType::Modifier,
            keys: vec![
                Key::new('\0', None), // Ctrl placeholder
                Key::new('⌘', None),  // Cmd
                Key::new('⌥', None),  // Option
                Key::new(' ', None),  // Space
                Key::new('\0', None), // Alt placeholder
                Key::new('\0', None), // Fn1 placeholder
                Key::new('\0', None), // Fn2 placeholder
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
        // 12th key (index 11) should be °
        assert_eq!(layout.rows[0].keys[11].base, '°');
        assert_eq!(layout.rows[0].keys[11].shift_variant, Some(')'));
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
        // Base characters
        assert_eq!(layout.get_base_key('²'), Some('²'));
        assert_eq!(layout.get_base_key('a'), Some('a'));
        assert_eq!(layout.get_base_key('q'), Some('q'));
        assert_eq!(layout.get_base_key('1'), Some('1'));
        // Shift variants
        assert_eq!(layout.get_base_key('³'), Some('²'));
        assert_eq!(layout.get_base_key('A'), Some('a'));
        assert_eq!(layout.get_base_key('Q'), Some('q'));
        assert_eq!(layout.get_base_key('&'), Some('1'));
        assert_eq!(layout.get_base_key('é'), Some('2'));
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
}
