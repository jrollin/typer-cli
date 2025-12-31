# Custom Lessons - Implementation Tasks

## Phase 1: Core Implementation ✅ COMPLETED

### Module Creation (src/content/custom.rs)
- [x] Create new file `src/content/custom.rs`
- [x] Define `CustomLessonMetadata` struct
- [x] Define `ParsedMarkdown` struct
- [x] Define `ParseError` enum with Display impl
- [x] Implement `parse_markdown_file(path: &Path) -> Result<ParsedMarkdown, ParseError>`
  - [x] File size check (reject >1MB)
  - [x] UTF-8 validation
  - [x] Front matter extraction (find --- delimiters)
  - [x] YAML parsing (manual key:value parser)
  - [x] Body content extraction
  - [x] Empty content validation
- [x] Implement `scan_directory(dir: &Path) -> Vec<Lesson>`
  - [x] Directory existence check
  - [x] Filter .md files
  - [x] Parse each file with error handling
  - [x] Create Lesson instances
  - [x] Title fallback to filename
- [x] Implement `deduplicate_titles(lessons: &mut [Lesson])`
  - [x] Count title occurrences
  - [x] Append (1), (2), (3) suffix
  - [x] Preserve original order
- [x] Implement `CustomLessonLoader` struct
- [x] Implement `CustomLessonLoader::load_all() -> Vec<Lesson>`
  - [x] Get config directory path (using std::env::var("HOME"))
  - [x] Scan ~/.config/typer-cli/custom/
  - [x] Scan ./custom/
  - [x] Combine results
  - [x] Deduplicate across both directories

### Enum Extension (src/content/lesson.rs)
- [x] Add `Custom { content: String }` variant to `LessonType` enum (line 378)
- [x] Verify pattern matching is exhaustive

### Category Integration (src/content/category.rs)
- [x] Add `Custom` variant to `LessonCategoryType` enum (line 13)
- [x] Add Custom category in `LessonCategory::all()` (lines 62-67)
  - [x] Set name: "Custom"
  - [x] Set description: "User-provided markdown lessons"
  - [x] Set color: Color::Blue
- [x] Add Custom case in `contains_lesson()` (lines 102-104)

### Content Generation (src/content/generator.rs)
- [x] Add `Custom { content }` match arm in ContentGenerator impl (lines 79-83)
- [x] Return content.chars().take(length).collect()

### Module Export (src/content/mod.rs)
- [x] Add `pub mod custom;` (line 9)
- [x] Add `pub use custom::CustomLessonLoader;` (line 20)

### App Integration (src/app.rs)
- [x] Add import for CustomLessonLoader (line 7)
- [x] Call `lessons.extend(CustomLessonLoader::load_all());` (lines 167-168)

### UI Enhancement (src/ui/render.rs)
- [x] Add helpful instruction message when Custom category is empty (lines 665-740)

## Phase 2: Testing ✅ COMPLETED

### Unit Tests (src/content/custom.rs)
- [x] `test_parse_front_matter_full` - Both title and description
- [x] `test_parse_front_matter_title_only` - Title only
- [x] `test_parse_front_matter_missing` - No front matter
- [x] `test_parse_markdown_valid` - Valid markdown file
- [x] `test_parse_markdown_empty_content` - Empty body error
- [x] `test_parse_markdown_preserves_formatting` - Line breaks preserved
- [x] `test_parse_yaml_line_valid` - YAML parsing
- [x] `test_parse_yaml_line_with_colon_in_value` - Colons in values
- [x] `test_parse_yaml_line_invalid` - Invalid YAML lines
- [x] `test_deduplicate_titles_none` - No duplicates
- [x] `test_deduplicate_titles_two` - Two duplicates
- [x] `test_deduplicate_titles_three` - Three duplicates

### Test Results
- [x] All 146 tests passing (11 new custom lesson tests)
- [x] No regressions in existing features
- [x] Clippy clean (no warnings)
- [x] Code formatted with cargo fmt

