/// Types de leçons disponibles
#[derive(Debug, Clone, PartialEq)]
pub enum LessonType {
    HomeRow { level: usize },
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

    /// Créer les leçons home row pour AZERTY
    pub fn home_row_lessons() -> Vec<Lesson> {
        vec![
            Lesson::new(
                LessonType::HomeRow { level: 1 },
                "Home Row - Index".to_string(),
                "Index fingers: f and j".to_string(),
                vec!['f', 'j'],
            ),
            Lesson::new(
                LessonType::HomeRow { level: 2 },
                "Home Row - Middle".to_string(),
                "Middle fingers: d and k".to_string(),
                vec!['d', 'k'],
            ),
            Lesson::new(
                LessonType::HomeRow { level: 3 },
                "Home Row - Ring".to_string(),
                "Ring fingers: s and l".to_string(),
                vec!['s', 'l'],
            ),
            Lesson::new(
                LessonType::HomeRow { level: 4 },
                "Home Row - Pinky".to_string(),
                "Pinky fingers: q and m".to_string(),
                vec!['q', 'm'],
            ),
            Lesson::new(
                LessonType::HomeRow { level: 5 },
                "Home Row - All Keys".to_string(),
                "All home row keys".to_string(),
                vec!['q', 's', 'd', 'f', 'g', 'h', 'j', 'k', 'l', 'm'],
            ),
            Lesson::new(
                LessonType::HomeRow { level: 6 },
                "Home Row - Words".to_string(),
                "Simple French words using home row".to_string(),
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
