use super::bigram::{BigramType, Language};
use super::code_symbols::ProgrammingLanguage;

/// Definition of a single key pair lesson
#[derive(Debug, Clone)]
pub struct KeyPairLessonDef {
    pub id: u8,
    pub title: &'static str,
    pub description: &'static str,
    pub keys: &'static [char],
}

/// All 17 individual key pair lessons
pub const KEY_PAIR_LESSONS: [KeyPairLessonDef; 17] = [
    // Lessons 1-4: Basic home row pairs
    KeyPairLessonDef {
        id: 1,
        title: "f-d  j-k",
        description: "Index/middle fingers",
        keys: &['f', 'd', 'j', 'k'],
    },
    KeyPairLessonDef {
        id: 2,
        title: "f-g  j-h",
        description: "Index reaches",
        keys: &['f', 'g', 'j', 'h'],
    },
    KeyPairLessonDef {
        id: 3,
        title: "s-a  l-;",
        description: "Ring finger pairs",
        keys: &['s', 'a', 'l', ';'],
    },
    KeyPairLessonDef {
        id: 4,
        title: "d-e  k-i",
        description: "Middle reaches up",
        keys: &['d', 'e', 'k', 'i'],
    },
    // Lessons 5-8: Extended reaches
    KeyPairLessonDef {
        id: 5,
        title: "f-r  j-u",
        description: "Index diagonal",
        keys: &['f', 'r', 'j', 'u'],
    },
    KeyPairLessonDef {
        id: 6,
        title: "f-t  j-y",
        description: "Index extended",
        keys: &['f', 't', 'j', 'y'],
    },
    KeyPairLessonDef {
        id: 7,
        title: "s-w  l-o",
        description: "Ring top/bottom",
        keys: &['s', 'w', 'l', 'o'],
    },
    KeyPairLessonDef {
        id: 8,
        title: "a-q  ;-p",
        description: "Pinky reaches",
        keys: &['a', 'q', ';', 'p'],
    },
    // Lessons 9-12: Bottom row
    KeyPairLessonDef {
        id: 9,
        title: "f-v  j-m",
        description: "Index to bottom",
        keys: &['f', 'v', 'j', 'm'],
    },
    KeyPairLessonDef {
        id: 10,
        title: "f-b  j-n",
        description: "Index bottom extended",
        keys: &['f', 'b', 'j', 'n'],
    },
    KeyPairLessonDef {
        id: 11,
        title: "d-c  k-,",
        description: "Middle to bottom",
        keys: &['d', 'c', 'k', ','],
    },
    KeyPairLessonDef {
        id: 12,
        title: "a-z  s-x",
        description: "Bottom row practice",
        keys: &['a', 'z', 's', 'x'],
    },
    // Lessons 13-17: Numbers and symbols
    KeyPairLessonDef {
        id: 13,
        title: "1 2 3 4 5 6  7 8 9 0",
        description: "Number row",
        keys: &['1', '2', '3', '4', '5', '6', '7', '8', '9', '0'],
    },
    KeyPairLessonDef {
        id: 14,
        title: "/ ? : . '  - _",
        description: "Punctuation",
        keys: &['/', '?', ':', '.', '\'', ' ', '-', '_'],
    },
    KeyPairLessonDef {
        id: 15,
        title: "[ ] ( ) { }  < > | \\",
        description: "Brackets",
        keys: &['[', ']', '(', ')', '{', '}', '<', '>', '|', '\\'],
    },
    KeyPairLessonDef {
        id: 16,
        title: "' - _ ! _ + =  & * ( )",
        description: "Symbols",
        keys: &['\'', '-', '_', '!', '_', '+', '=', '&', '*', '(', ')'],
    },
    KeyPairLessonDef {
        id: 17,
        title: "@ # $ % ^ &  & * ( )",
        description: "Special chars",
        keys: &['@', '#', '$', '%', '^', '&', '&', '*', '(', ')'],
    },
];

/// Definition of a lesson group
#[derive(Debug, Clone)]
pub struct KeyPairGroupDef {
    pub group_id: u8,
    pub title: &'static str,
    pub lesson_range: (u8, u8),
}

