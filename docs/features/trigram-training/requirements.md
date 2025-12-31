# Trigram Training - Requirements

> **Purpose**: Captures requirements for trigram (three-letter combination) practice
> **Module**: `src/content/` (extension)
> **Next Step**: See `design.md` for trigram generation strategy

## Trigram Practice Requirements

### US-1 Trigram Training Mode
THE SYSTEM SHALL provide dedicated lessons for practicing common three-letter combinations (trigrams)

### US-2 French Trigram Support
WHEN a user selects French trigram training
THE SYSTEM SHALL generate practice content with the 20 most common French trigrams including:
- "les" (les, lesquels)
- "des" (des, depuis)
- "ent" (ment, moment, parent)
- "ion" (action, nation, question)
- "que" (que, quelque, chaque)
- "ons" (nous, sons, raisons)
- "ant" (avant, pendant, durant)
- "ait" (fait, avait, était)
- "est" (est, ouest, reste)
- "our" (pour, jour, toujours)
- And 10 more high-frequency trigrams

### US-3 English Trigram Support
WHEN a user selects English trigram training
THE SYSTEM SHALL generate practice content with the 20 most common English trigrams including:
- "the" (the, them, these)
- "and" (and, hand, stand)
- "ing" (ing, thing, going)
- "ion" (tion, nation, action)
- "ent" (ent, went, bent)
- "for" (for, before, forget)
- "her" (her, here, where)
- "ter" (ter, after, water)
- "hat" (that, what, chat)
- "thi" (this, thing, think)
- And 10 more high-frequency trigrams

### US-4 Trigram Difficulty Levels
THE SYSTEM SHALL organize trigram lessons in progressive difficulty:
- Level 1: 5 most common trigrams (drill mode)
- Level 2: 10 most common trigrams (word mode)
- Level 3: 15 most common trigrams (mixed mode)
- Level 4: All 20 trigrams (full practice)

### US-5 Trigram Context Generation
WHEN generating trigram practice content
THE SYSTEM SHALL use three generation modes:
- Level 1 (Drill): Pure repetition - "the the the and and and"
- Level 2 (Word): Contextual words - "the them these and hand stand"
- Level 3-4 (Mixed): Realistic sentences combining multiple trigram words

### US-6 Trigram Frequency Weighting
THE SYSTEM SHALL order trigrams by real-world frequency based on corpus analysis
- English: Based on Peter Norvig's English letter frequency data
- French: Based on Lexique database trigram frequencies
- Normalized to 0.70-1.00 range for typing practice

## Integration Requirements

### US-7 Trigram Lesson Selection
THE SYSTEM SHALL add trigram training options to the lesson selection menu
- French Trigrams - Level 1-4
- English Trigrams - Level 1-4

### US-8 Trigram Statistics Tracking
WHEN a user completes a trigram lesson
THE SYSTEM SHALL track per-trigram accuracy and speed metrics
- Integrated with existing analytics system
- Per-trigram performance data for adaptive mode

## Future Phase Requirements

### US-9 Custom Trigram Sets (Phase 3+)
FUTURE: Allow users to create custom trigram practice sets
FUTURE: Import trigrams from user's own code/text samples
FUTURE: Language-specific trigrams (Spanish, German, etc.)

### US-10 Trigram Mastery Tracking (Phase 3+)
FUTURE: Track mastery level for each trigram
FUTURE: Recommend practice based on weak trigrams
FUTURE: Spaced repetition for trigram retention
