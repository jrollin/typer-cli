# Custom Lessons - Design Document

> **Purpose**: Technical design for user-provided markdown lessons
> **Module**: `src/content/custom.rs`
> **Previous Step**: See `requirements.md` for custom lesson requirements
> **Related**: See `../session-storage/design.md` for stats persistence

## Overview

The custom lessons feature allows users to create personalized typing practice content by writing markdown files with YAML front matter. The system scans two directories at startup (`~/.config/typer-cli/custom/` and `./custom/`), parses markdown files, and loads them into a new "Custom" category in the two-level menu system.

**Key Design Principle**: Load all content at startup and store in memory. Custom lessons are expected to be small text files (<1MB), making this approach simpler than lazy loading with minimal memory impact.

## Architecture

### Data Structures

**Metadata from Front Matter:**
```rust
pub struct CustomLessonMetadata {
    pub title: Option<String>,
    pub description: Option<String>,
}
```

**Parsed Markdown:**
```rust
pub struct ParsedMarkdown {
    pub metadata: CustomLessonMetadata,
    pub content: String,  // Body content only (no front matter)
}
```

**Error Types:**
```rust
pub enum ParseError {
    IoError(io::Error),
    InvalidFrontMatter(String),
    EmptyContent,
    FileTooLarge(usize),  // Size in bytes
}

impl Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ParseError::IoError(e) => write!(f, "I/O error: {}", e),
            ParseError::InvalidFrontMatter(msg) => write!(f, "Invalid YAML: {}", msg),
            ParseError::EmptyContent => write!(f, "Content body is empty"),
            ParseError::FileTooLarge(size) => write!(f, "File too large: {} bytes (max 1MB)", size),
        }
    }
}
```

### Core Functions

**1. File Parsing**
```rust
fn parse_markdown_file(path: &Path) -> Result<ParsedMarkdown, ParseError>
```
- Read file to string (UTF-8 validation)
- Check file size (reject >1MB)
- Extract YAML front matter between `---` delimiters
- Parse `title: value` and `description: value` (simple key:value parser)
- Extract body content after second `---`
- Validate content is non-empty
- Return ParsedMarkdown or ParseError

**2. Directory Scanning**
```rust
fn scan_directory(dir: &Path) -> Vec<Lesson>
```
- Check if directory exists (return empty Vec if not)
- Read directory entries
- Filter for `.md` extension only
- Parse each file with parse_markdown_file()
- On parse error: print warning to stderr, skip file
- Create Lesson instance with LessonType::Custom { content }
- Use metadata.title or filename (without .md) as lesson.title
- Use metadata.description or empty string as lesson.description
- Return Vec of successfully loaded lessons

**3. Title Deduplication**
```rust
fn deduplicate_titles(lessons: &mut Vec<Lesson>)
```
- Count occurrences of each title
- For duplicates, append (1), (2), (3) suffix
- Preserve original order

**4. Main Loader**
```rust
pub struct CustomLessonLoader;

impl CustomLessonLoader {
    pub fn load_all() -> Vec<Lesson>
}
```
- Get config directory path: `~/.config/typer-cli/custom/`
- Scan config directory
- Get current directory path: `./custom/`
- Scan current directory
- Combine results
- Deduplicate titles across both sources
- Return combined Vec

### Integration Points

**LessonType Extension** (`src/content/lesson.rs`):
```rust
pub enum LessonType {
    // ... existing variants ...
    FingerPair { finger_pair: FingerPairType, level: u8, with_shift: bool },
    Custom {
        content: String,  // Full markdown body
    },
}
```

**Category Extension** (`src/content/category.rs`):
```rust
pub enum LessonCategoryType {
    Adaptive,
    FingerTraining,
    KeyTraining,
    Languages,
    Code,
    Custom,  // NEW
}

// In LessonCategory::all()
Self {
    category_type: LessonCategoryType::Custom,
    name: "Custom",
    description: "User-provided markdown lessons",
    color: Color::Blue,
}

// In contains_lesson()
LessonCategoryType::Custom => {
    matches!(lesson.lesson_type, LessonType::Custom { .. })
}
```

