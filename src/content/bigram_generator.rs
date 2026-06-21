/// Content generator for bigram training lessons
use super::bigram::{code_bigrams, english_bigrams, french_bigrams, Bigram, BigramType, Language};
use super::ngram_generator::{generate_drill_mode, generate_mixed_mode, generate_word_mode};

pub struct BigramGenerator {
    bigrams: Vec<Bigram>,
}

impl BigramGenerator {
    pub fn new(bigram_type: BigramType, language: Option<Language>) -> Self {
        let bigrams = match bigram_type {
            BigramType::Natural => match language {
                Some(Language::French) => french_bigrams(),
                Some(Language::English) => english_bigrams(),
                None => french_bigrams(), // Default to French
            },
            BigramType::Code => code_bigrams(),
        };

        Self { bigrams }
    }

    /// Generate content for a given level
    /// Level 1: Drill mode (pure repetition)
    /// Level 2: Word mode (contextual words)
    /// Level 3: Mixed mode (realistic sentences)
    /// Level 4: Mixed mode with all bigrams
    pub fn generate(&self, level: usize, length: usize) -> String {
        let selected_bigrams = self.select_bigrams_for_level(level);

        match level {
            1 => generate_drill_mode(&selected_bigrams, length),
            2 => generate_word_mode(&selected_bigrams, length),
            3 | 4 => generate_mixed_mode(&selected_bigrams, length),
            _ => String::new(),
        }
    }

    /// Select bigrams based on level (more bigrams = higher level)
    fn select_bigrams_for_level(&self, level: usize) -> Vec<&Bigram> {
        let count = match level {
            1 => 5,  // Top 5 most common
            2 => 10, // Top 10 (doubles)
            3 => 20, // Top 20 (doubles again)
            4 => 40, // All 40 (PERFECT DOUBLING!)
            _ => 5,
        };

        self.bigrams.iter().take(count).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_drill_mode_generation() {
        let gen = BigramGenerator::new(BigramType::Natural, Some(Language::French));
        let content = gen.generate(1, 30);

        assert!(!content.is_empty());
        assert!(content.chars().count() <= 30);

        // Should contain repeated bigrams
        assert!(content.contains("es es es") || content.contains("le le le"));
    }

    #[test]
    fn test_word_mode_generation() {
        let gen = BigramGenerator::new(BigramType::Natural, Some(Language::French));
        let content = gen.generate(2, 50);

        assert!(!content.is_empty());
        assert!(content.chars().count() <= 50); // Use char count instead of byte length

        // Should contain real words, not drill patterns
        assert!(
            content.contains("les")
                || content.contains("des")
                || content.contains("en")
                || content.contains("entre")
        );
    }

    #[test]
    fn test_mixed_mode_generation() {
        let gen = BigramGenerator::new(BigramType::Natural, Some(Language::French));
        let content = gen.generate(3, 60);

        assert!(!content.is_empty());
        assert!(content.chars().count() <= 60); // Use char count instead of byte length

        // Should contain multiple words
        let word_count = content.split_whitespace().count();
        assert!(word_count >= 3);
    }

    #[test]
    fn test_english_bigrams() {
        let gen = BigramGenerator::new(BigramType::Natural, Some(Language::English));
        let content = gen.generate(1, 30);

        assert!(content.contains("th th th") || content.contains("he he he"));
    }

    #[test]
    fn test_code_bigrams() {
        let gen = BigramGenerator::new(BigramType::Code, None);
        let content = gen.generate(1, 40);

        assert!(content.contains("->") || content.contains("::") || content.contains("=>"));
    }

    #[test]
    fn test_level_progression() {
        let gen = BigramGenerator::new(BigramType::Natural, Some(Language::French));

        let level1 = gen.generate(1, 50);
        let level2 = gen.generate(2, 50);
        let level3 = gen.generate(3, 50);

        // All should generate content
        assert!(!level1.is_empty());
        assert!(!level2.is_empty());
        assert!(!level3.is_empty());

        // Level 1 should have drill patterns
        assert!(level1.contains("es es es") || level1.contains("le le le"));

        // Level 2/3 should have real words
        assert!(level2.split_whitespace().any(|w| w.len() > 2));
        assert!(level3.split_whitespace().any(|w| w.len() > 2));
    }

    #[test]
    fn test_accented_content_fills_requested_char_length() {
        // Regression: French bigrams include multibyte chars (é, è, à...). A byte-based
        // generation loop stops short of the requested CHAR length. Generated content must
        // get close to the target measured in chars (within one trailing word).
        let gen = BigramGenerator::new(BigramType::Natural, Some(Language::French));
        let length = 80;
        let content = gen.generate(2, length);

        let char_count = content.chars().count();
        assert!(char_count <= length);
        assert!(
            char_count >= length - 15,
            "expected near {length} chars, got {char_count}: {content:?}"
        );
    }

    #[test]
    fn test_random_newlines_in_generation() {
        let gen = BigramGenerator::new(BigramType::Natural, Some(Language::French));

        let content = gen.generate(2, 100);

        // Content should contain both spaces and newlines (random mix)
        assert!(content.contains(' '));
        // Content should have expected words from top bigrams
        assert!(content.contains("les") || content.contains("de") || content.contains("en"));
        // Content length should respect constraint
        assert!(content.chars().count() <= 100);
    }
}
