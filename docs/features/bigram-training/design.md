# Bigram Training - Design Document

> **Purpose**: Technical design for bigram practice content generation
> **Module**: `src/content/` (extension)
> **Previous Step**: See `requirements.md` for bigram requirements
> **Related**: See `../home-row-lessons/design.md` for content generation patterns

## Overview

The bigram training system generates practice content focused on common two-letter combinations. It uses frequency-based selection and realistic word contexts to build typing fluency.

## Architecture

### Data Structures

```rust
// Language enum for bigram selection
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Language {
    French,
    English,
}

// Bigram type classification
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum BigramType {
    Natural,  // Language bigrams (qu, th, er)
    Code,     // Programming symbols (-> :: =>)
}

// Bigram definition with frequency
#[derive(Debug, Clone)]
pub struct Bigram {
    pub pattern: String,
    pub frequency: f32,  // 0.0 to 1.0
    pub examples: Vec<String>,
}

// Extended Lesson type
pub enum LessonType {
    HomeRow { level: usize },
    Bigram {
        bigram_type: BigramType,
        language: Option<Language>,
        level: usize,  // 1=basic, 2=intermediate, 3=advanced
    },
}
```

### Bigram Data

**French Bigrams** (frequency-ordered):
```rust
const FRENCH_BIGRAMS: &[(&str, f32, &[&str])] = &[
    ("qu", 0.95, &["que", "qui", "quoi", "quelque"]),
    ("ou", 0.90, &["pour", "vous", "nous", "ou"]),
    ("en", 0.88, &["en", "ment", "bien", "rien"]),
    ("on", 0.85, &["on", "bon", "son", "non"]),
    ("es", 0.83, &["les", "des", "mes", "ses"]),
    ("er", 0.80, &["premier", "dernier", "aller"]),
    ("re", 0.78, &["re", "très", "entre", "être"]),
    ("de", 0.75, &["de", "des", "depuis", "devant"]),
    ("ai", 0.72, &["ai", "mais", "fait", "jamais"]),
    ("an", 0.70, &["an", "dans", "avant", "sans"]),
];
```

**English Bigrams**:
```rust
const ENGLISH_BIGRAMS: &[(&str, f32, &[&str])] = &[
    ("th", 0.95, &["the", "that", "with", "this"]),
    ("he", 0.90, &["he", "the", "when", "where"]),
    ("in", 0.88, &["in", "thing", "nothing", "into"]),
    ("er", 0.85, &["her", "over", "after", "never"]),
    ("an", 0.83, &["an", "and", "can", "than"]),
    ("re", 0.80, &["are", "were", "here", "there"]),
    ("on", 0.78, &["on", "one", "upon", "only"]),
    ("at", 0.75, &["at", "that", "what", "late"]),
    ("en", 0.72, &["been", "when", "then", "open"]),
    ("ed", 0.70, &["used", "called", "asked", "moved"]),
];
```

**Code Bigrams**:
```rust
const CODE_BIGRAMS: &[(&str, f32, &[&str])] = &[
    ("->", 0.95, &["x -> y", "fn() ->", "|x| -> x"]),
    ("::", 0.90, &["std::", "Vec::", "Self::"]),
    ("=>", 0.88, &["x => x", "match => {", "() => {}"]),
    ("!=", 0.85, &["x != y", "!= null", "!= 0"]),
    ("==", 0.83, &["x == y", "== null", "== 0"]),
    ("<=", 0.80, &["x <= y", "<= 10", "<= max"]),
    (">=", 0.78, &["x >= y", ">= 0", ">= min"]),
    ("&&", 0.75, &["x && y", "&& true", "if x &&"]),
    ("||", 0.72, &["x || y", "|| false", "if x ||"]),
    ("//", 0.70, &["// comment", "// TODO", "// FIXME"]),
];
```

## Content Generation

### Level-Based Generation

```rust
impl BigramGenerator {
    pub fn generate(&self, level: usize, length: usize) -> String {
        let bigrams = self.select_bigrams_for_level(level);

        match level {
            1 => self.generate_drill_mode(bigrams, length),
            2 => self.generate_word_mode(bigrams, length),
            3 => self.generate_mixed_mode(bigrams, length),
            _ => String::new(),
        }
    }

    fn select_bigrams_for_level(&self, level: usize) -> Vec<&Bigram> {
        let count = match level {
            1 => 5,   // Top 5 most common
            2 => 10,  // Top 10
            3 => 20,  // All available
            _ => 5,
        };

        self.bigrams
            .iter()
            .take(count)
            .collect()
    }
}
```

### Generation Modes