**Content Generation** (`src/content/generator.rs`):
```rust
// In ContentGenerator impl
LessonType::Custom { content } => {
    // Return content as-is, truncated to requested length
    // Preserves formatting: line breaks, spacing, indentation
    content.chars().take(length).collect()
}
```

**App Integration** (`src/app.rs`):
```rust
// After loading all other lessons (around line 165)
lessons.extend(CustomLessonLoader::load_all());
```

### Data Flow

```
Startup
  ├─→ CustomLessonLoader::load_all()
      ├─→ scan_directory(~/.config/typer-cli/custom/)
      │   ├─→ for each .md file:
      │   │   ├─→ parse_markdown_file()
      │   │   │   ├─→ extract front matter
      │   │   │   └─→ extract body content
      │   │   └─→ create Lesson { LessonType::Custom { content } }
      │   └─→ return Vec<Lesson>
      ├─→ scan_directory(./custom/)
      │   └─→ (same as above)
      ├─→ combine results
      ├─→ deduplicate_titles()
      └─→ return Vec<Lesson>
  └─→ lessons added to App.lessons

Category Selection
  └─→ Filter lessons by Custom category
      └─→ Display lesson titles

Lesson Start
  └─→ ContentGenerator::generate(lesson, length)
      └─→ Return content.chars().take(length)

Session Complete
  └─→ Save stats with lesson.title as identifier
```

## YAML Front Matter Parser

**Design Decision**: Manual parser, no external dependencies.

**Format**:
```markdown
---
title: My Custom Lesson
description: Practice domain-specific vocabulary
---

Actual content to practice typing goes here.
Line breaks and spacing are preserved.
```

**Parsing Algorithm**:
1. Split content by newlines
2. Check if first line is `---`
3. Collect lines until second `---`
4. For each line in front matter:
   - Split on first `:` character
   - Trim whitespace
   - Store key-value pair
5. Extract body content after second `---`
6. Trim leading/trailing whitespace from body

**Error Handling**:
- Missing second `---`: Treat as no front matter
- Invalid key:value format: Skip line
- Empty body: Return ParseError::EmptyContent
- UTF-8 decode error: Return ParseError::IoError

## Error Handling Strategy

**Philosophy**: Non-blocking, graceful degradation

**Warning Messages** (stderr, before TUI starts):
```
Warning: Failed to load "/home/user/.config/typer-cli/custom/bad.md": Invalid YAML: missing value for key 'title'
Warning: Failed to load "/home/user/custom/empty.md": Content body is empty
Warning: Failed to load "/home/user/custom/huge.md": File too large: 2097152 bytes (max 1MB)
```

**Graceful Handling**:
- Missing directories → Silent skip (not an error condition)
- Invalid files → Print warning, skip file, continue loading
- No .md files found → Custom category appears but is empty
- Parse errors → Warning message with specific details
- I/O errors → Warning message with error context

## Technical Decisions

### Why Load at Startup (Not Lazy Loading)?
- **Simplicity**: All lessons loaded once, no async complexity
- **Memory**: Custom lessons are small (<1MB), minimal impact
- **Performance**: Single I/O burst at startup, fast access during session
- **Consistency**: All lesson types follow same loading pattern

### Why Manual YAML Parser (No Dependencies)?
- **Simplicity**: Only need 2 fields (title, description)
- **Size**: Avoid adding serde_yaml dependency
- **Control**: Exact error messages, custom validation
- **Compatibility**: No version conflicts with existing serde usage

### Why Store Content in LessonType::Custom?
- **Consistency**: Follows existing pattern (lessons are self-contained)
- **Simplicity**: No separate content lookup needed
- **Performance**: Direct access during content generation
- **Alternative considered**: Separate HashMap<String, String> rejected as more complex

### Why Deduplicate with Suffix?
- **Transparency**: User sees both lessons exist
- **Safety**: Doesn't silently drop user content
- **Simplicity**: No need for user intervention
- **Alternative considered**: Skip duplicates rejected as data loss