/// All 4 lesson groups
pub const KEY_PAIR_GROUPS: [KeyPairGroupDef; 4] = [
    KeyPairGroupDef {
        group_id: 1,
        title: "Lessons 1-4",
        lesson_range: (1, 4),
    },
    KeyPairGroupDef {
        group_id: 2,
        title: "Lessons 5-8",
        lesson_range: (5, 8),
    },
    KeyPairGroupDef {
        group_id: 3,
        title: "Lessons 9-12",
        lesson_range: (9, 12),
    },
    KeyPairGroupDef {
        group_id: 4,
        title: "Lessons 13-17",
        lesson_range: (13, 17),
    },
];

/// Map between normal and shifted characters for AZERTY
#[derive(Debug, Clone)]
pub struct ShiftedCharMap {
    pub normal: char,
    pub shifted: char,
}

/// AZERTY keyboard shift mappings
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
pub fn get_shifted_char(c: char) -> Option<char> {
    AZERTY_SHIFT_MAP
        .iter()
        .find(|map| map.normal == c)
        .map(|map| map.shifted)
}

/// Types de leçons disponibles
#[derive(Debug, Clone, PartialEq)]
pub enum LessonType {
    /// Individual key pair lessons (17 lessons)
    KeyPair {
        lesson_id: u8,
    },
    /// Lesson groups (4 groups, with/without shift = 8 total)
    KeyPairGroup {
        group_id: u8,
        with_shift: bool,
    },
    Bigram {
        bigram_type: BigramType,
        language: Option<Language>,
        level: usize,
    },
    CodeSymbols {
        language: ProgrammingLanguage,
        level: usize,
    },
    Adaptive,
}

/// Représente une leçon de typing
#[derive(Debug, Clone)]
pub struct Lesson {
    pub lesson_type: LessonType,
    #[allow(dead_code)]
    pub title: String,
    #[allow(dead_code)]
    pub description: String,
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

    /// Create all individual key pair lessons
    pub fn key_pair_lessons() -> Vec<Lesson> {
        KEY_PAIR_LESSONS
            .iter()
            .map(|def| {
                Lesson::new(
                    LessonType::KeyPair { lesson_id: def.id },
                    def.title.to_string(),
                    def.description.to_string(),
                    def.keys.to_vec(),
                )
            })
            .collect()
    }

    /// Create lesson group lessons (with or without shift variants)
    pub fn key_pair_group_lessons(with_shift: bool) -> Vec<Lesson> {
        KEY_PAIR_GROUPS
            .iter()
            .map(|group_def| {
                let title = if with_shift {
                    format!("{} + Shift", group_def.title)
                } else {
                    group_def.title.to_string()
                };

                let description = if with_shift {
                    "Mixed case and symbols".to_string()
                } else {
                    "Combined practice".to_string()
                };

                Lesson::new(
                    LessonType::KeyPairGroup {
                        group_id: group_def.group_id,
                        with_shift,
                    },
                    title,
                    description,
                    vec![], // Keys will be collected from individual lessons during generation
                )
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_key_pair_lessons_count() {
        let lessons = Lesson::key_pair_lessons();
        assert_eq!(lessons.len(), 17);
    }

    #[test]
    fn test_key_pair_lesson_first() {
        let lessons = Lesson::key_pair_lessons();
        let lesson = &lessons[0];
        assert_eq!(lesson.lesson_type, LessonType::KeyPair { lesson_id: 1 });
        assert_eq!(lesson.keys, vec!['f', 'd', 'j', 'k']);
    }

    #[test]
    fn test_key_pair_groups_count() {
        let lessons_without_shift = Lesson::key_pair_group_lessons(false);
        let lessons_with_shift = Lesson::key_pair_group_lessons(true);
        assert_eq!(lessons_without_shift.len(), 4);
        assert_eq!(lessons_with_shift.len(), 4);
    }

    #[test]
    fn test_shifted_char_mapping() {
        assert_eq!(get_shifted_char('a'), Some('A'));
        assert_eq!(get_shifted_char('z'), Some('Z'));
        assert_eq!(get_shifted_char('&'), Some('1'));
        assert_eq!(get_shifted_char(';'), Some('.'));
        assert_eq!(get_shifted_char('x'), Some('X')); // lowercase letters map to uppercase
        assert_eq!(get_shifted_char('@'), None); // symbols without mapping return None
    }
}