**1. Drill Mode (Level 1)**: Pure bigram repetition
```rust
fn generate_drill_mode(&self, bigrams: Vec<&Bigram>, length: usize) -> String {
    let mut result = String::new();
    let mut idx = 0;

    while result.len() < length {
        if !result.is_empty() {
            result.push(' ');
        }

        let bigram = &bigrams[idx % bigrams.len()];
        // Repeat the bigram 3-4 times
        result.push_str(&format!("{} {} {}",
            bigram.pattern, bigram.pattern, bigram.pattern));

        idx += 1;
    }

    result.chars().take(length).collect()
}

// Output: "qu qu qu ou ou ou en en en qu qu qu..."
```

**2. Word Mode (Level 2)**: Bigrams in word context
```rust
fn generate_word_mode(&self, bigrams: Vec<&Bigram>, length: usize) -> String {
    let mut result = String::new();
    let mut idx = 0;

    while result.len() < length {
        if !result.is_empty() {
            result.push(' ');
        }

        let bigram = &bigrams[idx % bigrams.len()];
        let word = &bigram.examples[idx % bigram.examples.len()];
        result.push_str(word);

        idx += 1;
    }

    result.chars().take(length).collect()
}

// Output: "que qui quoi pour vous nous en bien..."
```

**3. Mixed Mode (Level 3)**: Realistic sentences
```rust
fn generate_mixed_mode(&self, bigrams: Vec<&Bigram>, length: usize) -> String {
    // Select words from examples and form natural sentences
    let sentences = vec![
        "que voulez vous faire",
        "nous sommes en train de bien travailler",
        "quoi de nouveau pour vous",
    ];

    sentences.join(" ").chars().take(length).collect()
}

// Output: Natural flowing text with target bigrams
```

## Integration

### Extended Lesson Structure

```rust
impl Lesson {
    pub fn bigram_lessons(language: Language) -> Vec<Lesson> {
        vec![
            Lesson::new(
                LessonType::Bigram {
                    bigram_type: BigramType::Natural,
                    language: Some(language),
                    level: 1,
                },
                format!("{:?} Bigrams - Level 1", language),
                "Top 5 common bigrams (drill mode)".to_string(),
                vec![], // Bigrams don't use char keys
            ),
            Lesson::new(
                LessonType::Bigram {
                    bigram_type: BigramType::Natural,
                    language: Some(language),
                    level: 2,
                },
                format!("{:?} Bigrams - Level 2", language),
                "Top 10 bigrams (word context)".to_string(),
                vec![],
            ),
            Lesson::new(
                LessonType::Bigram {
                    bigram_type: BigramType::Natural,
                    language: Some(language),
                    level: 3,
                },
                format!("{:?} Bigrams - Level 3", language),
                "All bigrams (realistic text)".to_string(),
                vec![],
            ),
        ]
    }

    pub fn code_bigram_lessons() -> Vec<Lesson> {
        vec![
            Lesson::new(
                LessonType::Bigram {
                    bigram_type: BigramType::Code,
                    language: None,
                    level: 1,
                },
                "Code Bigrams - Level 1".to_string(),
                "Arrow operators (-> => ::)".to_string(),
                vec![],
            ),
            // ... more levels
        ]
    }
}
```

### Menu Organization

```
Home Row Lessons
├── Level 1-6 (existing)

Bigram Training
├── French Bigrams
│   ├── Level 1 - Top 5 (drill)
│   ├── Level 2 - Top 10 (words)
│   └── Level 3 - All (sentences)
├── English Bigrams
│   ├── Level 1 - Top 5 (drill)
│   ├── Level 2 - Top 10 (words)
│   └── Level 3 - All (sentences)
└── Code Bigrams
    ├── Level 1 - Arrows
    ├── Level 2 - Comparisons
    └── Level 3 - All symbols
```

## Testing Strategy

```rust
#[cfg(test)]
mod tests {
    #[test]
    fn test_bigram_frequency_ordering() {
        let bigrams = FrenchBigrams::new();

        // Verify descending frequency order
        for i in 0..bigrams.len()-1 {
            assert!(bigrams[i].frequency >= bigrams[i+1].frequency);
        }
    }

    #[test]
    fn test_bigram_drill_generation() {
        let gen = BigramGenerator::new(Language::French);
        let content = gen.generate_drill_mode(/* ... */);

        // Should contain target bigram multiple times
        assert!(content.contains("qu qu qu"));
        assert!(content.len() >= 50);
    }

    #[test]
    fn test_bigram_word_context() {
        let gen = BigramGenerator::new(Language::French);
        let content = gen.generate_word_mode(/* ... */);

        // Should contain real words, not just bigrams
        assert!(content.contains("que") || content.contains("qui"));
        assert!(!content.contains("qu qu qu")); // No drill patterns
    }
}
```

## File Locations

- `src/content/bigram.rs` - Bigram data structures and definitions
- `src/content/bigram_generator.rs` - Content generation logic
- `src/content/lesson.rs` - Extended with BigramType

## Performance Considerations

- **Deterministic generation**: Same level = same content
- **Pre-computed bigram lists**: Load once at startup
- **Memory efficient**: Store only frequency and examples, generate on demand
- **Fast generation**: O(n) where n = desired length
