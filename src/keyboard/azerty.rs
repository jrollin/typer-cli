/// Layout clavier AZERTY
/// Phase 3+: Keyboard layout abstraction for future QWERTY/other layout support
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct AzertyLayout {
    pub home_row: Vec<char>,
}

/// Phase 3+: Keyboard layout abstraction for future QWERTY/other layout support
#[allow(dead_code)]
impl AzertyLayout {
    pub fn new() -> Self {
        Self {
            home_row: vec!['q', 's', 'd', 'f', 'g', 'h', 'j', 'k', 'l', 'm'],
        }
    }

    pub fn get_home_row(&self) -> &[char] {
        &self.home_row
    }

    pub fn is_home_row_key(&self, c: char) -> bool {
        self.home_row.contains(&c)
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
}
