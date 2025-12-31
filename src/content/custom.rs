use std::collections::HashMap;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};

use super::lesson::Lesson;
use super::lesson::LessonType;

const MAX_FILE_SIZE: usize = 1_048_576; // 1MB

/// Metadata extracted from YAML front matter
#[derive(Debug, Clone, Default)]
pub struct CustomLessonMetadata {
    pub title: Option<String>,
    pub description: Option<String>,
}

/// Parsed markdown file with metadata and content
#[derive(Debug, Clone)]
pub struct ParsedMarkdown {
    pub metadata: CustomLessonMetadata,
    pub content: String,
}

/// Errors that can occur when parsing custom lesson files
#[derive(Debug)]
pub enum ParseError {
    IoError(io::Error),
    /// Public API: Reserved for future strict YAML validation
    #[allow(dead_code)]
    InvalidFrontMatter(String),
    EmptyContent,
    FileTooLarge(usize),
}

impl std::fmt::Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            ParseError::IoError(e) => write!(f, "I/O error: {}", e),
            ParseError::InvalidFrontMatter(msg) => write!(f, "Invalid YAML: {}", msg),
            ParseError::EmptyContent => write!(f, "Content body is empty"),
            ParseError::FileTooLarge(size) => {
                write!(f, "File too large: {} bytes (max 1MB)", size)
            }
        }
    }
}

impl From<io::Error> for ParseError {
    fn from(err: io::Error) -> Self {
        ParseError::IoError(err)
    }
}

/// Parse a markdown file with optional YAML front matter
///
/// Expected format:
/// ```markdown
/// ---
/// title: My Lesson
/// description: Practice custom content
/// ---
///
/// Actual content to practice goes here.
/// ```
fn parse_markdown_file(path: &Path) -> Result<ParsedMarkdown, ParseError> {
    // Check file size first
    let metadata = fs::metadata(path)?;
    let size = metadata.len() as usize;
    if size > MAX_FILE_SIZE {
        return Err(ParseError::FileTooLarge(size));
    }

    // Read file content
    let content = fs::read_to_string(path)?;
    let lines: Vec<&str> = content.lines().collect();

    // Check for front matter
    if lines.is_empty() {
        return Err(ParseError::EmptyContent);
    }

    let mut lesson_metadata = CustomLessonMetadata::default();
    let body_content: String;

    // Check if file starts with front matter delimiter
    if lines.first() == Some(&"---") {
        // Find the closing delimiter
        if let Some(end_index) = lines.iter().skip(1).position(|&line| line == "---") {
            // Parse front matter (between first and second ---)
            for line in &lines[1..end_index + 1] {
                if let Some((key, value)) = parse_yaml_line(line) {
                    match key.as_str() {
                        "title" => lesson_metadata.title = Some(value),
                        "description" => lesson_metadata.description = Some(value),
                        _ => {} // Ignore unknown keys
                    }
                }
            }

            // Extract body content after second ---
            let body_lines = &lines[end_index + 2..];
            body_content = body_lines.join("\n").trim().to_string();
        } else {
            // No closing delimiter, treat entire content as body
            body_content = content.trim().to_string();
        }
    } else {
        // No front matter, entire content is body
        body_content = content.trim().to_string();
    }

    // Validate body content is not empty
    if body_content.is_empty() {
        return Err(ParseError::EmptyContent);
    }

    Ok(ParsedMarkdown {
        metadata: lesson_metadata,
        content: body_content,
    })
}

/// Parse a single YAML line in "key: value" format
fn parse_yaml_line(line: &str) -> Option<(String, String)> {
    let parts: Vec<&str> = line.splitn(2, ':').collect();
    if parts.len() == 2 {
        let key = parts[0].trim().to_string();
        let value = parts[1].trim().to_string();
        if !key.is_empty() && !value.is_empty() {
            return Some((key, value));
        }
    }
    None
}

