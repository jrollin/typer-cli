/// Content generator for common word training lessons
use super::bigram::Language;
use super::common_word::{english_words, french_words, Word};
use rand::Rng;

pub struct CommonWordGenerator {
    words: Vec<Word>,
}

impl CommonWordGenerator {
    pub fn new(language: Language) -> Self {
        let words = match language {
            Language::French => french_words(),
            Language::English => english_words(),
        };

        Self { words }
    }

    /// Generate content for a given level
    /// Level 1: Drill mode (word repetition)
    /// Level 2-4: Sentence mode (frequency-weighted random words)
    pub fn generate(&self, level: usize, length: usize) -> String {
        let selected_words = self.select_words_for_level(level);

        match level {
            1 => self.generate_drill_mode(&selected_words, length),
            2..=4 => self.generate_sentence_mode(&selected_words, length),
            _ => String::new(),
        }
    }

    /// Select words based on level (more words = higher level)
    fn select_words_for_level(&self, level: usize) -> Vec<&Word> {
        let count = match level {
            1 => 50,  // Top 50 most common
            2 => 100, // Top 100
            3 => 200, // Top 200
            4 => 500, // All 500
            _ => 50,
        };

        self.words.iter().take(count).collect()
    }

    /// Level 1: Word repetition drill
    /// Example: "the the be be to to"
    fn generate_drill_mode(&self, words: &[&Word], length: usize) -> String {
        let mut result = String::new();
        let mut idx = 0;

        while result.chars().count() < length {
            if !result.is_empty() {
                result.push(' ');
            }

            let word = words[idx % words.len()];
            // Repeat each word 2 times
            result.push_str(&format!("{} {}", word.text, word.text));

            idx += 1;
        }

        result.chars().take(length).collect()
    }

    /// Level 2-4: Natural word sequences with frequency weighting
    /// 70% from top 20%, 30% from full pool
    fn generate_sentence_mode(&self, words: &[&Word], length: usize) -> String {
        let mut rng = rand::thread_rng();
        let mut result = String::new();

        while result.chars().count() < length {
            if !result.is_empty() {
                result.push(' ');
            }

            // Frequency-weighted selection: 70% from top 20%, 30% from full pool
            let idx = if rng.gen::<f32>() < 0.7 {
                // Select from top 20% (high-frequency words)
                rng.gen_range(0..(words.len() / 5).max(1))
            } else {
                // Select from full pool
                rng.gen_range(0..words.len())
            };

            result.push_str(&words[idx].text);
        }

        result.chars().take(length).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn test_drill_mode_has_repetition() {
        let gen = CommonWordGenerator::new(Language::French);
        let content = gen.generate(1, 50);

        assert!(!content.is_empty());
        assert!(content.chars().count() <= 50);

        // Count word occurrences
        let counts: HashMap<_, usize> =
            content
                .split_whitespace()
                .fold(HashMap::new(), |mut acc, w| {
                    *acc.entry(w).or_insert(0) += 1;
                    acc
                });

        // Drill mode should have repeated words
        assert!(
            counts.values().any(|&c| c > 1),
            "Drill mode should repeat words"
        );
    }

    #[test]
    fn test_sentence_mode_has_variety() {
        let gen = CommonWordGenerator::new(Language::English);
        let content = gen.generate(2, 100);

        assert!(!content.is_empty());
        assert!(content.chars().count() <= 100);

        // Should have multiple different words
        let unique_words: std::collections::HashSet<_> = content.split_whitespace().collect();

        assert!(unique_words.len() >= 5, "Sentence mode should have variety");
    }

    #[test]
    fn test_level_word_selection() {
        let gen = CommonWordGenerator::new(Language::English);

        let level1 = gen.select_words_for_level(1);
        let level2 = gen.select_words_for_level(2);
        let level3 = gen.select_words_for_level(3);
        let level4 = gen.select_words_for_level(4);

        assert_eq!(level1.len(), 50);
        assert_eq!(level2.len(), 100);
        assert_eq!(level3.len(), 200);
        assert_eq!(level4.len(), 500);
    }

    #[test]
    fn test_content_length_constraint() {
        let gen = CommonWordGenerator::new(Language::French);

        let content1 = gen.generate(1, 30);
        let content2 = gen.generate(2, 50);
        let content3 = gen.generate(3, 75);

        assert!(content1.chars().count() <= 30);
        assert!(content2.chars().count() <= 50);
        assert!(content3.chars().count() <= 75);
    }

    #[test]
    fn test_empty_on_invalid_level() {
        let gen = CommonWordGenerator::new(Language::English);
        let content = gen.generate(0, 50);
        assert_eq!(content, "");

        let content = gen.generate(5, 50);
        assert_eq!(content, "");
    }

    #[test]
    fn test_drill_mode_uses_top_words() {
        let gen = CommonWordGenerator::new(Language::English);
        let content = gen.generate(1, 50);

        // Should contain "the" which is #1 most common English word
        assert!(content.contains("the"), "Should use most common words");
    }

    #[test]
    fn test_sentence_mode_frequency_bias() {
        let gen = CommonWordGenerator::new(Language::French);

        // Generate larger sample to test frequency bias (using char count)
        let content = gen.generate(2, 500);
        let words: Vec<&str> = content.split_whitespace().collect();

        // Count occurrences of "le" (most common French word)
        let le_count = words.iter().filter(|&&w| w == "le").count();

        // "le" should appear more frequently than average due to frequency weighting
        // With 70/30 bias toward top 20%, "le" should appear above random chance
        // Using larger sample (500 chars) increases probability of seeing "le"
        assert!(
            le_count > 0,
            "Most common word should appear in sentence mode"
        );
    }
}
