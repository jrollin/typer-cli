use super::bigram::{BigramType, Language};

/// Types de leçons disponibles
#[derive(Debug, Clone, PartialEq)]
pub enum LessonType {
    HomeRow {
        level: usize,
    },
    Bigram {
        bigram_type: BigramType,
        language: Option<Language>,
        level: usize,
    },
}

/// Représente une leçon de typing
#[derive(Debug, Clone)]
pub struct Lesson {
    pub lesson_type: LessonType,
    #[allow(dead_code)]
    pub title: String,
    #[allow(dead_code)]
    pub description: String,
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

    /// Créer les leçons home row pour AZERTY
    /// Progressive cumulative approach: each level builds on previous ones
    pub fn home_row_lessons() -> Vec<Lesson> {
        vec![
            // Level 1: Index fingers only
            Lesson::new(
                LessonType::HomeRow { level: 1 },
                "Home Row - Level 1".to_string(),
                "Index fingers: f and j".to_string(),
                vec!['f', 'j'],
            ),
            // Level 2: Add middle fingers (cumulative)
            Lesson::new(
                LessonType::HomeRow { level: 2 },
                "Home Row - Level 2".to_string(),
                "Add middle fingers: f, j, d, k".to_string(),
                vec!['f', 'j', 'd', 'k'],
            ),
            // Level 3: Add ring fingers (cumulative)
            Lesson::new(
                LessonType::HomeRow { level: 3 },
                "Home Row - Level 3".to_string(),
                "Add ring fingers: f, j, d, k, s, l".to_string(),
                vec!['f', 'j', 'd', 'k', 's', 'l'],
            ),
            // Level 4: Add pinkies (cumulative)
            Lesson::new(
                LessonType::HomeRow { level: 4 },
                "Home Row - Level 4".to_string(),
                "Add pinkies: f, j, d, k, s, l, q, m".to_string(),
                vec!['f', 'j', 'd', 'k', 's', 'l', 'q', 'm'],
            ),
            // Level 5: All home row keys
            Lesson::new(
                LessonType::HomeRow { level: 5 },
                "Home Row - Level 5".to_string(),
                "All home row keys".to_string(),
                vec!['q', 's', 'd', 'f', 'g', 'h', 'j', 'k', 'l', 'm'],
            ),
            // Level 6: French words using home row
            Lesson::new(
                LessonType::HomeRow { level: 6 },
                "Home Row - Level 6".to_string(),
                "French words using home row".to_string(),
                vec!['q', 's', 'd', 'f', 'g', 'h', 'j', 'k', 'l', 'm'],
            ),
        ]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_home_row_lessons_count() {
        let lessons = Lesson::home_row_lessons();
        assert_eq!(lessons.len(), 6);
    }

    #[test]
    fn test_home_row_lesson_level_1() {
        let lessons = Lesson::home_row_lessons();
        let lesson = &lessons[0];
        assert_eq!(lesson.lesson_type, LessonType::HomeRow { level: 1 });
        assert_eq!(lesson.keys, vec!['f', 'j']);
    }

    #[test]
    fn test_home_row_lesson_all_keys() {
        let lessons = Lesson::home_row_lessons();
        let lesson = &lessons[4];
        assert_eq!(lesson.lesson_type, LessonType::HomeRow { level: 5 });
        assert_eq!(lesson.keys.len(), 10);
    }
}
