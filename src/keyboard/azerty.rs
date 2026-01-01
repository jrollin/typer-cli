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
            Finger::LeftPinky => Color::Magenta,        // Purple/pink
            Finger::LeftRing => Color::LightCyan,       // Bright aqua
            Finger::LeftMiddle => Color::Blue,          // Dark blue
            Finger::LeftIndex => Color::Green,          // Green
            Finger::RightIndex => Color::Yellow,        // Bright yellow
            Finger::RightMiddle => Color::LightMagenta, // Pink/light purple
            Finger::RightRing => Color::LightRed,       // Orange-red
            Finger::RightPinky => Color::White,         // Bright white
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
    pub altgr_variant: Option<char>,
    #[allow(dead_code)]
    pub display_width: u8,
    pub finger: Finger,
}

impl Key {
    pub fn new(
        base: char,
        shift_variant: Option<char>,
        altgr_variant: Option<char>,
        finger: Finger,
    ) -> Self {
        Self {
            base,
            shift_variant,
            altgr_variant,
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
    pub altgr_mappings: HashMap<char, char>,
}

/// Phase 3+: Keyboard layout abstraction for future QWERTY/other layout support
#[allow(dead_code)]
impl AzertyLayout {
    pub fn new() -> Self {
        let shift_mappings = Self::build_shift_mappings();
        let altgr_mappings = Self::build_altgr_mappings();

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
            altgr_mappings,
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
                Key::new('²', Some('³'), None, Finger::LeftPinky), // ² key (replaces Esc)
                Key::new('&', Some('1'), None, Finger::LeftPinky), // No AltGr in standard
                Key::new('é', Some('2'), Some('~'), Finger::LeftPinky),
                Key::new('"', Some('3'), Some('#'), Finger::LeftRing),
                Key::new('\'', Some('4'), Some('{'), Finger::LeftMiddle),
                Key::new('(', Some('5'), Some('['), Finger::LeftIndex),
                Key::new('-', Some('6'), Some('|'), Finger::LeftIndex),
                Key::new('è', Some('7'), Some('`'), Finger::RightIndex),
                Key::new('_', Some('8'), Some('\\'), Finger::RightIndex),
                Key::new('ç', Some('9'), Some('^'), Finger::RightMiddle),
                Key::new('à', Some('0'), Some('@'), Finger::RightRing),
                Key::new(')', Some('°'), Some(']'), Finger::RightPinky),
                Key::new('=', Some('+'), Some('}'), Finger::RightPinky),
            ],
        }
    }

    /// Build top row (azertyuiop^$ + Enter)
    fn top_row() -> KeyboardRow {
        KeyboardRow {
            row_type: RowType::Top,
            keys: vec![
                Key::new('a', Some('A'), None, Finger::LeftPinky),
                Key::new('z', Some('Z'), None, Finger::LeftRing),
                Key::new('e', Some('E'), None, Finger::LeftMiddle),
                Key::new('r', Some('R'), None, Finger::LeftIndex),
                Key::new('t', Some('T'), None, Finger::LeftIndex),
                Key::new('y', Some('Y'), None, Finger::RightIndex),
                Key::new('u', Some('U'), None, Finger::RightIndex),
                Key::new('i', Some('I'), None, Finger::RightMiddle),
                Key::new('o', Some('O'), None, Finger::RightRing),
                Key::new('p', Some('P'), None, Finger::RightPinky),
                Key::new('^', Some('¨'), None, Finger::RightPinky),
                Key::new('$', Some('£'), None, Finger::RightPinky),
                Key::new('\n', None, None, Finger::RightPinky), // Enter key (newline character)
            ],
        }
    }

    /// Build home row (qsdfghjklmù* + Enter continuation)
    fn home_row_keys() -> KeyboardRow {
        KeyboardRow {
            row_type: RowType::Home,
            keys: vec![
                Key::new('q', Some('Q'), None, Finger::LeftPinky),
                Key::new('s', Some('S'), None, Finger::LeftRing),
                Key::new('d', Some('D'), None, Finger::LeftMiddle),
                Key::new('f', Some('F'), None, Finger::LeftIndex),
                Key::new('g', Some('G'), None, Finger::LeftIndex),
                Key::new('h', Some('H'), None, Finger::RightIndex),
                Key::new('j', Some('J'), None, Finger::RightIndex),
                Key::new('k', Some('K'), None, Finger::RightMiddle),
                Key::new('l', Some('L'), None, Finger::RightRing),
                Key::new('m', Some('M'), None, Finger::RightPinky),
                Key::new('ù', Some('%'), None, Finger::RightPinky),
                Key::new('*', Some('µ'), None, Finger::RightPinky),
                Key::new('\n', None, None, Finger::RightPinky), // Enter key continuation (same as top row Enter)
            ],
        }
    }

