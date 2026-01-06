use super::bigram::{BigramType, Language};
use super::code_symbols::ProgrammingLanguage;

/// Map between normal and shifted characters for AZERTY
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct ShiftedCharMap {
    pub normal: char,
    pub shifted: char,
}

/// AZERTY keyboard shift mappings
#[allow(dead_code)]
pub const AZERTY_SHIFT_MAP: &[ShiftedCharMap] = &[
    // Letters become uppercase
    ShiftedCharMap {
        normal: 'a',
        shifted: 'A',
    },
    ShiftedCharMap {
        normal: 'b',
        shifted: 'B',
    },
    ShiftedCharMap {
        normal: 'c',
        shifted: 'C',
    },
    ShiftedCharMap {
        normal: 'd',
        shifted: 'D',
    },
    ShiftedCharMap {
        normal: 'e',
        shifted: 'E',
    },
    ShiftedCharMap {
        normal: 'f',
        shifted: 'F',
    },
    ShiftedCharMap {
        normal: 'g',
        shifted: 'G',
    },
    ShiftedCharMap {
        normal: 'h',
        shifted: 'H',
    },
    ShiftedCharMap {
        normal: 'i',
        shifted: 'I',
    },
    ShiftedCharMap {
        normal: 'j',
        shifted: 'J',
    },
    ShiftedCharMap {
        normal: 'k',
        shifted: 'K',
    },
    ShiftedCharMap {
        normal: 'l',
        shifted: 'L',
    },
    ShiftedCharMap {
        normal: 'm',
        shifted: 'M',
    },
    ShiftedCharMap {
        normal: 'n',
        shifted: 'N',
    },
    ShiftedCharMap {
        normal: 'o',
        shifted: 'O',
    },
    ShiftedCharMap {
        normal: 'p',
        shifted: 'P',
    },
    ShiftedCharMap {
        normal: 'q',
        shifted: 'Q',
    },
    ShiftedCharMap {
        normal: 'r',
        shifted: 'R',
    },
    ShiftedCharMap {
        normal: 's',
        shifted: 'S',
    },
    ShiftedCharMap {
        normal: 't',
        shifted: 'T',
    },
    ShiftedCharMap {
        normal: 'u',
        shifted: 'U',
    },
    ShiftedCharMap {
        normal: 'v',
        shifted: 'V',
    },
    ShiftedCharMap {
        normal: 'w',
        shifted: 'W',
    },
    ShiftedCharMap {
        normal: 'x',
        shifted: 'X',
    },
    ShiftedCharMap {
        normal: 'y',
        shifted: 'Y',
    },
    ShiftedCharMap {
        normal: 'z',
        shifted: 'Z',
    },
    // AZERTY number row (symbols by default, numbers when shifted)
    ShiftedCharMap {
        normal: '&',
        shifted: '1',
    },
    ShiftedCharMap {
        normal: 'é',
        shifted: '2',
    },
    ShiftedCharMap {
        normal: '"',
        shifted: '3',
    },
    ShiftedCharMap {
        normal: '\'',
        shifted: '4',
    },
    ShiftedCharMap {
        normal: '(',
        shifted: '5',
    },
    ShiftedCharMap {
        normal: '-',
        shifted: '6',
    },
    ShiftedCharMap {
        normal: 'è',
        shifted: '7',
    },
    ShiftedCharMap {
        normal: '_',
        shifted: '8',
    },
    ShiftedCharMap {
        normal: 'ç',
        shifted: '9',
    },
    ShiftedCharMap {
        normal: 'à',
        shifted: '0',
    },
    // Other symbols
    ShiftedCharMap {
        normal: ';',
        shifted: '.',
    },
    ShiftedCharMap {
        normal: ':',
        shifted: '/',
    },
    ShiftedCharMap {
        normal: '!',
        shifted: '§',
    },
];

/// Get shifted variant of a character
#[allow(dead_code)]
pub fn get_shifted_char(c: char) -> Option<char> {
    AZERTY_SHIFT_MAP
        .iter()
        .find(|map| map.normal == c)
        .map(|map| map.shifted)
}

/// Finger pair combinations for bilateral training (left + right)
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum FingerPairType {
    Pinky,  // Left pinky + Right pinky
    Ring,   // Left ring + Right ring
    Middle, // Left middle + Right middle
    Index,  // Left index + Right index
}

/// Row progression levels for progressive keyboard learning
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum RowLevel {
    Level1, // Home row only
    Level2, // Home + Top
    Level3, // Home + Top + Bottom
    Level4, // All rows (+ Numbers)
}