/// Scan a directory for markdown files and convert them to Lessons
fn scan_directory(dir: &Path) -> Vec<Lesson> {
    // Return empty vec if directory doesn't exist
    if !dir.exists() {
        return Vec::new();
    }

    let mut lessons = Vec::new();

    // Read directory entries
    let entries = match fs::read_dir(dir) {
        Ok(entries) => entries,
        Err(_) => return Vec::new(),
    };

    for entry in entries.flatten() {
        let path = entry.path();

        // Only process .md files
        if path.extension().and_then(|s| s.to_str()) != Some("md") {
            continue;
        }

        // Parse the markdown file
        match parse_markdown_file(&path) {
            Ok(parsed) => {
                // Use title from metadata or filename (without extension)
                let title = parsed.metadata.title.unwrap_or_else(|| {
                    path.file_stem()
                        .and_then(|s| s.to_str())
                        .unwrap_or("Untitled")
                        .to_string()
                });

                let description = parsed.metadata.description.unwrap_or_default();

                // Create lesson with custom content
                let lesson = Lesson {
                    title,
                    description,
                    keys: Vec::new(), // Not applicable for custom lessons
                    lesson_type: LessonType::Custom {
                        content: parsed.content,
                    },
                };

                lessons.push(lesson);
            }
            Err(e) => {
                // Print warning to stderr and continue
                eprintln!("Warning: Failed to load \"{}\": {}", path.display(), e);
            }
        }
    }

    lessons
}

/// Deduplicate lesson titles by appending (1), (2), (3) suffixes
fn deduplicate_titles(lessons: &mut [Lesson]) {
    // Count occurrences of each title
    let mut title_counts: HashMap<String, usize> = HashMap::new();
    for lesson in lessons.iter() {
        *title_counts.entry(lesson.title.clone()).or_insert(0) += 1;
    }

    // Track how many times we've seen each duplicate title
    let mut seen_counts: HashMap<String, usize> = HashMap::new();

    // Add suffixes to duplicates
    for lesson in lessons.iter_mut() {
        let count = title_counts.get(&lesson.title).copied().unwrap_or(1);

        if count > 1 {
            let seen = seen_counts.entry(lesson.title.clone()).or_insert(0);
            *seen += 1;
            lesson.title = format!("{} ({})", lesson.title, seen);
        }
    }
}

/// Loader for custom user-provided lessons
pub struct CustomLessonLoader;

