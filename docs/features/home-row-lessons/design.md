# Home Row Lessons - Design Document

> **Purpose**: Technical design for home row lesson content generation
> **Module**: `src/content/`
> **Previous Step**: See `requirements.md` for lesson requirements
> **Related**: See `../keyboard-layout/design.md` for AZERTY layout

## Overview

The content generation system creates progressive practice lessons for AZERTY home row mastery. It uses deterministic generation for consistent practice sessions and integrates with the keyboard layout module for key selection.

## Lesson Generation Strategy

### Progressive Home Row Lessons

**Level-by-level progression:**

```rust
// Level 1: Index fingers only (f, j)
"ff jj ff jj fj jf fj jf"

// Level 2: Add middle fingers (d, k)
"dd kk ff jj dk kd fj dk"

// Level 3: Add ring fingers (s, l)
"ss ll dd kk sl ls fj dk"

// Level 4: Add pinkies (q, m)
"qq mm ss ll qm mq sl dk"

// Level 5: All home row
"qsdf jklm qwer asdf jklm"

// Level 6: French words
"les des mes ses tes"
```

### Implementation

```rust
pub trait ContentGenerator {
    fn generate(&self, level: u8) -> String;
}

impl ContentGenerator for HomeRowGenerator {
    fn generate(&self, level: u8) -> String {
        match level {
            1 => self.generate_two_keys(&['f', 'j']),
            2 => self.generate_four_keys(&['f', 'j', 'd', 'k']),
            3 => self.generate_six_keys(&['f', 'j', 'd', 'k', 's', 'l']),
            4 => self.generate_eight_keys(&['f', 'j', 'd', 'k', 's', 'l', 'q', 'm']),
            5 => self.generate_all_home_row(),
            6 => self.generate_french_words(),
            _ => self.generate(&1), // Default to level 1
        }
    }
}
```

**Design Decisions:**
- **Deterministic**: Same level = same content (predictable for testing)
- **Progressive difficulty**: Gradual finger addition
- **Appropriate length**: 50-100 chars (1-2 minutes of practice)
- **Space-separated**: Enables accurate WPM calculation

### Content Generation Algorithms

**Two-key pattern:**
```rust
fn generate_two_keys(&self, keys: &[char; 2]) -> String {
    let mut content = String::new();

    // Repetition phase
    for key in keys {
        content.push_str(&format!("{}{} ", key, key));
    }

    // Alternation phase
    for _ in 0..10 {
        content.push(keys[0]);
        content.push(keys[1]);
        content.push(' ');
    }

    content.trim().to_string()
}
```

**French word generation (Level 6):**
```rust
fn generate_french_words(&self) -> String {
    let words = vec![
        "les", "des", "mes", "ses", "tes",
        "je", "le", "de", "que", "se",
        // Only words using home row keys: q,s,d,f,g,h,j,k,l,m
    ];

    words.join(" ")
}
```

## Lesson Types

### Current (Phase 1)

```rust
pub enum Lesson {
    HomeRow { level: u8 },
}

impl Lesson {
    pub fn name(&self) -> String {
        match self {
            Lesson::HomeRow { level } => format!("Home Row - Level {}", level),
        }
    }

    pub fn difficulty(&self) -> u8 {
        match self {
            Lesson::HomeRow { level } => *level,
        }
    }
}
```

### Future Extensions (Phase 2+)

```rust
// Bigram generation
pub enum Lesson {
    HomeRow { level: u8 },
    Bigram { language: Language },
    Code { lang: ProgrammingLanguage },
    Adaptive { weak_keys: Vec<char> },
}

impl ContentGenerator for BigramGenerator {
    fn generate(&self, language: Language) -> String {
        match language {
            Language::French => self.generate_bigrams(&["qu", "ou", "en", "on"]),
            Language::English => self.generate_bigrams(&["th", "er", "he", "an"]),
        }
    }
}

// Code symbol generation
impl ContentGenerator for CodeGenerator {
    fn generate(&self, lang: ProgrammingLanguage) -> String {
        match lang {
            ProgrammingLanguage::Rust => "fn main() { let x = 42; }",
            ProgrammingLanguage::TypeScript => "const x: number = 42;",
            ProgrammingLanguage::Python => "def main(): x = 42",
        }
    }
}
```

## Integration with Keyboard Layout

The content generator uses the keyboard layout module to determine valid keys:

```rust
use crate::keyboard::azerty::AZERTY_HOME_ROW;

fn generate_all_home_row(&self) -> String {
    let keys: Vec<char> = AZERTY_HOME_ROW.chars().collect();
    // Generate content using all home row keys
}
```

## Testing Strategy

### Unit Tests

```rust
#[cfg(test)]
mod tests {
    #[test]
    fn test_home_row_level_1() {
        let generator = HomeRowGenerator::new();
        let content = generator.generate(1);

        // Should only contain 'f', 'j', and spaces
        assert!(content.chars().all(|c| c == 'f' || c == 'j' || c == ' '));

        // Should be appropriate length
        assert!(content.len() >= 50 && content.len() <= 150);
    }

    #[test]
    fn test_deterministic_generation() {
        let generator = HomeRowGenerator::new();
        let content1 = generator.generate(1);
        let content2 = generator.generate(1);

        assert_eq!(content1, content2);
    }

    #[test]
    fn test_french_words_home_row_only() {
        let generator = HomeRowGenerator::new();
        let content = generator.generate(6);

        // All characters should be from home row
        let valid_chars: HashSet<char> = "qsdfghjklm ".chars().collect();
        assert!(content.chars().all(|c| valid_chars.contains(&c)));
    }
}
```

## File Locations

- `src/content/lesson.rs` - Lesson type definitions
- `src/content/generator.rs` - Content generation implementations
- `src/content/mod.rs` - Module exports
