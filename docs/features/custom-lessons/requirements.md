# Custom Lessons - Requirements

## Purpose
Allow users to create and practice custom typing content by providing markdown files with YAML front matter, enabling personalized training for domain-specific vocabulary, code patterns, or any custom text.

## User Stories

### US-1: Load Custom Lessons from Config Directory
WHEN the application starts
THE SYSTEM SHALL scan `~/.config/typer-cli/custom/` for markdown files with `.md` extension

### US-2: Load Custom Lessons from Current Directory
WHEN the application starts
THE SYSTEM SHALL scan `./custom/` (current working directory) for markdown files with `.md` extension

### US-3: Handle Missing Directories Gracefully
WHEN a custom lesson directory does not exist
THE SYSTEM SHALL silently skip it without raising an error

### US-4: Filter Non-Markdown Files
WHEN scanning custom lesson directories
THE SYSTEM SHALL ignore files without `.md` extension

### US-5: Handle Invalid Files Gracefully
WHEN a markdown file cannot be read or parsed
THE SYSTEM SHALL print a warning message to stderr and continue loading other files

### US-6: Parse YAML Front Matter
WHEN a markdown file contains YAML front matter delimited by `---`
THE SYSTEM SHALL extract `title` and `description` fields

### US-7: Fallback to Filename for Missing Title
WHEN a markdown file has no front matter or missing `title` field
THE SYSTEM SHALL use the filename (without `.md` extension) as the lesson title

### US-8: Handle Malformed Front Matter
WHEN the front matter is malformed
THE SYSTEM SHALL treat it as if no front matter exists and use filename fallback

### US-9: Validate Content Body
WHEN a markdown file has empty content body (after front matter)
THE SYSTEM SHALL skip the file with a warning message

### US-10: Preserve Text Formatting
WHEN generating practice content from a custom lesson
THE SYSTEM SHALL preserve all formatting including line breaks, spacing, and indentation

### US-11: Truncate Long Content
WHEN the requested practice length exceeds available content
THE SYSTEM SHALL truncate to available content length

### US-12: Store Content in Memory
WHEN loading custom lessons at startup
THE SYSTEM SHALL store the full markdown content body (excluding front matter) in memory

### US-13: Deduplicate Lesson Titles
WHEN multiple lessons have the same title
THE SYSTEM SHALL append `(1)`, `(2)`, `(3)` suffix to duplicates

### US-14: Deduplicate Across Directories
WHEN determining duplicate titles
THE SYSTEM SHALL consider lessons from both directories together

### US-15: Display Custom Category
WHEN at least one custom lesson exists
THE SYSTEM SHALL display a "Custom" category in the main menu

### US-16: Show Empty Custom Category
WHEN no custom lessons exist
THE SYSTEM SHALL still display the "Custom" category but show it as empty

### US-17: List Custom Lessons by Title
WHEN the Custom category menu is displayed
THE SYSTEM SHALL list all successfully loaded custom lessons by title

### US-18: Start Custom Lesson Session
WHEN a user selects a custom lesson
THE SYSTEM SHALL start a typing session with the lesson's content

### US-19: Track Statistics by Title
WHEN recording session statistics for a custom lesson
THE SYSTEM SHALL use the lesson's title as the identifier

### US-20: Follow Existing Storage Format
WHEN saving statistics for a custom lesson
THE SYSTEM SHALL follow the existing session storage format

### US-21: Fast Directory Scanning
WHEN scanning custom lesson directories
THE SYSTEM SHALL complete within 500ms for up to 100 files

### US-22: Limit File Size
WHEN a custom lesson file exceeds 1MB
THE SYSTEM SHALL skip it with a warning message

### US-23: Clear Error Messages
WHEN displaying error messages for invalid custom lessons
THE SYSTEM SHALL provide clear file paths and specific error details

### US-24: Reload on Restart
WHEN a user adds new markdown files to custom directories
THE SYSTEM SHALL load them on next application restart (no hot-reload required)

### US-25: No External Dependencies
WHEN implementing custom lessons
THE SYSTEM SHALL not introduce new external dependencies beyond Rust standard library

## Acceptance Criteria

- [ ] User can create `~/.config/typer-cli/custom/test.md` and see it in Custom category
- [ ] User can create `./custom/test.md` and see it in Custom category
- [ ] Front matter with `title: My Lesson` displays "My Lesson" in menu
- [ ] Missing front matter displays filename (without .md) in menu
- [ ] Line breaks and spacing are preserved during typing practice
- [ ] Duplicate titles get (1), (2) suffix automatically
- [ ] Invalid markdown files show warnings but don't crash application
- [ ] Statistics are saved with lesson title as identifier
- [ ] Custom category appears in menu even when empty
- [ ] No new external dependencies added (uses std library only)
- [ ] Directory scanning completes quickly (<500ms for 100 files)
- [ ] Files over 1MB are skipped with warning
