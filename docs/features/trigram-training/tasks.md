# Trigram Training - Tasks

## Overview
This document outlines the tasks required to implement the Trigram Training feature. The tasks are organized by module and priority.

## Tasks

### 1. Research and Data Preparation
- [x] **Task 1.1**: Run `scripts/fetch_trigrams.py` to generate trigram data for English and French.
- [x] **Task 1.2**: Validate the generated trigram data for accuracy and completeness.
- [x] **Task 1.3**: Normalize trigram frequencies to the 0.70-1.00 range.

### 2. Data Structures and Trigram Definition
- [x] **Task 2.1**: Implement the `Trigram` struct in `src/content/trigram.rs`.
- [x] **Task 2.2**: Define `french_trigrams()` and `english_trigrams()` functions in `src/content/trigram.rs`.
- [x] **Task 2.3**: Ensure each trigram includes 10 example words for variety.
- [x] **Task 2.4**: Validate that all trigrams are compatible with the AZERTY layout.

### 3. Trigram Generator
- [x] **Task 3.1**: Implement the `TrigramGenerator` struct in `src/content/trigram_generator.rs`.
- [x] **Task 3.2**: Add `new()` method to initialize the generator with language-specific trigrams.
- [x] **Task 3.3**: Implement `generate()` method to support drill, word, and mixed modes.
- [x] **Task 3.4**: Add `select_trigrams_for_level()` method to filter trigrams by difficulty level.
- [x] **Task 3.5**: Implement `generate_drill_mode()`, `generate_word_mode()`, and `generate_mixed_mode()` methods.

### 4. Lesson Integration
- [x] **Task 4.1**: Extend the `LessonType` enum in `src/content/lesson.rs` to include `Trigram`.
- [x] **Task 4.2**: Implement `trigram_lessons()` function in `src/content/lesson.rs` to generate lessons for all levels.
- [x] **Task 4.3**: Add metadata (title, description) for trigram lessons.

### 5. Menu Integration
- [x] **Task 5.1**: Add trigram lessons to the lesson selection menu in `src/app.rs`.
- [x] **Task 5.2**: Ensure trigram lessons are categorized under "French Trigrams" and "English Trigrams".

### 6. Testing
- [x] **Task 6.1**: Write unit tests for the `Trigram` struct and data in `src/content/trigram.rs`.
- [x] **Task 6.2**: Write unit tests for the `TrigramGenerator` in `src/content/trigram_generator.rs`.
- [x] **Task 6.3**: Verify that all trigrams and their examples are valid for the AZERTY layout.
- [x] **Task 6.4**: Test trigram lesson generation and integration with the menu system.

### 7. Documentation
- [x] **Task 7.1**: Update `docs/features/trigram-training/requirements.md` with any changes.
- [x] **Task 7.2**: Update `docs/features/trigram-training/design.md` with implementation details.
- [x] **Task 7.3**: Ensure all tasks in this file are updated as they are completed.

### 8. Future Work
- [ ] **Task 8.1**: Add support for multi-layout trigram generation (e.g., QWERTY, BÉPO).
- [ ] **Task 8.2**: Implement custom trigram sets for user-provided trigrams.
- [ ] **Task 8.3**: Add language-specific trigrams (e.g., Spanish, German).
- [ ] **Task 8.4**: Integrate trigram mastery tracking and spaced repetition.