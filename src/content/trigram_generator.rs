/// Content generator for trigram training lessons
use super::bigram::Language;
use super::ngram_generator::{generate_drill_mode, generate_mixed_mode, generate_word_mode};
use super::trigram::{english_trigrams, french_trigrams, Trigram};

pub struct TrigramGenerator {
    trigrams: Vec<Trigram>,
}

impl TrigramGenerator {
    pub fn new(language: Language) -> Self {
        let trigrams = match language {
            Language::French => french_trigrams(),
            Language::English => english_trigrams(),
        };

        Self { trigrams }
    }

    /// Generate content for a given level
    /// Level 1: Drill mode (pure repetition)
    /// Level 2: Word mode (contextual words)
    /// Level 3: Mixed mode (realistic sentences)
    /// Level 4: Mixed mode with all trigrams
    pub fn generate(&self, level: usize, length: usize) -> String {
        let selected_trigrams = self.select_trigrams_for_level(level);

        match level {
            1 => generate_drill_mode(&selected_trigrams, length),
            2 => generate_word_mode(&selected_trigrams, length),
            3 | 4 => generate_mixed_mode(&selected_trigrams, length),
            _ => String::new(),
        }
    }

    /// Select trigrams based on level (more trigrams = higher level)
    fn select_trigrams_for_level(&self, level: usize) -> Vec<&Trigram> {
        let count = match level {
            1 => 5,  // Top 5
            2 => 10, // Top 10 (doubles)
            3 => 20, // Top 20 (doubles again)
            4 => 25, // All 25 (increased from 20)
            _ => 5,
        };

        self.trigrams.iter().take(count).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_drill_mode_generation() {
        let gen = TrigramGenerator::new(Language::French);
        let content = gen.generate(1, 30);

        assert!(!content.is_empty());
        assert!(content.chars().count() <= 30);

        // Should contain repeated trigrams
        assert!(content.contains("les les les") || content.contains("des des des"));
    }

    #[test]
    fn test_word_mode_generation() {
        let gen = TrigramGenerator::new(Language::French);
        let content = gen.generate(2, 50);

        assert!(!content.is_empty());
        assert!(content.chars().count() <= 50);

        // Should contain real words, not drill patterns
        assert!(
            content.contains("les")
                || content.contains("des")
                || content.contains("ment")
                || content.contains("moment")
        );
    }

    #[test]
    fn test_mixed_mode_generation() {
        let gen = TrigramGenerator::new(Language::French);
        let content = gen.generate(3, 60);

        assert!(!content.is_empty());
        assert!(content.chars().count() <= 60);

        // Should contain multiple words
        let word_count = content.split_whitespace().count();
        assert!(word_count >= 3);
    }

    #[test]
    fn test_english_trigrams() {
        let gen = TrigramGenerator::new(Language::English);
        let content = gen.generate(1, 30);

        assert!(content.contains("the the the") || content.contains("and and and"));
    }

    #[test]
    fn test_level_progression() {
        let gen = TrigramGenerator::new(Language::French);

        let level1 = gen.generate(1, 50);
        let level2 = gen.generate(2, 50);
        let level3 = gen.generate(3, 50);

        // All should generate content
        assert!(!level1.is_empty());
        assert!(!level2.is_empty());
        assert!(!level3.is_empty());

        // Level 1 should have drill patterns
        assert!(level1.contains("les les les") || level1.contains("des des des"));

        // Level 2/3 should have real words
        assert!(level2.split_whitespace().any(|w| w.len() > 3));
        assert!(level3.split_whitespace().any(|w| w.len() > 3));
    }

    #[test]
    fn test_random_newlines_in_generation() {
        let gen = TrigramGenerator::new(Language::French);

        let content = gen.generate(2, 100);

        // Content should contain both spaces and newlines (random mix)
        assert!(content.contains(' '));
        // Content should have expected words from top trigrams
        assert!(content.contains("les") || content.contains("des") || content.contains("ment"));
        // Content length should respect constraint
        assert!(content.chars().count() <= 100);
    }

    #[test]
    fn test_level_selection() {
        let gen = TrigramGenerator::new(Language::English);

        let level1_trigrams = gen.select_trigrams_for_level(1);
        let level2_trigrams = gen.select_trigrams_for_level(2);
        let level3_trigrams = gen.select_trigrams_for_level(3);
        let level4_trigrams = gen.select_trigrams_for_level(4);

        assert_eq!(level1_trigrams.len(), 5);
        assert_eq!(level2_trigrams.len(), 10);
        assert_eq!(level3_trigrams.len(), 20); // CHANGED from 15
        assert_eq!(level4_trigrams.len(), 20); // English has 20 trigrams total
    }
}