/// Types de leçons disponibles
#[derive(Debug, Clone, PartialEq)]
pub enum LessonType {
    Bigram {
        bigram_type: BigramType,
        language: Option<Language>,
        level: usize,
    },
    Trigram {
        language: Language,
        level: usize,
    },
    CommonWords {
        language: Language,
        level: usize,
    },
    CodeSymbols {
        language: ProgrammingLanguage,
        level: usize,
    },
    Adaptive,
    /// Finger-based training by finger pair, level, and shift variant
    FingerPair {
        finger_pair: FingerPairType,
        level: u8,        // 1=Home Row, 2=Extended, 3=All Keys
        with_shift: bool, // false=base chars, true=mixed case+symbols
    },
    /// Row progression training (all fingers, progressive rows)
    RowProgression {
        level: RowLevel,
        with_shift: bool, // false=base chars, true=mixed case+symbols
    },
    /// User-provided custom markdown lessons
    Custom {
        content: String,
    },
}

/// Représente une leçon de typing
#[derive(Debug, Clone)]
pub struct Lesson {
    pub lesson_type: LessonType,
    /// Public API: Lesson metadata for future UI tooltips and analytics export
    #[allow(dead_code)]
    pub title: String,
    /// Public API: Lesson metadata for future UI tooltips and analytics export
    #[allow(dead_code)]
    pub description: String,
    /// Public API: Lesson metadata for future UI tooltips and analytics export
    #[allow(dead_code)]
    pub keys: Vec<char>,
}

impl Lesson {
    pub fn new(
        lesson_type: LessonType,
        title: String,
        description: String,
        keys: Vec<char>,
    ) -> Self {
        Self {
            lesson_type,
            title,
            description,
            keys,
        }
    }

    /// Create code symbol lessons for a programming language
    pub fn code_symbol_lessons(language: ProgrammingLanguage) -> Vec<Lesson> {
        let lang_name = match language {
            ProgrammingLanguage::TypeScript => "TypeScript",
            ProgrammingLanguage::Rust => "Rust",
            ProgrammingLanguage::Python => "Python",
        };

        vec![
            Lesson::new(
                LessonType::CodeSymbols { language, level: 1 },
                format!("{} - Level 1", lang_name),
                "Basic brackets: () [] {}".to_string(),
                vec![],
            ),
            Lesson::new(
                LessonType::CodeSymbols { language, level: 2 },
                format!("{} - Level 2", lang_name),
                "Basic operators: + - * / =".to_string(),
                vec![],
            ),
            Lesson::new(
                LessonType::CodeSymbols { language, level: 3 },
                format!("{} - Level 3", lang_name),
                "Comparisons: == != < >".to_string(),
                vec![],
            ),
            Lesson::new(
                LessonType::CodeSymbols { language, level: 4 },
                format!("{} - Level 4", lang_name),
                "Arrows & special: -> => ::".to_string(),
                vec![],
            ),
            Lesson::new(
                LessonType::CodeSymbols { language, level: 5 },
                format!("{} - Level 5", lang_name),
                "Compound operators: && ||".to_string(),
                vec![],
            ),
            Lesson::new(
                LessonType::CodeSymbols { language, level: 6 },
                format!("{} - Level 6", lang_name),
                "Realistic code snippets".to_string(),
                vec![],
            ),
        ]
    }

    /// Create bigram lessons for a specific language or code
    pub fn bigram_lessons(bigram_type: BigramType, language: Option<Language>) -> Vec<Lesson> {
        let lang_name = match bigram_type {
            BigramType::Natural => match language {
                Some(Language::French) => "French",
                Some(Language::English) => "English",
                None => "Natural",
            },
            BigramType::Code => "Code",
        };

        vec![
            Lesson::new(
                LessonType::Bigram {
                    bigram_type,
                    language,
                    level: 1,
                },
                format!("{} Bigrams - Level 1", lang_name),
                "Drill mode: Pure repetition".to_string(),
                vec![],
            ),
            Lesson::new(
                LessonType::Bigram {
                    bigram_type,
                    language,
                    level: 2,
                },
                format!("{} Bigrams - Level 2", lang_name),
                "Word mode: Contextual practice".to_string(),
                vec![],
            ),
            Lesson::new(
                LessonType::Bigram {
                    bigram_type,
                    language,
                    level: 3,
                },
                format!("{} Bigrams - Level 3", lang_name),
                "Mixed mode: Realistic text".to_string(),
                vec![],
            ),
            Lesson::new(
                LessonType::Bigram {
                    bigram_type,
                    language,
                    level: 4,
                },
                format!("{} Bigrams - Level 4", lang_name),
                match (bigram_type, language) {
                    (BigramType::Natural, Some(Language::French)) => {
                        "Mixed mode: All 40 bigrams (with accents)".to_string()
                    }
                    (BigramType::Natural, Some(Language::English)) => {
                        "Mixed mode: All 30 bigrams".to_string()
                    }
                    (BigramType::Code, _) => "Mixed mode: All 10 code bigrams".to_string(),
                    _ => "Mixed mode: All bigrams".to_string(),
                },
                vec![],
            ),
        ]
    }