## Phase 3: Documentation ✅ COMPLETED

### User Documentation (README.md)
- [x] Add Custom Lessons section (lines 181-231)
- [x] Document markdown file format
- [x] Show YAML front matter example
- [x] List both directory locations
- [x] Explain title fallback behavior
- [x] Note deduplication suffix
- [x] Include practical example (git-commands.md)
- [x] Update test count to 146

### Contributor Documentation (docs/README.md)
- [x] Add custom-lessons to Features Overview (lines 213-225)
- [x] Update Phase/Status section to 3.6 (line 300)
- [x] Update Total Tests to 146 (line 302)
- [x] Update Total Lessons count (line 333)
- [x] Add to Completed Features list (line 331)
- [x] Link to feature documentation

### AI Context (CLAUDE.md)
- [x] Add custom-lessons to features table (line 71)
- [x] Update current phase to 3.7 (line 23)
- [x] Note module location in project structure (line 137)
- [x] Update test count to 146 (lines 80, 103)
- [x] Update current status section (lines 171-174)
- [x] Update dead code annotations count to 29 (line 28)
- [x] Update categories count to 6 (line 58)

### Feature Documentation (docs/features/custom-lessons/)
- [x] Create requirements.md with 25 US-X user stories
- [x] Create design.md with technical architecture
- [x] Create tasks.md with implementation checklist

## Phase 4: Polish ✅ COMPLETED

### Code Quality
- [x] Run `cargo fmt` for formatting
- [x] Run `cargo clippy` - no warnings
- [x] Add #[allow(dead_code)] for ParseError::InvalidFrontMatter
- [x] Check error messages are clear and helpful
- [x] Ensure consistent code style with project

### Edge Cases
- [x] File size validation (1MB limit with FileTooLarge error)
- [x] UTF-8 validation (via fs::read_to_string)
- [x] Empty content validation (EmptyContent error)
- [x] Missing directories (silent skip)
- [x] Title deduplication across directories
- [x] Filename fallback for missing title

### Error Messages
- [x] Warning format: "Warning: Failed to load \"{path}\": {error}"
- [x] Prints to stderr (eprintln!)
- [x] File paths shown with path.display()
- [x] Clear error descriptions in ParseError::Display

## Phase 5: Implementation Summary ✅ COMPLETED

### What Was Built
- **Core module**: `src/content/custom.rs` (350+ lines)
  - Manual YAML parser (no external dependencies)
  - File scanning and validation
  - Title deduplication
  - Graceful error handling

- **Integration**:
  - LessonType::Custom variant
  - Custom category (Blue color)
  - Content generator (preserves formatting)
  - Empty state UI with instructions

- **Quality**:
  - 146 tests passing (11 new)
  - Zero clippy warnings
  - Complete documentation
  - No new dependencies

### Files Modified
1. `src/content/custom.rs` - NEW (core implementation)
2. `src/content/lesson.rs` - Added Custom variant
3. `src/content/category.rs` - Added Custom category
4. `src/content/generator.rs` - Added Custom content generation
5. `src/content/mod.rs` - Exported CustomLessonLoader
6. `src/app.rs` - Load custom lessons at startup
7. `src/ui/render.rs` - Empty state instructions
8. `README.md` - User documentation
9. `docs/README.md` - Contributor documentation
10. `CLAUDE.md` - AI context updates
11. `docs/features/custom-lessons/` - Feature documentation (requirements, design, tasks)

### Ready for Next Phase
The custom lessons feature is **complete and production-ready**. Users can now:
- Create markdown files in `~/.config/typer-cli/custom/` or `./custom/`
- Use YAML front matter for metadata (optional)
- Practice multi-line content with preserved formatting
- See helpful instructions when the Custom category is empty
- Track statistics for custom lessons

**Status**: ✅ All tasks completed, tested, and documented.
