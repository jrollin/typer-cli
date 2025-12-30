# Common Word Training - Requirements

**Feature**: Common Word Training
**Phase**: 3
**Status**: ✅ Complete
**Last Updated**: 2025-12-30

## Overview

Extends typing practice with the 500 most common words in French and English, enabling realistic vocabulary training with progressive difficulty levels.

## Requirements (EARS Format)

### R35: Word Database

WHEN the system initializes common word training
THE SYSTEM SHALL provide 500 most common words per language (French, English)

WHEN words are selected from corpus data
THE SYSTEM SHALL normalize frequency weights to 0.70-1.00 range

WHEN words are ordered in the database
THE SYSTEM SHALL sort by descending frequency (most common first)

### R36: Progressive Difficulty Levels

WHEN user selects Level 1
THE SYSTEM SHALL provide practice with top 50 most common words

WHEN user selects Level 2
THE SYSTEM SHALL provide practice with top 100 most common words

WHEN user selects Level 3
THE SYSTEM SHALL provide practice with top 200 most common words

WHEN user selects Level 4
THE SYSTEM SHALL provide practice with all 500 most common words

### R37: Drill Mode (Level 1)

WHEN user practices Level 1 common words
THE SYSTEM SHALL generate word repetition drills (e.g., "the the be be to to")

WHEN generating drill content
THE SYSTEM SHALL repeat each word exactly 2 times consecutively

### R38: Sentence Mode (Levels 2-4)

WHEN user practices Levels 2-4 common words
THE SYSTEM SHALL generate natural word sequences mimicking sentences

WHEN selecting words for sentence mode
THE SYSTEM SHALL use frequency-weighted random selection

WHEN applying frequency weighting
THE SYSTEM SHALL select 70% of words from top 20% (high-frequency)

WHEN applying frequency weighting
THE SYSTEM SHALL select 30% of words from full pool (all levels)

### R39: Lesson Integration

WHEN user views lesson menu
THE SYSTEM SHALL display 4 French common word lessons

WHEN user views lesson menu
THE SYSTEM SHALL display 4 English common word lessons

WHEN lessons are displayed
THE SYSTEM SHALL position common word lessons after trigram lessons

### R40: Content Generation

WHEN generating practice content
THE SYSTEM SHALL limit output to specified character length

WHEN character length is reached
THE SYSTEM SHALL truncate content at exact character boundary

WHEN using multi-byte UTF-8 characters
THE SYSTEM SHALL count characters (not bytes) for length constraints

### R41: Data Sources

WHEN sourcing English word frequencies
THE SYSTEM SHALL use COCA (Corpus of Contemporary American English) data

WHEN sourcing French word frequencies
THE SYSTEM SHALL use Lexique 3.83 database

### R42: Testing Requirements

WHEN testing word databases
THE SYSTEM SHALL verify exactly 500 words per language

WHEN testing frequency ordering
THE SYSTEM SHALL verify descending frequency order for all words

WHEN testing content generation
THE SYSTEM SHALL verify drill mode produces word repetition

WHEN testing content generation
THE SYSTEM SHALL verify sentence mode produces word variety

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