    /// Create trigram lessons for a language
    pub fn trigram_lessons(language: Language) -> Vec<Lesson> {
        let lang_name = match language {
            Language::French => "French",
            Language::English => "English",
        };

        vec![
            Lesson::new(
                LessonType::Trigram { language, level: 1 },
                format!("{} Trigrams - Level 1", lang_name),
                "Drill mode: Pure repetition (5 trigrams)".to_string(),
                vec![],
            ),
            Lesson::new(
                LessonType::Trigram { language, level: 2 },
                format!("{} Trigrams - Level 2", lang_name),
                "Word mode: Contextual practice (10 trigrams)".to_string(),
                vec![],
            ),
            Lesson::new(
                LessonType::Trigram { language, level: 3 },
                format!("{} Trigrams - Level 3", lang_name),
                "Mixed mode: Realistic text (20 trigrams)".to_string(), // Updated from 15 to 20
                vec![],
            ),
            Lesson::new(
                LessonType::Trigram { language, level: 4 },
                format!("{} Trigrams - Level 4", lang_name),
                match language {
                    Language::French => "Mixed mode: All 25 trigrams (with accents)".to_string(),
                    Language::English => "Mixed mode: All 20 trigrams".to_string(),
                },
                vec![],
            ),
        ]
    }

    /// Create common word lessons for a language
    pub fn common_word_lessons(language: Language) -> Vec<Lesson> {
        let lang_name = match language {
            Language::French => "French",
            Language::English => "English",
        };

        vec![
            Lesson::new(
                LessonType::CommonWords { language, level: 1 },
                format!("{} Words - Level 1", lang_name),
                "Top 50 most common words".to_string(),
                vec![],
            ),
            Lesson::new(
                LessonType::CommonWords { language, level: 2 },
                format!("{} Words - Level 2", lang_name),
                "Top 100 most common words".to_string(),
                vec![],
            ),
            Lesson::new(
                LessonType::CommonWords { language, level: 3 },
                format!("{} Words - Level 3", lang_name),
                "Top 200 most common words".to_string(),
                vec![],
            ),
            Lesson::new(
                LessonType::CommonWords { language, level: 4 },
                format!("{} Words - Level 4", lang_name),
                "All 500 most common words".to_string(),
                vec![],
            ),
        ]
    }

    /// Create adaptive lesson (personalized training)
    pub fn adaptive_lesson() -> Self {
        Lesson::new(
            LessonType::Adaptive,
            "Adaptive Mode".to_string(),
            "Personalized practice based on your weak areas".to_string(),
            vec![],
        )
    }

    /// Create all 24 finger-based lessons (4 finger pairs × 6 lessons each)
    pub fn finger_pair_lessons() -> Vec<Lesson> {
        use FingerPairType::*;
        let mut lessons = Vec::new();

        for &finger_pair in &[Pinky, Ring, Middle, Index] {
            for level in 1..=3 {
                // Base version (no shift)
                lessons.push(Lesson::new(
                    LessonType::FingerPair {
                        finger_pair,
                        level,
                        with_shift: false,
                    },
                    Self::finger_pair_title(finger_pair, level, false),
                    Self::finger_pair_description(finger_pair, level, false),
                    vec![], // Keys determined dynamically during generation
                ));

                // Shift version (mixed case)
                lessons.push(Lesson::new(
                    LessonType::FingerPair {
                        finger_pair,
                        level,
                        with_shift: true,
                    },
                    Self::finger_pair_title(finger_pair, level, true),
                    Self::finger_pair_description(finger_pair, level, true),
                    vec![], // Keys determined dynamically during generation
                ));
            }
        }

        lessons
    }