### Why filename as fallback?
- **Convenience**: Quick lesson creation without front matter
- **Clarity**: Filename is visible in file manager
- **Convention**: Matches common markdown practice

### Why Two Directories?
- **Config directory**: Persistent, user-specific lessons
- **Current directory**: Project-specific, version-controlled lessons
- **Flexibility**: Users can choose what to share/version
- **Precedence**: Both loaded equally, deduplication handles conflicts

## Edge Cases

| Case | Behavior |
|------|----------|
| Very large file (>1MB) | Skip with warning |
| Binary file with .md extension | UTF-8 validation fails → IoError warning |
| Symbolic links | Followed by OS (fs::read_to_string) |
| Permission denied | IoError warning, skip file |
| Front matter without closing `---` | Treat as no front matter, use full content |
| Duplicate filenames in different dirs | Both loaded, deduplicated by title |
| Empty directory | Custom category shows "No lessons available" |
| Unicode in title/content | Fully supported (Rust strings are UTF-8) |
| Multiple `---` in content | Only first pair used for front matter |

## Testing Strategy

### Unit Tests

**File Parsing**:
- `test_parse_front_matter_full` - Both title and description
- `test_parse_front_matter_title_only` - Title only
- `test_parse_front_matter_description_only` - Description only
- `test_parse_front_matter_missing` - No front matter
- `test_parse_markdown_valid` - Complete markdown file
- `test_parse_markdown_empty_content` - Should return EmptyContent error
- `test_parse_markdown_no_closing_delimiter` - Treat as no front matter
- `test_parse_file_too_large` - Should return FileTooLarge error

**Directory Scanning**:
- `test_scan_directory_empty` - Empty directory returns empty Vec
- `test_scan_directory_missing` - Non-existent directory returns empty Vec
- `test_scan_directory_mixed_files` - Only .md files loaded
- `test_scan_directory_multiple` - Multiple valid files

**Deduplication**:
- `test_deduplicate_titles_none` - No duplicates
- `test_deduplicate_titles_two` - Two duplicates get (1), (2)
- `test_deduplicate_titles_three` - Three duplicates get (1), (2), (3)
- `test_deduplicate_preserves_order` - Original order maintained

**Loader**:
- `test_load_all_combines_directories` - Both directories loaded
- `test_load_all_deduplicates_across_dirs` - Cross-directory deduplication

### Integration Tests

**Manual Testing Checklist**:
1. Create `~/.config/typer-cli/custom/test1.md` with full front matter
2. Create `./custom/test2.md` with no front matter (filename fallback)
3. Create duplicate titles in both directories, verify suffix
4. Navigate to Custom category, verify both lessons appear
5. Start custom lesson, verify content preserves line breaks
6. Complete session, verify stats saved with correct title
7. Create invalid markdown (empty content), verify warning
8. Create file >1MB, verify warning and skip
9. Test with missing directories, verify no errors
10. Test Unicode content and titles

## Dependencies

**No new external dependencies required**:
- `std::fs` - File I/O
- `std::io` - Error handling
- `std::path::{Path, PathBuf}` - Path manipulation
- `std::collections::HashMap` - Deduplication counting
- `dirs` - Already in project (config directory path)

**Existing modules**:
- `src/content/lesson.rs` - LessonType enum extension
- `src/content/category.rs` - Category system integration
- `src/content/generator.rs` - ContentGenerator implementation
- `src/app.rs` - Lesson loading orchestration

## File Size Limits

**Rationale for 1MB limit**:
- Typical custom lesson: 100-500 words = 500-2500 bytes
- Safety margin: 1MB = 2000x typical size
- Memory impact: 100 lessons @ 1MB each = 100MB (acceptable)
- Protection: Prevents accidental loading of large files

## Future Enhancements (Out of Scope)

- Hot-reload of custom lessons (filesystem watching)
- Markdown rendering/preview before practice
- Custom lesson templates or creation wizard
- Import lessons from URLs or GitHub gists
- Lesson validation (spell check, difficulty rating)
- Custom lesson metadata (author, tags, difficulty)
- Sharing/exporting custom lessons