    /// Build bottom row (<wxcvbn,;:! + Right Shift placeholder)
    fn bottom_row() -> KeyboardRow {
        KeyboardRow {
            row_type: RowType::Bottom,
            keys: vec![
                Key::new('<', Some('>'), None, Finger::LeftPinky),
                Key::new('w', Some('W'), None, Finger::LeftPinky),
                Key::new('x', Some('X'), None, Finger::LeftRing),
                Key::new('c', Some('C'), None, Finger::LeftMiddle),
                Key::new('v', Some('V'), None, Finger::LeftIndex),
                Key::new('b', Some('B'), None, Finger::LeftIndex),
                Key::new('n', Some('N'), None, Finger::RightIndex),
                Key::new(',', Some('?'), None, Finger::RightIndex),
                Key::new(';', Some('.'), None, Finger::RightMiddle),
                Key::new(':', Some('/'), None, Finger::RightRing),
                Key::new('!', Some('§'), None, Finger::RightPinky),
                Key::new('\0', None, None, Finger::RightPinky), // Right Shift placeholder (null character)
            ],
        }
    }

    /// Build space bar row
    fn space_row() -> KeyboardRow {
        KeyboardRow {
            row_type: RowType::Space,
            keys: vec![
                Key::new(' ', None, None, Finger::Thumb), // Space bar - no shift variant
            ],
        }
    }