impl CustomLessonLoader {
    /// Load all custom lessons from both config and current directory
    pub fn load_all() -> Vec<Lesson> {
        let mut lessons = Vec::new();

        // Load from config directory: ~/.config/typer-cli/custom/
        if let Ok(home) = std::env::var("HOME") {
            let custom_dir = PathBuf::from(home)
                .join(".config")
                .join("typer-cli")
                .join("custom");
            lessons.extend(scan_directory(&custom_dir));
        }

        // Load from current directory: ./custom/
        let current_dir = PathBuf::from("./custom");
        lessons.extend(scan_directory(&current_dir));

        // Deduplicate titles across both sources
        deduplicate_titles(&mut lessons);

        lessons
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_yaml_line_valid() {
        assert_eq!(
            parse_yaml_line("title: My Lesson"),
            Some(("title".to_string(), "My Lesson".to_string()))
        );

        assert_eq!(
            parse_yaml_line("description: Practice typing"),
            Some(("description".to_string(), "Practice typing".to_string()))
        );
    }

    #[test]
    fn test_parse_yaml_line_with_colon_in_value() {
        assert_eq!(
            parse_yaml_line("title: Lesson: Advanced"),
            Some(("title".to_string(), "Lesson: Advanced".to_string()))
        );
    }

    #[test]
    fn test_parse_yaml_line_invalid() {
        assert_eq!(parse_yaml_line("invalid line"), None);
        assert_eq!(parse_yaml_line("key:"), None);
        assert_eq!(parse_yaml_line(":value"), None);
        assert_eq!(parse_yaml_line(""), None);
    }

    #[test]
    fn test_parse_front_matter_full() {
        let content = "---\ntitle: Test Lesson\ndescription: Test description\n---\n\nContent here";
        let temp_file = std::env::temp_dir().join("test_full.md");
        fs::write(&temp_file, content).unwrap();

        let result = parse_markdown_file(&temp_file).unwrap();
        assert_eq!(result.metadata.title, Some("Test Lesson".to_string()));
        assert_eq!(
            result.metadata.description,
            Some("Test description".to_string())
        );
        assert_eq!(result.content, "Content here");

        fs::remove_file(temp_file).unwrap();
    }

    #[test]
    fn test_parse_front_matter_title_only() {
        let content = "---\ntitle: Test Lesson\n---\n\nContent here";
        let temp_file = std::env::temp_dir().join("test_title.md");
        fs::write(&temp_file, content).unwrap();

        let result = parse_markdown_file(&temp_file).unwrap();
        assert_eq!(result.metadata.title, Some("Test Lesson".to_string()));
        assert_eq!(result.metadata.description, None);
        assert_eq!(result.content, "Content here");

        fs::remove_file(temp_file).unwrap();
    }

    #[test]
    fn test_parse_front_matter_missing() {
        let content = "Just content without front matter";
        let temp_file = std::env::temp_dir().join("test_no_front.md");
        fs::write(&temp_file, content).unwrap();

        let result = parse_markdown_file(&temp_file).unwrap();
        assert_eq!(result.metadata.title, None);
        assert_eq!(result.metadata.description, None);
        assert_eq!(result.content, "Just content without front matter");

        fs::remove_file(temp_file).unwrap();
    }

    #[test]
    fn test_parse_markdown_empty_content() {
        let content = "---\ntitle: Test\n---\n\n";
        let temp_file = std::env::temp_dir().join("test_empty.md");
        fs::write(&temp_file, content).unwrap();

        let result = parse_markdown_file(&temp_file);
        assert!(matches!(result, Err(ParseError::EmptyContent)));

        fs::remove_file(temp_file).unwrap();
    }

    #[test]
    fn test_parse_markdown_preserves_formatting() {
        let content = "---\ntitle: Test\n---\n\nLine 1\n  Indented line\nLine 3";
        let temp_file = std::env::temp_dir().join("test_format.md");
        fs::write(&temp_file, content).unwrap();

        let result = parse_markdown_file(&temp_file).unwrap();
        assert_eq!(result.content, "Line 1\n  Indented line\nLine 3");

        fs::remove_file(temp_file).unwrap();
    }

    #[test]
    fn test_deduplicate_titles_none() {
        let mut lessons = vec![
            Lesson {
                title: "Lesson 1".to_string(),
                description: "".to_string(),
                keys: vec![],
                lesson_type: LessonType::Custom {
                    content: "content".to_string(),
                },
            },
            Lesson {
                title: "Lesson 2".to_string(),
                description: "".to_string(),
                keys: vec![],
                lesson_type: LessonType::Custom {
                    content: "content".to_string(),
                },
            },
        ];

        deduplicate_titles(&mut lessons);

        assert_eq!(lessons[0].title, "Lesson 1");
        assert_eq!(lessons[1].title, "Lesson 2");
    }

    #[test]
    fn test_deduplicate_titles_two() {
        let mut lessons = vec![
            Lesson {
                title: "Same".to_string(),
                description: "".to_string(),
                keys: vec![],
                lesson_type: LessonType::Custom {
                    content: "content1".to_string(),
                },
            },
            Lesson {
                title: "Same".to_string(),
                description: "".to_string(),
                keys: vec![],
                lesson_type: LessonType::Custom {
                    content: "content2".to_string(),
                },
            },
        ];

        deduplicate_titles(&mut lessons);

        assert_eq!(lessons[0].title, "Same (1)");
        assert_eq!(lessons[1].title, "Same (2)");
    }

    #[test]
    fn test_deduplicate_titles_three() {
        let mut lessons = vec![
            Lesson {
                title: "Duplicate".to_string(),
                description: "".to_string(),
                keys: vec![],
                lesson_type: LessonType::Custom {
                    content: "content1".to_string(),
                },
            },
            Lesson {
                title: "Duplicate".to_string(),
                description: "".to_string(),
                keys: vec![],
                lesson_type: LessonType::Custom {
                    content: "content2".to_string(),
                },
            },
            Lesson {
                title: "Duplicate".to_string(),
                description: "".to_string(),
                keys: vec![],
                lesson_type: LessonType::Custom {
                    content: "content3".to_string(),
                },
            },
        ];

        deduplicate_titles(&mut lessons);

        assert_eq!(lessons[0].title, "Duplicate (1)");
        assert_eq!(lessons[1].title, "Duplicate (2)");
        assert_eq!(lessons[2].title, "Duplicate (3)");
    }
}
