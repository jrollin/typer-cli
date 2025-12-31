use ratatui::style::Color;

use super::bigram::BigramType;
use super::lesson::{Lesson, LessonType};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum LessonCategoryType {
    Adaptive,
    FingerTraining,
    KeyTraining,
    Languages,
    Code,
    Custom,
}

#[derive(Debug, Clone)]
pub struct LessonCategory {
    pub category_type: LessonCategoryType,
    pub name: &'static str,
    pub description: &'static str,
    pub color: Color,
}

impl LessonCategory {
    pub fn all(has_adaptive: bool) -> Vec<Self> {
        let mut categories = Vec::new();

        if has_adaptive {
            categories.push(Self {
                category_type: LessonCategoryType::Adaptive,
                name: "Adaptive",
                description: "Personalized training based on your weak areas",
                color: Color::Cyan,
            });
        }

        categories.extend([
            Self {
                category_type: LessonCategoryType::FingerTraining,
                name: "Finger Training",
                description: "Bilateral finger-based drills (24 lessons)",
                color: Color::Green,
            },
            Self {
                category_type: LessonCategoryType::KeyTraining,
                name: "Key Training",
                description: "Progressive key pair exercises (25 lessons)",
                color: Color::Cyan,
            },
            Self {
                category_type: LessonCategoryType::Languages,
                name: "Languages",
                description: "French & English bigrams, trigrams, and words",
                color: Color::Yellow,
            },
            Self {
                category_type: LessonCategoryType::Code,
                name: "Code",
                description: "Programming symbols for TypeScript, Rust, Python",
                color: Color::Magenta,
            },
            Self {
                category_type: LessonCategoryType::Custom,
                name: "Custom",
                description: "User-provided markdown lessons",
                color: Color::Blue,
            },
        ]);

        categories
    }

    /// Check if a lesson belongs to this category
    pub fn contains_lesson(&self, lesson: &Lesson) -> bool {
        match self.category_type {
            LessonCategoryType::Adaptive => {
                matches!(lesson.lesson_type, LessonType::Adaptive)
            }
            LessonCategoryType::FingerTraining => {
                matches!(lesson.lesson_type, LessonType::FingerPair { .. })
            }
            LessonCategoryType::KeyTraining => matches!(
                lesson.lesson_type,
                LessonType::KeyPair { .. } | LessonType::KeyPairGroup { .. }
            ),
            LessonCategoryType::Languages => matches!(
                lesson.lesson_type,
                LessonType::Bigram {
                    bigram_type: BigramType::Natural,
                    ..
                } | LessonType::Trigram { .. }
                    | LessonType::CommonWords { .. }
            ),
            LessonCategoryType::Code => matches!(
                lesson.lesson_type,
                LessonType::CodeSymbols { .. }
                    | LessonType::Bigram {
                        bigram_type: BigramType::Code,
                        ..
                    }
            ),
            LessonCategoryType::Custom => {
                matches!(lesson.lesson_type, LessonType::Custom { .. })
            }
        }
    }
}