    /// Build modifier row (bottom row with Ctrl, Cmd, Option, etc.)
    fn modifier_row() -> KeyboardRow {
        KeyboardRow {
            row_type: RowType::Modifier,
            keys: vec![
                Key::new('\0', None, None, Finger::LeftPinky), // Ctrl placeholder
                Key::new('⌘', None, None, Finger::RightPinky), // Cmd
                Key::new('⌥', None, None, Finger::RightPinky), // Option
                Key::new(' ', None, None, Finger::Thumb),      // Space
                Key::new('\0', None, None, Finger::RightPinky), // Alt placeholder
                Key::new('\0', None, None, Finger::RightPinky), // Fn1 placeholder
                Key::new('\0', None, None, Finger::RightPinky), // Fn2 placeholder
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

        // Number row symbols (base -> shift variant)
        map.insert('&', '1');
        map.insert('é', '2');
        map.insert('"', '3');
        map.insert('\'', '4');
        map.insert('(', '5');
        map.insert('-', '6');
        map.insert('è', '7');
        map.insert('_', '8');
        map.insert('ç', '9');
        map.insert('à', '0');
        map.insert(')', '°');
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

    /// Build AltGr mappings for number row keys (standard French AZERTY)
    fn build_altgr_mappings() -> HashMap<char, char> {
        let mut map = HashMap::new();

        // Standard French AZERTY AltGr mappings (number row only)
        // ² key has no AltGr mapping
        // & (1 key) has no AltGr mapping in standard layout
        map.insert('é', '~'); // 2 key
        map.insert('"', '#'); // 3 key
        map.insert('\'', '{'); // 4 key
        map.insert('(', '['); // 5 key
        map.insert('-', '|'); // 6 key
        map.insert('è', '`'); // 7 key
        map.insert('_', '\\'); // 8 key
        map.insert('ç', '^'); // 9 key
        map.insert('à', '@'); // 0 key
        map.insert(')', ']'); // - key
        map.insert('=', '}'); // = key

        map
    }

    /// Find the base key for a given character (handles shift and AltGr variants)
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
                if key.altgr_variant == Some(c) {
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

    /// Check if character requires AltGr
    pub fn requires_altgr(&self, c: char) -> bool {
        self.altgr_mappings.values().any(|&v| v == c)
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
        // Second key should be &
        assert_eq!(layout.rows[0].keys[1].base, '&');
        assert_eq!(layout.rows[0].keys[1].shift_variant, Some('1'));
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
        // On AZERTY: base -> shift variant (symbol -> number)
        assert_eq!(layout.shift_mappings.get(&'&'), Some(&'1'));
        assert_eq!(layout.shift_mappings.get(&'('), Some(&'5'));
        assert_eq!(layout.shift_mappings.get(&'à'), Some(&'0'));
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
        assert_eq!(layout.get_base_key('a'), Some('a'));
        assert_eq!(layout.get_base_key('q'), Some('q'));
        assert_eq!(layout.get_base_key('&'), Some('&')); // Base character
        assert_eq!(layout.get_base_key('é'), Some('é')); // Base character
                                                         // Shift variants (numbers require shift on French AZERTY)
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
        // Numbers require shift on AZERTY (shift variants of symbols)
        assert!(layout.requires_shift('1'));
        assert!(layout.requires_shift('2'));
        assert!(layout.requires_shift('3'));
        assert!(layout.requires_shift('5'));
        assert!(layout.requires_shift('0'));
        assert!(layout.requires_shift('³'));
        // Special character shift variants
        assert!(layout.requires_shift('>'));
        assert!(layout.requires_shift('%'));
        assert!(layout.requires_shift('µ'));
        assert!(layout.requires_shift('+'));
        assert!(layout.requires_shift('°'));
        // Base characters don't require shift
        assert!(!layout.requires_shift('²'));
        assert!(!layout.requires_shift('a'));
        assert!(!layout.requires_shift('q'));
        assert!(!layout.requires_shift('<'));
        assert!(!layout.requires_shift('ù'));
        assert!(!layout.requires_shift('*'));
        assert!(!layout.requires_shift('&'));
        assert!(!layout.requires_shift('é'));
        assert!(!layout.requires_shift('('));
        assert!(!layout.requires_shift('='));
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

    #[test]
    fn test_altgr_mapping_number_row() {
        let layout = AzertyLayout::new();
        // Standard French AZERTY AltGr mappings
        // ² and & keys have no AltGr mapping
        assert_eq!(layout.altgr_mappings.get(&'é'), Some(&'~'));
        assert_eq!(layout.altgr_mappings.get(&'"'), Some(&'#'));
        assert_eq!(layout.altgr_mappings.get(&'\''), Some(&'{'));
        assert_eq!(layout.altgr_mappings.get(&'('), Some(&'['));
        assert_eq!(layout.altgr_mappings.get(&'-'), Some(&'|'));
        assert_eq!(layout.altgr_mappings.get(&'è'), Some(&'`'));
        assert_eq!(layout.altgr_mappings.get(&'_'), Some(&'\\'));
        assert_eq!(layout.altgr_mappings.get(&'ç'), Some(&'^'));
        assert_eq!(layout.altgr_mappings.get(&'à'), Some(&'@'));
        assert_eq!(layout.altgr_mappings.get(&')'), Some(&']'));
        assert_eq!(layout.altgr_mappings.get(&'='), Some(&'}'));
    }

    #[test]
    fn test_requires_altgr() {
        let layout = AzertyLayout::new();
        // AltGr characters (standard French AZERTY)
        assert!(layout.requires_altgr('@'));
        assert!(layout.requires_altgr('#'));
        assert!(layout.requires_altgr('{'));
        assert!(layout.requires_altgr('['));
        assert!(layout.requires_altgr('|'));
        assert!(layout.requires_altgr('`'));
        assert!(layout.requires_altgr('\\'));
        assert!(layout.requires_altgr('^'));
        assert!(layout.requires_altgr('~'));
        assert!(layout.requires_altgr(']'));
        assert!(layout.requires_altgr('}'));
        // Non-AltGr characters
        assert!(!layout.requires_altgr('a'));
        assert!(!layout.requires_altgr('&'));
        assert!(!layout.requires_altgr('1'));
        assert!(!layout.requires_altgr('²'));
        assert!(!layout.requires_altgr('³'));
    }

    #[test]
    fn test_get_base_key_with_altgr() {
        let layout = AzertyLayout::new();
        assert_eq!(layout.get_base_key('@'), Some('à'));
        assert_eq!(layout.get_base_key('#'), Some('"'));
        assert_eq!(layout.get_base_key('{'), Some('\''));
        assert_eq!(layout.get_base_key('['), Some('('));
        assert_eq!(layout.get_base_key('|'), Some('-'));
        assert_eq!(layout.get_base_key('`'), Some('è'));
        assert_eq!(layout.get_base_key('\\'), Some('_'));
        assert_eq!(layout.get_base_key('^'), Some('ç'));
        assert_eq!(layout.get_base_key('~'), Some('é'));
        assert_eq!(layout.get_base_key(']'), Some(')'));
        assert_eq!(layout.get_base_key('}'), Some('='));
    }

    #[test]
    fn test_number_row_has_altgr_variants() {
        let layout = AzertyLayout::new();
        let number_row = &layout.rows[0];
        // ² key (position 0) has no AltGr
        assert_eq!(number_row.keys[0].altgr_variant, None);
        // & key (position 1) has no AltGr in standard layout
        assert_eq!(number_row.keys[1].altgr_variant, None);
        // é key (position 2) has ~ with AltGr
        assert_eq!(number_row.keys[2].altgr_variant, Some('~'));
        // à key (position 10) has @ with AltGr
        assert_eq!(number_row.keys[10].altgr_variant, Some('@'));
        // = key (position 12) has } with AltGr
        assert_eq!(number_row.keys[12].altgr_variant, Some('}'));
    }

    #[test]
    fn test_number_row_has_superscript_key() {
        let layout = AzertyLayout::new();
        let number_row = &layout.rows[0];
        // First key should be ² with shift variant ³
        assert_eq!(number_row.keys[0].base, '²');
        assert_eq!(number_row.keys[0].shift_variant, Some('³'));
    }

    #[test]
    fn test_number_row_count() {
        let layout = AzertyLayout::new();
        let number_row = &layout.rows[0];
        // Should have 13 keys now (was 12)
        assert_eq!(number_row.keys.len(), 13);
    }

    #[test]
    fn test_other_rows_no_altgr() {
        let layout = AzertyLayout::new();
        for key in &layout.rows[1].keys {
            assert_eq!(key.altgr_variant, None);
        }
    }

    #[test]
    fn test_shift_character_highlighting() {
        let layout = AzertyLayout::new();

        // Test that '/' (shift variant of ':') is correctly detected
        let next_char = '/';
        let requires_shift = layout.requires_shift(next_char);
        assert!(requires_shift, "'/' should require shift");

        // Find the ':' key
        let base_key = layout.get_base_key(next_char);
        assert_eq!(base_key, Some(':'), "Base key for '/' should be ':'");

        // Verify the ':' key has '/' as shift variant
        let colon_key = layout.find_key(':').unwrap();
        assert_eq!(colon_key.shift_variant, Some('/'));
    }

    #[test]
    fn test_altgr_character_highlighting() {
        let layout = AzertyLayout::new();

        // Test that '{' (altgr variant of ''') is correctly detected
        let next_char = '{';
        let requires_altgr = layout.requires_altgr(next_char);
        assert!(requires_altgr, "'{{' should require AltGr");

        // Find the ''' key
        let base_key = layout.get_base_key(next_char);
        assert_eq!(base_key, Some('\''), "Base key for '{{' should be '''");

        // Verify the ''' key has '{' as altgr variant
        let apostrophe_key = layout.find_key('\'').unwrap();
        assert_eq!(apostrophe_key.altgr_variant, Some('{'));
    }

    #[test]
    fn test_number_highlighting() {
        let layout = AzertyLayout::new();

        // Test that '3' (shift variant of '"') is correctly detected
        let next_char = '3';
        let requires_shift = layout.requires_shift(next_char);
        assert!(requires_shift, "'3' should require shift on AZERTY");

        // Find the '"' key
        let base_key = layout.get_base_key(next_char);
        assert_eq!(base_key, Some('"'), "Base key for '3' should be '\"'");

        // Verify the '"' key has '3' as shift variant
        let quote_key = layout.find_key('"').unwrap();
        assert_eq!(quote_key.shift_variant, Some('3'));

        // Test another number
        let next_char2 = '5';
        let base_key2 = layout.get_base_key(next_char2);
        assert_eq!(base_key2, Some('('), "Base key for '5' should be '('");

        let paren_key = layout.find_key('(').unwrap();
        assert_eq!(paren_key.shift_variant, Some('5'));
    }
}
