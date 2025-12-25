# Bigram Training - Task Tracking

> **Purpose**: Implementation progress for bigram training feature
> **Module**: `src/content/`
> **Status**: ⏳ PLANNED (Phase 2+)

## Phase 2+: Implementation ✓ COMPLETED

### Data Structures (src/content/bigram.rs) ✓
- [x] Define `Language` enum (French, English)
- [x] Define `BigramType` enum (Natural, Code)
- [x] Define `Bigram` struct with pattern, frequency, examples
- [x] Create French bigram data (10 patterns)
- [x] Create English bigram data (10 patterns)
- [x] Create Code bigram data (10 patterns)
- [x] Implement frequency-based sorting (descending order)

### Content Generation (src/content/bigram_generator.rs) ✓
- [x] Implement `BigramGenerator` struct
- [x] Level 1: Drill mode generation (pure repetition: "qu qu qu")
- [x] Level 2: Word mode generation (contextual words: "que qui quoi")
- [x] Level 3: Mixed mode generation (realistic sentences)
- [x] Implement level-based bigram selection (5/7/10 for levels 1/2/3)
- [x] Ensure deterministic generation for testing

### Lesson Integration (src/content/lesson.rs) ✓
- [x] Extend `LessonType` enum with `Bigram` variant
- [x] Implement `Lesson::bigram_lessons()` unified factory
- [x] French bigram lessons (3 levels)
- [x] English bigram lessons (3 levels)
- [x] Code bigram lessons (3 levels)
- [x] Integrate with ContentGenerator trait

### UI Integration (src/app.rs) ✓
- [x] Add all bigram lessons to app lesson list
- [x] 15 total lessons (6 home row + 9 bigrams)
- [x] Automatic integration with existing menu
- [x] Session saving works with bigram lesson titles

### Testing ✓
- [x] Test frequency ordering validation (5 tests)
- [x] Test drill mode generation
- [x] Test word mode generation
- [x] Test mixed mode generation
- [x] Test deterministic generation
- [x] Test all 3 languages (French, English, Code)
- [x] Test level progression
- [x] 12 new tests added (total: 44 tests, 43 passing)

### Documentation
- [x] Update tasks.md with implementation status
- [ ] Update README with bigram training feature
- [ ] Add examples to CLAUDE.md
- [ ] Update user documentation

## Phase 3: Advanced Features

### Bigram Analytics
- [ ] Track per-bigram accuracy
- [ ] Track per-bigram speed
- [ ] Identify weak bigrams
- [ ] Display bigram-specific statistics

### Custom Bigrams
- [ ] Allow user-defined bigram sets
- [ ] Import from text files
- [ ] Extract from user code samples
- [ ] Save custom lesson configurations

## Implementation Notes

### Priority Order
1. Start with French bigrams (aligned with AZERTY/French focus)
2. Add English bigrams (common request)
3. Add Code bigrams (developer-focused)

### Estimated Complexity
- **Data structures**: Low (similar to existing Lesson structure)
- **Generation logic**: Medium (requires word context)
- **UI integration**: Low (extend existing menu)
- **Overall**: Medium complexity, 2-3 days of work

### Dependencies
- No external dependencies required
- Uses existing content generation patterns
- Builds on current lesson structure

### Testing Strategy
- Unit tests for each generation mode
- Integration tests for menu flow
- Manual testing for content quality
- Validation of bigram frequency data
