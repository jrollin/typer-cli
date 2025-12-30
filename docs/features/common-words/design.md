# Common Word Training - Technical Design

**Feature**: Common Word Training
**Phase**: 3
**Status**: ✅ Complete
**Last Updated**: 2025-12-30

## Architecture Overview

Common Word Training follows the established bigram/trigram pattern with a data module and generator module, integrated into the existing lesson system.

## Component Design

### 1. Data Module (`src/content/common_word.rs`)

**Purpose**: Store word data and frequency information

```rust
pub struct Word {
    pub text: String,
    pub frequency: f32,    // 0.70-1.00 normalized
    pub length: usize,      // character count for future analytics
}

pub fn french_words() -> Vec<Word>   // 500 words
pub fn english_words() -> Vec<Word>  // 500 words
```

**Key Decisions**:
- No `examples` array (unlike bigrams/trigrams) - words are self-contained
- Include `length` field for future filtering/analytics
- Frequency normalization: 1.00 (most common) → 0.70 (least common)
- Uses shared `Language` enum from bigram module

**Data Sources**:
- **English**: COCA corpus (Corpus of Contemporary American English)
- **French**: Lexique 3.83 database

**Top 10 Words**:
- English: the, be, to, of, and, a, in, that, have, I
- French: le, de, un, être, et, à, il, avoir, ne, je

### 2. Generator Module (`src/content/common_word_generator.rs`)

**Purpose**: Generate practice content for different levels

```rust
pub struct CommonWordGenerator {
    words: Vec<Word>,
}

impl CommonWordGenerator {
    pub fn new(language: Language) -> Self
    pub fn generate(&self, level: usize, length: usize) -> String
    fn select_words_for_level(&self, level: usize) -> Vec<&Word>
    fn generate_drill_mode(&self, words: &[&Word], length: usize) -> String
    fn generate_sentence_mode(&self, words: &[&Word], length: usize) -> String
}
```

**Level-Based Word Selection**:
| Level | Word Count | Description |
|-------|-----------|-------------|
| 1 | 50 | Top 50 most common |
| 2 | 100 | Top 100 most common |
| 3 | 200 | Top 200 most common |
| 4 | 500 | All 500 words |

**Generation Modes**:

1. **Drill Mode (Level 1)**
   - Pattern: "the the be be to to"
   - Each word repeated 2 times consecutively
   - Focus: Muscle memory for most common words
   - Loop through selected words in order

2. **Sentence Mode (Levels 2-4)**
   - Natural word sequences
   - Frequency-weighted random selection:
     - 70% from top 20% of words (high-frequency bias)
     - 30% from full word pool
   - Focus: Realistic typing practice
   - Uses `rand::gen_range()` for randomization

**Character Counting**:
```rust
// CRITICAL: Use .chars().count() not .len()
while result.chars().count() < length {
    // ...
}
result.chars().take(length).collect()
```
- Handles UTF-8 multi-byte characters correctly (French accents)
- Ensures accurate length constraints

### 3. Lesson Type Extension (`src/content/lesson.rs`)

**Enum Variant**:
```rust
pub enum LessonType {
    // ... existing variants
    CommonWords {
        language: Language,
        level: usize,
    },
}
```

**Factory Method**:
```rust
pub fn common_word_lessons(language: Language) -> Vec<Lesson> {
    vec![
        Lesson::new(LessonType::CommonWords { language, level: 1 }, ...),
        Lesson::new(LessonType::CommonWords { language, level: 2 }, ...),
        Lesson::new(LessonType::CommonWords { language, level: 3 }, ...),
        Lesson::new(LessonType::CommonWords { language, level: 4 }, ...),
    ]
}
```

**Lesson Titles**:
- French Words - Level 1: "Top 50 most common words"
- French Words - Level 2: "Top 100 most common words"
- French Words - Level 3: "Top 200 most common words"
- French Words - Level 4: "All 500 most common words"

(Same pattern for English)

### 4. Content Generator Integration (`src/content/generator.rs`)

**Match Arm**:
```rust
LessonType::CommonWords { language, level } => {
    let generator = CommonWordGenerator::new(*language);
    generator.generate(*level, length)
}
```

**Import**:
```rust
use super::common_word_generator::CommonWordGenerator;
```

### 5. Menu Integration (`src/app.rs`)

**Lesson Order**:
```
...
French Bigrams (3 lessons)
English Bigrams (3 lessons)
Code Bigrams (4 lessons)
French Trigrams (4 lessons)
English Trigrams (4 lessons)
French Common Words (4 lessons)  ← NEW
English Common Words (4 lessons) ← NEW
TypeScript Code Symbols (6 lessons)
Rust Code Symbols (6 lessons)
Python Code Symbols (6 lessons)
```

**Implementation**:
```rust
lessons.extend(Lesson::common_word_lessons(Language::French));
lessons.extend(Lesson::common_word_lessons(Language::English));
```

