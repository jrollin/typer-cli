/// Bigram training support for typing practice
/// Bigrams are common two-letter combinations that improve typing fluency
/// Language for natural bigrams
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Language {
    French,
    English,
}

/// Type of bigram practice
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum BigramType {
    Natural, // Language bigrams (qu, th, er)
    Code,    // Programming symbols (-> :: =>)
}

/// A single bigram with frequency and example words
#[derive(Debug, Clone)]
pub struct Bigram {
    pub pattern: String,
    #[allow(dead_code)]
    pub frequency: f32, // 0.0 to 1.0, higher = more common (for future use)
    pub examples: Vec<String>,
}

impl Bigram {
    pub fn new(pattern: &str, frequency: f32, examples: &[&str]) -> Self {
        Self {
            pattern: pattern.to_string(),
            frequency,
            examples: examples.iter().map(|s| s.to_string()).collect(),
        }
    }
}

/// French language bigrams (frequency-ordered)
pub fn french_bigrams() -> Vec<Bigram> {
    vec![
        Bigram::new("qu", 0.95, &["que", "qui", "quoi", "quelque"]),
        Bigram::new("ou", 0.90, &["pour", "vous", "nous", "ou"]),
        Bigram::new("en", 0.88, &["en", "ment", "bien", "rien"]),
        Bigram::new("on", 0.85, &["on", "bon", "son", "non"]),
        Bigram::new("es", 0.83, &["les", "des", "mes", "ses"]),
        Bigram::new("er", 0.80, &["premier", "dernier", "aller"]),
        Bigram::new("re", 0.78, &["re", "très", "entre", "être"]),
        Bigram::new("de", 0.75, &["de", "des", "depuis", "devant"]),
        Bigram::new("ai", 0.72, &["ai", "mais", "fait", "jamais"]),
        Bigram::new("an", 0.70, &["an", "dans", "avant", "sans"]),
    ]
}

/// English language bigrams (frequency-ordered)
pub fn english_bigrams() -> Vec<Bigram> {
    vec![
        Bigram::new("th", 0.95, &["the", "that", "with", "this"]),
        Bigram::new("he", 0.90, &["he", "the", "when", "where"]),
        Bigram::new("in", 0.88, &["in", "thing", "nothing", "into"]),
        Bigram::new("er", 0.85, &["her", "over", "after", "never"]),
        Bigram::new("an", 0.83, &["an", "and", "can", "than"]),
        Bigram::new("re", 0.80, &["are", "were", "here", "there"]),
        Bigram::new("on", 0.78, &["on", "one", "upon", "only"]),
        Bigram::new("at", 0.75, &["at", "that", "what", "late"]),
        Bigram::new("en", 0.72, &["been", "when", "then", "open"]),
        Bigram::new("ed", 0.70, &["used", "called", "asked", "moved"]),
    ]
}

/// Code/programming bigrams (frequency-ordered)
pub fn code_bigrams() -> Vec<Bigram> {
    vec![
        Bigram::new("->", 0.95, &["x -> y", "fn() ->", "|x| -> x"]),
        Bigram::new("::", 0.90, &["std::", "Vec::", "Self::"]),
        Bigram::new("=>", 0.88, &["x => x", "match => {", "() => {}"]),
        Bigram::new("!=", 0.85, &["x != y", "!= null", "!= 0"]),
        Bigram::new("==", 0.83, &["x == y", "== null", "== 0"]),
        Bigram::new("<=", 0.80, &["x <= y", "<= 10", "<= max"]),
        Bigram::new(">=", 0.78, &["x >= y", ">= 0", ">= min"]),
        Bigram::new("&&", 0.75, &["x && y", "&& true", "if x &&"]),
        Bigram::new("||", 0.72, &["x || y", "|| false", "if x ||"]),
        Bigram::new("//", 0.70, &["// comment", "// TODO", "// FIXME"]),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_french_bigrams_frequency_order() {
        let bigrams = french_bigrams();

        // Verify descending frequency order
        for i in 0..bigrams.len() - 1 {
            assert!(
                bigrams[i].frequency >= bigrams[i + 1].frequency,
                "Bigrams should be ordered by frequency"
            );
        }
    }

    #[test]
    fn test_english_bigrams_frequency_order() {
        let bigrams = english_bigrams();

        for i in 0..bigrams.len() - 1 {
            assert!(bigrams[i].frequency >= bigrams[i + 1].frequency);
        }
    }

    #[test]
    fn test_code_bigrams_frequency_order() {
        let bigrams = code_bigrams();

        for i in 0..bigrams.len() - 1 {
            assert!(bigrams[i].frequency >= bigrams[i + 1].frequency);
        }
    }

    #[test]
    fn test_bigram_structure() {
        let bigrams = french_bigrams();

        assert!(!bigrams.is_empty());

        // Check first bigram
        let first = &bigrams[0];
        assert_eq!(first.pattern, "qu");
        assert_eq!(first.frequency, 0.95);
        assert!(!first.examples.is_empty());
        assert!(first.examples.contains(&"que".to_string()));
    }

    #[test]
    fn test_all_bigrams_have_examples() {
        let all_bigrams = vec![french_bigrams(), english_bigrams(), code_bigrams()];

        for bigram_set in all_bigrams {
            for bigram in bigram_set {
                assert!(
                    !bigram.examples.is_empty(),
                    "Bigram '{}' should have examples",
                    bigram.pattern
                );
            }
        }
    }
}
