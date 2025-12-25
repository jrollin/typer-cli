# Bigram Training - Requirements

> **Purpose**: Captures requirements for bigram (two-letter combination) practice
> **Module**: `src/content/` (extension)
> **Next Step**: See `design.md` for bigram generation strategy

## Bigram Practice Requirements

### R14: Bigram Training Mode
THE SYSTEM SHALL provide dedicated lessons for practicing common two-letter combinations (bigrams)

### R15: French Bigram Support
WHEN a user selects French bigram training
THE SYSTEM SHALL generate practice content with common French bigrams including:
- "qu" (queue, qui, que)
- "ou" (pour, vous, nous)
- "en" (en, ment, bien)
- "on" (on, bon, son)
- "an" (an, dans, avant)
- "es" (les, des, mes)
- "ai" (ai, mais, fait)
- "er" (er, aller, premier)
- "re" (re, très, entre)
- "de" (de, des)

### R16: English Bigram Support
WHEN a user selects English bigram training
THE SYSTEM SHALL generate practice content with common English bigrams including:
- "th" (the, that, with)
- "he" (he, the, when)
- "in" (in, ing, thing)
- "er" (er, over, after)
- "an" (an, and, can)
- "re" (re, are, were)
- "on" (on, one, upon)
- "at" (at, that, what)
- "en" (en, been, when)
- "ed" (ed, used, called)

### R17: Code Bigram Support
WHEN a user selects code bigram training
THE SYSTEM SHALL generate practice content with common programming bigrams including:
- "->" (arrow/pointer)
- "::" (scope resolution)
- "=>" (fat arrow)
- "!=" (not equal)
- "==" (equality)
- "<=" (less than or equal)
- ">=" (greater than or equal)
- "&&" (logical and)
- "||" (logical or)
- "//" (comment)

### R18: Bigram Difficulty Levels
THE SYSTEM SHALL organize bigram lessons in progressive difficulty:
- Level 1: 5 most common bigrams
- Level 2: 10 most common bigrams
- Level 3: All 20+ common bigrams mixed

### R19: Bigram Context Generation
WHEN generating bigram practice content
THE SYSTEM SHALL embed bigrams in realistic word contexts rather than isolated pairs
- Example: "the quick" instead of "th th th"
- Example: "qu qu qu" for pure drill, then "qui que quoi" for context

### R20: Bigram Frequency Weighting
THE SYSTEM SHALL weight bigram appearance frequency based on real-world usage data
- More common bigrams appear more frequently in practice
- Adaptive difficulty based on user error rates

## Integration Requirements

### R21: Bigram Lesson Selection
THE SYSTEM SHALL add bigram training options to the lesson selection menu
- French Bigrams
- English Bigrams
- Code Bigrams

### R22: Bigram Statistics Tracking
WHEN a user completes a bigram lesson
THE SYSTEM SHALL track per-bigram accuracy and speed metrics
- Which bigrams were typed correctly
- Average time per bigram
- Error patterns per bigram

## Future Phase Requirements

### R23: Custom Bigram Sets (Phase 3+)
FUTURE: Allow users to create custom bigram practice sets
FUTURE: Import bigrams from user's own code/text samples
FUTURE: Language-specific bigrams (Spanish, German, etc.)

### R24: Bigram Mastery Tracking (Phase 3+)
FUTURE: Track mastery level for each bigram
FUTURE: Recommend practice based on weak bigrams
FUTURE: Spaced repetition for bigram retention