## Data Research Script (`scripts/fetch_common_words.py`)

**Purpose**: Generate Rust code from corpus data

**Features**:
- Reads word frequency data from corpus sources
- Normalizes frequencies to 0.70-1.00 range
- Generates `Word::new()` calls for Rust code
- Outputs statistics (top words, frequency range)

**Usage**:
```bash
python3 scripts/fetch_common_words.py > output.txt
# Copy generated Rust code to common_word.rs
```

**Sample Output**:
```rust
pub fn english_words() -> Vec<Word> {
    vec![
        Word::new("the", 1.000),
        Word::new("be", 0.999),
        // ... 498 more
    ]
}
```

## Testing Strategy

### Unit Tests (13 new tests)

**Data Module Tests** (`common_word.rs`):
1. `test_word_count` - Verify 500 words per language
2. `test_frequency_order_english` - Descending frequency order
3. `test_frequency_order_french` - Descending frequency order
4. `test_word_structure` - Word struct fields
5. `test_frequency_range` - 1.00 to ≥0.70
6. `test_word_length_stored` - Character length calculation

**Generator Module Tests** (`common_word_generator.rs`):
1. `test_drill_mode_has_repetition` - Word repetition in drill mode
2. `test_sentence_mode_has_variety` - Multiple unique words
3. `test_level_word_selection` - Correct counts (50/100/200/500)
4. `test_content_length_constraint` - Character length limits
5. `test_empty_on_invalid_level` - Level 0, 5+ return empty
6. `test_drill_mode_uses_top_words` - Contains most common words
7. `test_sentence_mode_frequency_bias` - High-frequency words appear more

**Total Test Count**: 129 tests (116 before + 13 new)

## Design Decisions & Rationale

### 1. No Examples Array
**Decision**: Word struct has no `examples` field (unlike Bigram/Trigram)

**Rationale**:
- Words are self-contained practice units
- No need for contextual examples
- Memory efficiency: 500 words vs 5000 example strings
- Simpler data structure

### 2. Two Generation Modes (vs Three)
**Decision**: Drill + Sentence (no "Word" mode like trigrams)

**Rationale**:
- Drill mode sufficient for memorization (Level 1)
- Sentence mode handles realistic practice (Levels 2-4)
- Words don't need intermediate "word context" like bigrams do
- Simplifies implementation

### 3. Frequency Weighting (70/30 Split)
**Decision**: Sentence mode uses 70% from top 20%, 30% from full pool

**Rationale**:
- Mimics natural language distribution
- High-frequency words get more practice (better learning)
- More realistic than uniform random
- Balances common words with variety

### 4. Character-Based Length Counting
**Decision**: Use `.chars().count()` not `.len()`

**Rationale**:
- French words have multi-byte UTF-8 characters (é, è, à)
- `.len()` counts bytes (incorrect)
- `.chars().count()` counts Unicode scalar values (correct)
- Essential for accurate length constraints

### 5. Length Metadata Included
**Decision**: Word struct stores `length: usize`

**Rationale**:
- Future feature: filter words by length
- Analytics: performance by word length
- Minimal cost: single usize per word
- Pre-computed (not calculated on-demand)

## File Structure

```
src/content/
├── common_word.rs              # Data structures, 500-word lists
├── common_word_generator.rs    # Content generation logic
├── generator.rs                 # +CommonWords match arm
├── lesson.rs                    # +CommonWords enum variant
└── mod.rs                       # +module exports

scripts/
└── fetch_common_words.py       # Data generation tool

docs/features/common-words/
├── requirements.md              # EARS format requirements
└── design.md                    # This file
```

## Performance Characteristics

**Memory**:
- 500 words × 2 languages = 1000 Word structs
- Average word length: ~6 chars = ~6 bytes
- Total text: ~6KB
- Frequency + length: ~12 bytes per word
- **Total**: ~18KB for all word data

**Generation Speed**:
- Drill mode: O(n) where n = character length
- Sentence mode: O(n) with random selection overhead
- Negligible impact (<1ms for 500 chars)

**Randomization**:
- Uses `rand::thread_rng()` (non-deterministic)
- Frequency-weighted selection: O(1) per word
- No sorting or complex algorithms

## Integration Points

**Shared Components**:
- `Language` enum from `bigram.rs`
- `ContentGenerator` trait from `generator.rs`
- `Lesson` factory pattern from `lesson.rs`

**External Dependencies**:
- `rand` crate for random selection
- `serde` (inherited) for future JSON export

## Future Enhancements

**Potential Additions**:
1. Word difficulty ratings (beyond frequency)
2. Part-of-speech filtering
3. Adaptive word selection based on user errors
4. Sentence templates for more natural flow
5. Word definitions/translations for learning mode

**Not Planned**:
- Grammar rules or syntax checking
- Language-specific sentence structure
- Multi-word phrases or idioms
