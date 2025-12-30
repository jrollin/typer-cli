/// Content generator for bigram training lessons
use super::bigram::{code_bigrams, english_bigrams, french_bigrams, Bigram, BigramType, Language};

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
            1 => self.generate_drill_mode(&selected_bigrams, length),
            2 => self.generate_word_mode(&selected_bigrams, length),
            3 | 4 => self.generate_mixed_mode(&selected_bigrams, length),
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

    /// Level 1: Pure bigram repetition
    /// Example: "qu qu qu ou ou ou en en en"
    fn generate_drill_mode(&self, bigrams: &[&Bigram], length: usize) -> String {
        let mut result = String::new();
        let mut idx = 0;

        while result.len() < length {
            if !result.is_empty() {
                result.push(' ');
            }

            let bigram = bigrams[idx % bigrams.len()];
            // Repeat the bigram 3 times
            result.push_str(&format!(
                "{} {} {}",
                bigram.pattern, bigram.pattern, bigram.pattern
            ));

            idx += 1;
        }

        result.chars().take(length).collect()
    }

    /// Level 2: Bigrams in word context
    /// Example: "que qui quoi pour vous nous"
    fn generate_word_mode(&self, bigrams: &[&Bigram], length: usize) -> String {
        let mut result = String::new();
        let mut bigram_idx = 0;

        while result.len() < length {
            if !result.is_empty() {
                result.push(' ');
            }

            let bigram = bigrams[bigram_idx % bigrams.len()];

            // Cycle through examples for this bigram
            let example_idx = (bigram_idx / bigrams.len()) % bigram.examples.len();
            let word = &bigram.examples[example_idx];

            result.push_str(word);
            bigram_idx += 1;
        }

        result.chars().take(length).collect()
    }

    /// Level 3: Realistic sentences with target bigrams
    /// Combines examples into natural-looking phrases
    fn generate_mixed_mode(&self, bigrams: &[&Bigram], length: usize) -> String {
        let mut result = String::new();
        let mut word_count = 0;

        while result.len() < length {
            if word_count > 0 {
                result.push(' ');
            }

            // Pick a bigram
            let bigram = bigrams[word_count % bigrams.len()];

            // Pick an example
            let example_idx = (word_count / bigrams.len()) % bigram.examples.len();
            let word = &bigram.examples[example_idx];

            result.push_str(word);
            word_count += 1;
        }

        result.chars().take(length).collect()
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
        assert!(content.len() <= 30);

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
    fn test_deterministic_generation() {
        let gen = BigramGenerator::new(BigramType::Natural, Some(Language::French));

        let content1 = gen.generate(2, 40);
        let content2 = gen.generate(2, 40);

        // Same level and length should produce same content
        assert_eq!(content1, content2);
    }
}
