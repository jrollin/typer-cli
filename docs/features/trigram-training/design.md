# Trigram Training - Design Document

> **Purpose**: Technical design for trigram practice content generation
> **Module**: `src/content/` (extension)
> **Previous Step**: See `requirements.md` for trigram requirements
> **Related**: See `../bigram-training/design.md` for similar pattern-based content generation

## Overview

The trigram training system generates practice content focused on common three-letter combinations. It extends the bigram pattern with longer sequences, building on frequency-based selection and realistic word contexts.

## Architecture

### Data Structures

```rust
// Trigram definition with frequency (similar to Bigram)
#[derive(Debug, Clone)]
pub struct Trigram {
    pub pattern: String,      // 3-character sequence
    pub frequency: f32,        // 0.70 to 1.00 (normalized)
    pub examples: Vec<String>, // 10 example words per trigram
}

// Extended Lesson type
pub enum LessonType {
    HomeRow { level: usize },
    Bigram { bigram_type: BigramType, language: Option<Language>, level: usize },
    Trigram { language: Language, level: usize },  // NEW
}
```

### Trigram Data

**French Trigrams** (25 total, frequency-ordered, includes accented characters):
```rust
// Top 20 non-accented trigrams + 5 accented patterns
// Frequency range: 1.00 → 0.67
// Notable accented trigrams: été, ère, ées, tés, rès
// Each trigram has 10 example words for variety
```

Key highlights:
- **25 total trigrams** (20 non-accented + 5 accented)
- **Clean progression**: Level scaling 5 → 10 → 20 → 25
- **Accented patterns**: Covers é, è, ê for French typing
- **10 examples per trigram**: Ensures varied practice

**English Trigrams** (20 most common):
```rust
pub fn english_trigrams() -> Vec<Trigram> {
    vec![
        Trigram::new("the", 1.00, &["the", "them", ...]),
        Trigram::new("and", 0.99, &["and", "hand", ...]),
        Trigram::new("ing", 0.98, &["ing", "thing", ...]),
        Trigram::new("ion", 0.97, &["tion", "nation", ...]),
        Trigram::new("ent", 0.96, &["ent", "went", ...]),
        // ... 15 more trigrams
    ]
}
```

## Content Generation

### TrigramGenerator

```rust
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

    pub fn generate(&self, level: usize, length: usize) -> String {
        let selected = self.select_trigrams_for_level(level);
        match level {
            1 => self.generate_drill_mode(&selected, length),
            2 => self.generate_word_mode(&selected, length),
            3 | 4 => self.generate_mixed_mode(&selected, length),
            _ => String::new(),
        }
    }

    fn select_trigrams_for_level(&self, level: usize) -> Vec<&Trigram> {
        let count = match level {
            1 => 5,   // Top 5
            2 => 10,  // Top 10 (doubles)
            3 => 20,  // Top 20 (doubles again)
            4 => 25,  // All 25 (includes accents)
            _ => 5,
        };
        self.trigrams.iter().take(count).collect()
    }
}
```

### Generation Modes

**Level 1 - Drill Mode**:
```
"the the the and and and ing ing ing"
```
Pure repetition of trigram patterns (3x each).

**Level 2 - Word Mode**:
```
"the them these and hand stand thing going"
```
Cycles through example words for each trigram.

**Level 3-4 - Mixed Mode**:
```
"the quick brown fox and then something interesting"
```
Natural-looking text combining multiple trigram words.

## Integration Points

### File Structure
```
src/content/
├── trigram.rs              # NEW: Data structures and trigram lists
├── trigram_generator.rs    # NEW: Content generator
├── lesson.rs               # MODIFY: Add trigram lesson types
└── mod.rs                  # MODIFY: Export trigram modules
```

### Lesson Integration
```rust
// In src/content/lesson.rs
impl Lesson {
    pub fn trigram_lessons(language: Language) -> Vec<Lesson> {
        vec![
            Lesson::new(
                LessonType::Trigram { language, level: 1 },
                format!("{:?} Trigrams - Level 1", language),
                "Drill mode: Pure repetition".to_string(),
                vec![],
            ),
            // ... levels 2-4
        ]
    }
}
```

### Menu Integration
Add to lesson menu after bigram lessons:
- French Trigrams - Level 1-4 (after French Bigrams)
- English Trigrams - Level 1-4 (after English Bigrams)

## Research Script

### scripts/fetch_trigrams.py
Similar to `update_bigrams.py` but for 3-letter patterns:
- Fetch top 20 trigrams from corpus data
- Generate 10 example words per trigram
- Validate examples contain target trigram
- Output Rust code with normalized frequencies (0.70-1.00)

**Data Sources**:
- English: Peter Norvig's trigram frequency data
- French: Lexique database trigram analysis

## Testing Strategy

### Unit Tests
```rust
#[test]
fn test_trigram_structure() {
    let trigrams = french_trigrams();
    assert_eq!(trigrams.len(), 20);
    assert_eq!(trigrams[0].pattern, "les");
    assert_eq!(trigrams[0].examples.len(), 10);
}

#[test]
fn test_trigram_frequency_order() {
    let trigrams = english_trigrams();
    for i in 0..trigrams.len() - 1 {
        assert!(trigrams[i].frequency >= trigrams[i + 1].frequency);
    }
}

#[test]
fn test_examples_contain_trigrams() {
    let trigrams = french_trigrams();
    for trigram in trigrams {
        for example in &trigram.examples {
            assert!(example.to_lowercase().contains(&trigram.pattern));
        }
    }
}
```

## Implementation Sequence

1. **Research** (scripts/fetch_trigrams.py) - Generate trigram data
2. **Data** (src/content/trigram.rs) - Define structures and data
3. **Generator** (src/content/trigram_generator.rs) - Content generation logic
4. **Lessons** (src/content/lesson.rs) - Add trigram lesson types
5. **Integration** (src/app.rs) - Wire up to menu system
6. **Testing** - Verify all tests pass

## Design Decisions

### Why 25 Trigrams?
- Balances coverage vs. overwhelm (20 base + 5 accented)
- Provides clean doubling progression: 5 → 10 → 20 → 25
- Level 4 provides comprehensive practice including French accents
- Accented trigrams essential for French typing fluency

### Why 10 Examples Per Trigram?
- Consistent with updated bigram format (10 examples)
- Provides variety in practice
- Sufficient for Level 4 full practice
- Allows diverse word patterns for each trigram

### Why Same 4-Level Structure?
- Consistent progression with bigrams
- Users understand the pattern
- Reuses familiar difficulty curve

### Why No Code Trigrams?
- Code patterns are typically 2-char (operators) or full keywords
- 3-char code sequences less meaningful than 2-char
- Can be added in future if demand exists