    fn finger_pair_title(pair: FingerPairType, level: u8, with_shift: bool) -> String {
        let pair_name = match pair {
            FingerPairType::Pinky => "Pinky Fingers",
            FingerPairType::Ring => "Ring Fingers",
            FingerPairType::Middle => "Middle Fingers",
            FingerPairType::Index => "Index Fingers",
        };
        let level_name = match level {
            1 => "Home Row",
            2 => "Extended",
            3 => "All Keys",
            _ => "Unknown",
        };
        if with_shift {
            format!("{} - {} + Shift", pair_name, level_name)
        } else {
            format!("{} - {}", pair_name, level_name)
        }
    }

    fn finger_pair_description(_pair: FingerPairType, level: u8, with_shift: bool) -> String {
        let level_desc = match level {
            1 => "Home row keys only",
            2 => "Home + top/bottom rows",
            3 => "All keys including numbers and symbols",
            _ => "Unknown level",
        };
        if with_shift {
            format!("{} with mixed case", level_desc)
        } else {
            level_desc.to_string()
        }
    }

    /// Create all 8 row progression lessons (4 levels × 2 variants)
    pub fn row_progression_lessons() -> Vec<Lesson> {
        use RowLevel::*;
        let mut lessons = Vec::new();

        for &level in &[Level1, Level2, Level3, Level4] {
            // Base version (no shift)
            lessons.push(Lesson::new(
                LessonType::RowProgression {
                    level,
                    with_shift: false,
                },
                Self::row_level_title(level, false),
                Self::row_level_description(level, false),
                vec![], // Keys determined dynamically during generation
            ));

            // Shift version (mixed case + symbols)
            lessons.push(Lesson::new(
                LessonType::RowProgression {
                    level,
                    with_shift: true,
                },
                Self::row_level_title(level, true),
                Self::row_level_description(level, true),
                vec![], // Keys determined dynamically during generation
            ));
        }

        lessons
    }

    fn row_level_title(level: RowLevel, with_shift: bool) -> String {
        let level_name = match level {
            RowLevel::Level1 => "Row Level 1 - Home Row",
            RowLevel::Level2 => "Row Level 2 - Home + Top",
            RowLevel::Level3 => "Row Level 3 - Home + Top + Bottom",
            RowLevel::Level4 => "Row Level 4 - Full Keyboard",
        };
        if with_shift {
            format!("{} (+ Shift)", level_name)
        } else {
            level_name.to_string()
        }
    }

    fn row_level_description(level: RowLevel, with_shift: bool) -> String {
        let level_desc = match level {
            RowLevel::Level1 => "Home row keys: q s d f g h j k l m",
            RowLevel::Level2 => "Home + top rows",
            RowLevel::Level3 => "Home + top + bottom rows",
            RowLevel::Level4 => "All rows including numbers",
        };
        if with_shift {
            format!("{} with mixed case and symbols", level_desc)
        } else {
            level_desc.to_string()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_shifted_char_mapping() {
        assert_eq!(get_shifted_char('a'), Some('A'));
        assert_eq!(get_shifted_char('z'), Some('Z'));
        assert_eq!(get_shifted_char('&'), Some('1'));
        assert_eq!(get_shifted_char(';'), Some('.'));
        assert_eq!(get_shifted_char('x'), Some('X')); // lowercase letters map to uppercase
        assert_eq!(get_shifted_char('@'), None); // symbols without mapping return None
    }

    #[test]
    fn test_row_progression_lessons_count() {
        let lessons = Lesson::row_progression_lessons();
        assert_eq!(lessons.len(), 8); // 4 levels × 2 variants
    }

    #[test]
    fn test_row_progression_lesson_types() {
        let lessons = Lesson::row_progression_lessons();

        // First lesson should be Level1 without shift
        assert_eq!(
            lessons[0].lesson_type,
            LessonType::RowProgression {
                level: RowLevel::Level1,
                with_shift: false
            }
        );

        // Second lesson should be Level1 with shift
        assert_eq!(
            lessons[1].lesson_type,
            LessonType::RowProgression {
                level: RowLevel::Level1,
                with_shift: true
            }
        );

        // Last lesson should be Level4 with shift
        assert_eq!(
            lessons[7].lesson_type,
            LessonType::RowProgression {
                level: RowLevel::Level4,
                with_shift: true
            }
        );
    }

    #[test]
    fn test_row_progression_titles() {
        let lessons = Lesson::row_progression_lessons();

        assert_eq!(lessons[0].title, "Row Level 1 - Home Row");
        assert_eq!(lessons[1].title, "Row Level 1 - Home Row (+ Shift)");
        assert_eq!(lessons[6].title, "Row Level 4 - Full Keyboard");
        assert_eq!(lessons[7].title, "Row Level 4 - Full Keyboard (+ Shift)");
    }
}
