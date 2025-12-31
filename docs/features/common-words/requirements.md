# Common Word Training - Requirements

**Feature**: Common Word Training
**Phase**: 3
**Status**: ✅ Complete
**Last Updated**: 2025-12-30

## Overview

Extends typing practice with the 500 most common words in French and English, enabling realistic vocabulary training with progressive difficulty levels.

## Requirements (EARS Format)

### US-1 Word Database

WHEN the system initializes common word training
THE SYSTEM SHALL provide 500 most common words per language (French, English)

### US-2 Normalize Frequency Weights
WHEN words are selected from corpus data
THE SYSTEM SHALL normalize frequency weights to 0.70-1.00 range

### US-3 Sort Words by Frequency
WHEN words are ordered in the database
THE SYSTEM SHALL sort by descending frequency (most common first)

### US-4 Progressive Difficulty Levels

WHEN user selects Level 1
THE SYSTEM SHALL provide practice with top 50 most common words

### US-5 Level 2 Word Selection
WHEN user selects Level 2
THE SYSTEM SHALL provide practice with top 100 most common words

### US-6 Level 3 Word Selection
WHEN user selects Level 3
THE SYSTEM SHALL provide practice with top 200 most common words

### US-7 Level 4 Word Selection
WHEN user selects Level 4
THE SYSTEM SHALL provide practice with all 500 most common words

### US-8 Drill Mode (Level 1)

WHEN user practices Level 1 common words
THE SYSTEM SHALL generate word repetition drills (e.g., "the the be be to to")

### US-9 Repeat Each Word Twice
WHEN generating drill content
THE SYSTEM SHALL repeat each word exactly 2 times consecutively

### US-10 Sentence Mode (Levels 2-4)

WHEN user practices Levels 2-4 common words
THE SYSTEM SHALL generate natural word sequences mimicking sentences

### US-11 Frequency-Weighted Selection
WHEN selecting words for sentence mode
THE SYSTEM SHALL use frequency-weighted random selection

### US-12 High-Frequency Bias
WHEN applying frequency weighting
THE SYSTEM SHALL select 70% of words from top 20% (high-frequency)

### US-13 Full Pool Selection
WHEN applying frequency weighting
THE SYSTEM SHALL select 30% of words from full pool (all levels)

### US-14 Lesson Integration

WHEN user views lesson menu
THE SYSTEM SHALL display 4 French common word lessons

### US-15 English Lessons Display
WHEN user views lesson menu
THE SYSTEM SHALL display 4 English common word lessons

### US-16 Lesson Ordering
WHEN lessons are displayed
THE SYSTEM SHALL position common word lessons after trigram lessons

### US-17 Content Generation

WHEN generating practice content
THE SYSTEM SHALL limit output to specified character length

### US-18 Truncate at Boundary
WHEN character length is reached
THE SYSTEM SHALL truncate content at exact character boundary

### US-19 UTF-8 Character Counting
WHEN using multi-byte UTF-8 characters
THE SYSTEM SHALL count characters (not bytes) for length constraints

### US-20 Data Sources

WHEN sourcing English word frequencies
THE SYSTEM SHALL use COCA (Corpus of Contemporary American English) data

### US-21 French Word Database
WHEN sourcing French word frequencies
THE SYSTEM SHALL use Lexique 3.83 database

### US-22 Testing Requirements

WHEN testing word databases
THE SYSTEM SHALL verify exactly 500 words per language

### US-23 Verify Frequency Order
WHEN testing frequency ordering
THE SYSTEM SHALL verify descending frequency order for all words

### US-24 Test Drill Mode
WHEN testing content generation
THE SYSTEM SHALL verify drill mode produces word repetition

### US-25 Test Sentence Mode
WHEN testing content generation
THE SYSTEM SHALL verify sentence mode produces word variety

### US-26 Test Level Counts
WHEN testing level selection
THE SYSTEM SHALL verify correct word counts (50/100/200/500)

## Success Criteria

- ✅ 8 new lessons in menu (French L1-4, English L1-4)
- ✅ All 129 tests pass (+13 new tests)
- ✅ 500 words per language with normalized frequencies
- ✅ Drill mode shows repetition pattern
- ✅ Sentence mode shows variety and frequency bias
- ✅ Content respects character length constraints
- ✅ UTF-8 character counting works correctly
- ✅ No duplicate words in databases

## Dependencies

- **Phase 2 (Bigrams)**: Shares Language enum
- **Phase 2 (Trigrams)**: Follows same architectural pattern
- **rand crate**: Required for frequency-weighted random selection

## Non-Requirements

- ❌ No word metadata beyond frequency (no POS tags, difficulty ratings)
- ❌ No language-specific grammar rules or sentence structure
- ❌ No word definitions or translations
- ❌ No adaptive word selection based on user performance (handled by Adaptive Mode)
