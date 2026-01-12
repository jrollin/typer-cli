# Common Words - Tasks

## Overview
This document outlines the tasks required to implement or verify the Common Words feature. The feature is **already complete**, but this file serves as a reference for future maintenance or extensions.

## Tasks

### 1. Data Preparation and Validation
- [x] **Task 1.1**: Run `scripts/fetch_common_words.py` to generate word data for English and French.
- [x] **Task 1.2**: Validate the generated word data for accuracy and completeness.
- [x] **Task 1.3**: Normalize word frequencies to the 0.70-1.00 range.
- [x] **Task 1.4**: Ensure exactly 500 words exist per language.
- [x] **Task 1.5**: Verify that words are sorted by descending frequency.

### 2. Data Structures and Word Definition
- [x] **Task 2.1**: Implement the `Word` struct in `src/content/common_word.rs`.
- [x] **Task 2.2**: Define `french_words()` and `english_words()` functions in `src/content/common_word.rs`.
- [x] **Task 2.3**: Ensure the `Word` struct includes `text`, `frequency`, and `length` fields.
- [x] **Task 2.4**: Validate that all words are compatible with the AZERTY layout.

### 3. Common Word Generator
- [x] **Task 3.1**: Implement the `CommonWordGenerator` struct in `src/content/common_word_generator.rs`.
- [x] **Task 3.2**: Add `new()` method to initialize the generator with language-specific words.
- [x] **Task 3.3**: Implement `generate()` method to support drill and sentence modes.
- [x] **Task 3.4**: Add `select_words_for_level()` method to filter words by difficulty level.
- [x] **Task 3.5**: Implement `generate_drill_mode()` and `generate_sentence_mode()` methods.
- [x] **Task 3.6**: Ensure frequency-weighted random selection in sentence mode (70% top 20%, 30% full pool).
- [x] **Task 3.7**: Implement UTF-8 character counting for length constraints.

### 4. Lesson Integration
- [x] **Task 4.1**: Extend the `LessonType` enum in `src/content/lesson.rs` to include `CommonWords`.
- [x] **Task 4.2**: Implement `common_word_lessons()` function in `src/content/lesson.rs` to generate lessons for all levels.
- [x] **Task 4.3**: Add metadata (title, description) for common word lessons.

### 5. Menu Integration
- [x] **Task 5.1**: Add common word lessons to the lesson selection menu in `src/app.rs`.
- [x] **Task 5.2**: Ensure common word lessons are categorized under "French Common Words" and "English Common Words".
- [x] **Task 5.3**: Position common word lessons after trigram lessons in the menu.

### 6. Testing
- [x] **Task 6.1**: Write unit tests for the `Word` struct and data in `src/content/common_word.rs`.
- [x] **Task 6.2**: Write unit tests for the `CommonWordGenerator` in `src/content/common_word_generator.rs`.
- [x] **Task 6.3**: Verify that all words and their frequencies are valid.
- [x] **Task 6.4**: Test drill mode for word repetition.
- [x] **Task 6.5**: Test sentence mode for frequency-weighted random selection.
- [x] **Task 6.6**: Test level selection for correct word counts (50/100/200/500).
- [x] **Task 6.7**: Test UTF-8 character counting for multi-byte characters.

### 7. Documentation
- [x] **Task 7.1**: Update `docs/features/common-words/requirements.md` with feature requirements.
- [x] **Task 7.2**: Update `docs/features/common-words/design.md` with implementation details.
- [x] **Task 7.3**: Ensure all tasks in this file are updated as completed.

### 8. Future Work
- [ ] **Task 8.1**: Add support for custom word lists (user-provided words).
- [ ] **Task 8.2**: Implement word mastery tracking and spaced repetition.
- [ ] **Task 8.3**: Add support for additional languages (e.g., Spanish, German).
- [ ] **Task 8.4**: Extend sentence mode with grammar rules for more natural sentences.