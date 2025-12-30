use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, Paragraph},
    Frame,
};

use crate::content::Lesson;
use crate::data::Stats;
use crate::engine::analytics::{AdaptiveAnalytics, MasteryLevel};
use crate::engine::TypingSession;
use crate::keyboard::AzertyLayout;
use crate::ui::keyboard::{render_keyboard, render_keyboard_compact, KeyboardConfig};
use std::collections::HashMap;

/// Structure for visible text window
struct VisibleWindow {
    lines: Vec<String>,
    /// Cursor line within visible window (always 0 for first line)
    #[allow(dead_code)]
    cursor_line: usize,
    /// Cursor offset within the cursor line
    #[allow(dead_code)]
    cursor_offset: usize,
    /// Which wrapped line number the window starts at in full content
    #[allow(dead_code)]
    window_start_line: usize,
    /// Cumulative character count at start of each visible line (for index translation)
    line_start_indices: Vec<usize>,
}

/// Wrap text to fit terminal width using word boundaries
fn wrap_text(content: &str, width: usize) -> Vec<String> {
    let mut lines = Vec::new();
    let mut current_line = String::new();

    for word in content.split_whitespace() {
        if current_line.is_empty() {
            current_line = word.to_string();
        } else if current_line.len() + 1 + word.len() <= width {
            current_line.push(' ');
            current_line.push_str(word);
        } else {
            lines.push(current_line);
            current_line = word.to_string();
        }
    }

    if !current_line.is_empty() {
        lines.push(current_line);
    }

    lines
}

/// Find which wrapped line contains a given character position
fn find_cursor_line(lines: &[String], char_pos: usize) -> (usize, usize) {
    let mut char_count = 0;

    for (line_idx, line) in lines.iter().enumerate() {
        let line_len = line.chars().count();
        if char_pos < char_count + line_len {
            return (line_idx, char_pos.saturating_sub(char_count));
        }
        char_count += line_len + 1; // +1 for space between words
    }

    // If not found, return last line
    (lines.len().saturating_sub(1), 0)
}

/// Extract 3-line visible window starting from cursor position
fn extract_visible_window(session: &TypingSession, width: usize) -> VisibleWindow {
    let content = &session.content;
    let cursor_pos = session.current_index;

    // Calculate effective width (subtract borders and padding)
    let effective_width = width.saturating_sub(4);

    // Wrap text to terminal width
    let lines = wrap_text(content, effective_width);

    // Find which line contains the cursor
    let (cursor_line_idx, cursor_offset_in_line) = find_cursor_line(&lines, cursor_pos);

    // Extract 3 lines starting from cursor line
    let visible_lines: Vec<String> = lines
        .iter()
        .skip(cursor_line_idx)
        .take(3)
        .cloned()
        .collect();

    // Compute cumulative character indices for visible lines
    let mut line_start_indices = Vec::new();
    for idx in cursor_line_idx..(cursor_line_idx + visible_lines.len()) {
        // Calculate chars from start of content to this line
        let chars_before_line: usize = lines
            .iter()
            .take(idx)
            .map(|l| l.chars().count() + 1) // +1 for space between words
            .sum();
        line_start_indices.push(chars_before_line);
    }

    VisibleWindow {
        lines: visible_lines,
        cursor_line: 0, // Cursor is always on first visible line
        cursor_offset: cursor_offset_in_line,
        window_start_line: cursor_line_idx,
        line_start_indices,
    }
}

/// Create styled expected text with character-level visual feedback
/// - Correctly typed characters: dark gray (dimmed)
/// - Mistyped characters: red (error indication)
/// - Next character to type: white + bold + underlined
/// - Remaining characters: white
fn create_styled_expected_text(
    session: &TypingSession,
    window: &VisibleWindow,
) -> Vec<Line<'static>> {
    let mut result_lines = Vec::new();

    for (line_idx, line) in window.lines.iter().enumerate() {
        let mut spans = Vec::new();
        let line_start_index = window.line_start_indices[line_idx];

        for (char_offset, ch) in line.chars().enumerate() {
            let absolute_index = line_start_index + char_offset;

            let style = if absolute_index < session.current_index {
                // Already typed - check if correct or incorrect
                if absolute_index < session.inputs.len() {
                    let input = &session.inputs[absolute_index];
                    if input.is_correct {
                        // Correct - dim (dark gray)
                        Style::default().fg(Color::DarkGray)
                    } else {
                        // Incorrect - red to show error
                        Style::default().fg(Color::Red)
                    }
                } else {
                    // Fallback (shouldn't happen)
                    Style::default().fg(Color::DarkGray)
                }
            } else if absolute_index == session.current_index {
                // Next character - highlight
                Style::default()
                    .fg(Color::White)
                    .add_modifier(Modifier::BOLD | Modifier::UNDERLINED)
            } else {
                // Remaining - normal
                Style::default().fg(Color::White)
            };

            spans.push(Span::styled(ch.to_string(), style));
        }

        result_lines.push(Line::from(spans));
    }

    result_lines
}

/// Rendu de l'interface principale
#[allow(clippy::too_many_arguments)]
pub fn render(
    f: &mut Frame,
    session: &TypingSession,
    wpm: f64,
    accuracy: f64,
    keyboard_visible: bool,
    keyboard_layout: &AzertyLayout,
    analytics: &Option<AdaptiveAnalytics>,
    keyboard_config: &KeyboardConfig,
    lesson_name: &str,
) {
    let terminal_height = f.area().height;

    // Dynamic constraints based on keyboard visibility and terminal size
    // New layout: Header -> Stats -> Content -> Keyboard -> Spacer -> Instructions
    let constraints = if keyboard_visible {
        if terminal_height >= 28 {
            // Full keyboard with shift indicators
            vec![
                Constraint::Length(3),  // Header
                Constraint::Length(3),  // Stats (moved after header)
                Constraint::Length(14), // Content (7 + 7 lines with padding)
                Constraint::Length(12), // Keyboard (follows content)
                Constraint::Min(0),     // Spacer (absorbs remaining space)
                Constraint::Length(3),  // Instructions (bottom)
            ]
        } else if terminal_height >= 23 {
            // Full keyboard without shift line
            vec![
                Constraint::Length(3),  // Header
                Constraint::Length(3),  // Stats (moved after header)
                Constraint::Length(14), // Content (7 + 7 lines with padding)
                Constraint::Length(10), // Keyboard (follows content)
                Constraint::Min(0),     // Spacer
                Constraint::Length(3),  // Instructions (bottom)
            ]
        } else {
            // Compact keyboard
            vec![
                Constraint::Length(3),  // Header
                Constraint::Length(3),  // Stats (moved after header)
                Constraint::Length(14), // Content (7 + 7 lines with padding)
                Constraint::Length(3),  // Keyboard (compact)
                Constraint::Min(0),     // Spacer
                Constraint::Length(3),  // Instructions (bottom)
            ]
        }
    } else {
        // Layout without keyboard
        vec![
            Constraint::Length(3), // Header
            Constraint::Length(3), // Stats (moved after header)
            Constraint::Min(8),    // Content (can expand when no keyboard)
            Constraint::Length(3), // Instructions (bottom)
        ]
    };

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(2)
        .constraints(constraints)
        .split(f.area());

    let mut chunk_idx = 0;

    // Header
    render_header(f, chunks[chunk_idx], lesson_name);
    chunk_idx += 1;

    // Stats (moved after header)
    render_stats(
        f,
        chunks[chunk_idx],
        wpm,
        accuracy,
        session.remaining_time(),
    );
    chunk_idx += 1;

    // Content area (typing area)
    render_typing_area(f, chunks[chunk_idx], session);
    chunk_idx += 1;

    // Keyboard (follows content with margin)
    if keyboard_visible {
        let next_char = session.content.chars().nth(session.current_index);

        if terminal_height < 20 {
            render_keyboard_compact(f, chunks[chunk_idx], keyboard_layout, next_char);
        } else {
            render_keyboard(
                f,
                chunks[chunk_idx],
                keyboard_layout,
                next_char,
                analytics,
                keyboard_config,
            );
        }
        chunk_idx += 1; // Move to spacer
        chunk_idx += 1; // Move to instructions
    }
    // When keyboard is not visible, chunk_idx is already at instructions position

    // Instructions (bottom)
    render_instructions(f, chunks[chunk_idx]);
}

/// Rendu du header
fn render_header(f: &mut Frame, area: Rect, lesson_name: &str) {
    let title_text = format!("TYPER CLI - {}", lesson_name);
    let title = Paragraph::new(title_text)
        .style(
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        )
        .alignment(Alignment::Center)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .padding(ratatui::widgets::Padding::horizontal(1)),
        );

    f.render_widget(title, area);
}

/// Create multiline colored input display
fn create_colored_input_multiline(session: &TypingSession, width: usize) -> Vec<Line<'static>> {
    let effective_width = width.saturating_sub(4);
    let mut lines = Vec::new();
    let mut current_line_spans = Vec::new();
    let mut current_line_width = 0;

    for input in session.inputs.iter() {
        let color = if input.is_correct {
            Color::Green
        } else {
            Color::Red
        };
        let display_char = if input.typed == ' ' {
            '·'
        } else {
            input.typed
        };

        // Check if adding this character would exceed line width
        if current_line_width >= effective_width {
            lines.push(Line::from(current_line_spans.clone()));
            current_line_spans.clear();
            current_line_width = 0;
        }

        current_line_spans.push(Span::styled(
            display_char.to_string(),
            Style::default().fg(color),
        ));
        current_line_width += 1;
    }

    // Add cursor to current line
    if !session.is_complete() {
        current_line_spans.push(Span::styled(
            "█",
            Style::default()
                .fg(Color::White)
                .add_modifier(Modifier::SLOW_BLINK),
        ));
    }

    // Push final line
    if !current_line_spans.is_empty() {
        lines.push(Line::from(current_line_spans));
    }

    // Return only the first 3 lines (sliding window)
    lines.into_iter().take(3).collect()
}

/// Rendu de la zone de typing (multiline with sliding window)
fn render_typing_area(f: &mut Frame, area: Rect, session: &TypingSession) {
    let terminal_width = area.width as usize;

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(7), // Expected text (3 lines + borders + padding)
            Constraint::Length(7), // User input (3 lines + borders + padding)
        ])
        .split(area);

    // Expected text - 3-line sliding window with character-level styling
    let window = extract_visible_window(session, terminal_width);
    let expected_lines = create_styled_expected_text(session, &window);

    let expected_text = Paragraph::new(expected_lines).block(
        Block::default()
            .title("Text to type")
            .borders(Borders::ALL)
            .padding(ratatui::widgets::Padding::new(2, 2, 1, 0)),
    );

    f.render_widget(expected_text, chunks[0]);

    // User input - multiline colored display
    let user_input_lines = create_colored_input_multiline(session, terminal_width);
    let input_widget = Paragraph::new(user_input_lines).block(
        Block::default()
            .title("Your input")
            .borders(Borders::ALL)
            .padding(ratatui::widgets::Padding::new(2, 2, 1, 0)),
    );

    f.render_widget(input_widget, chunks[1]);
}

/// Rendu des statistiques
fn render_stats(
    f: &mut Frame,
    area: Rect,
    wpm: f64,
    accuracy: f64,
    remaining: std::time::Duration,
) {
    let stats_text = format!(
        " WPM: {:.0}  │  Accuracy: {:.1}%  │  Time Remaining: {:02}:{:02}",
        wpm,
        accuracy,
        remaining.as_secs() / 60,
        remaining.as_secs() % 60
    );

    let stats = Paragraph::new(stats_text)
        .style(Style::default().fg(Color::Yellow))
        .alignment(Alignment::Center)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .padding(ratatui::widgets::Padding::horizontal(1)),
        );

    f.render_widget(stats, area);
}

/// Rendu des instructions
fn render_instructions(f: &mut Frame, area: Rect) {
    let instructions = vec![
        Line::from(""),
        Line::from(Span::styled(
            "ESC to quit",
            Style::default().fg(Color::Gray),
        )),
    ];

    let instructions_widget = Paragraph::new(instructions).alignment(Alignment::Center);

    f.render_widget(instructions_widget, area);
}

/// Rendu du menu de sélection de leçon
pub fn render_menu(
    f: &mut Frame,
    lessons: &[Lesson],
    selected: usize,
    scroll_offset: usize,
    category_name: Option<&str>,
) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(2)
        .constraints([
            Constraint::Length(3), // Header
            Constraint::Min(10),   // Menu
            Constraint::Length(3), // Instructions
        ])
        .split(f.area());

    // Header
    let header_text = if let Some(name) = category_name {
        format!("TYPER CLI - {} Lessons", name)
    } else {
        "TYPER CLI - Select a Lesson".to_string()
    };
    let header = Paragraph::new(header_text)
        .style(
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        )
        .alignment(Alignment::Center)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .padding(ratatui::widgets::Padding::horizontal(1)),
        );
    f.render_widget(header, chunks[0]);

    // Calculate visible area height (minus borders and padding)
    let menu_area_height = chunks[1].height.saturating_sub(2) as usize;

    // Build lesson items with category-specific grouping separators
    let mut all_items: Vec<ListItem> = Vec::new();

    match category_name {
        Some("Languages") => {
            // Group lessons by language
            use crate::content::bigram::Language;
            use crate::content::lesson::LessonType;

            let mut current_language: Option<Language> = None;

            for (i, lesson) in lessons.iter().enumerate() {
                // Detect language from lesson type
                let lesson_language = match &lesson.lesson_type {
                    LessonType::Bigram {
                        language: Some(lang),
                        ..
                    }
                    | LessonType::Trigram { language: lang, .. }
                    | LessonType::CommonWords { language: lang, .. } => Some(*lang),
                    _ => None,
                };

                // Add separator when language changes
                if lesson_language != current_language && lesson_language.is_some() {
                    current_language = lesson_language;

                    // Add blank line before separator (except for first group)
                    if i > 0 {
                        all_items.push(ListItem::new(Line::from("")));
                    }

                    // Add language separator
                    let language_name = match current_language {
                        Some(Language::French) => "FRENCH",
                        Some(Language::English) => "ENGLISH",
                        None => "",
                    };

                    all_items.push(ListItem::new(Line::from(Span::styled(
                        format!("─── {} ───", language_name),
                        Style::default()
                            .fg(Color::Cyan)
                            .add_modifier(Modifier::BOLD),
                    ))));
                }

                // Add lesson item
                let style = if i == selected {
                    Style::default()
                        .fg(Color::Yellow)
                        .add_modifier(Modifier::BOLD)
                } else {
                    Style::default().fg(Color::White)
                };

                let prefix = if i == selected { "▶ " } else { "  " };
                let content = format!("{}{}. {}", prefix, i + 1, lesson.title);

                all_items.push(ListItem::new(Line::from(Span::styled(content, style))));
            }
        }
        Some("Finger Training") => {
            // Group lessons by finger pair
            use crate::content::lesson::{FingerPairType, LessonType};

            let mut current_finger_pair: Option<FingerPairType> = None;

            for (i, lesson) in lessons.iter().enumerate() {
                // Detect finger pair from lesson type
                let lesson_finger_pair = match &lesson.lesson_type {
                    LessonType::FingerPair { finger_pair, .. } => Some(*finger_pair),
                    _ => None,
                };

                // Add separator when finger pair changes
                if lesson_finger_pair != current_finger_pair && lesson_finger_pair.is_some() {
                    current_finger_pair = lesson_finger_pair;

                    // Add blank line before separator (except for first group)
                    if i > 0 {
                        all_items.push(ListItem::new(Line::from("")));
                    }

                    // Add finger pair separator
                    let finger_name = match current_finger_pair {
                        Some(FingerPairType::Pinky) => "PINKY FINGERS",
                        Some(FingerPairType::Ring) => "RING FINGERS",
                        Some(FingerPairType::Middle) => "MIDDLE FINGERS",
                        Some(FingerPairType::Index) => "INDEX FINGERS",
                        None => "",
                    };

                    all_items.push(ListItem::new(Line::from(Span::styled(
                        format!("─── {} ───", finger_name),
                        Style::default()
                            .fg(Color::Green)
                            .add_modifier(Modifier::BOLD),
                    ))));
                }

                // Add lesson item
                let style = if i == selected {
                    Style::default()
                        .fg(Color::Yellow)
                        .add_modifier(Modifier::BOLD)
                } else {
                    Style::default().fg(Color::White)
                };

                let prefix = if i == selected { "▶ " } else { "  " };
                let content = format!("{}{}. {}", prefix, i + 1, lesson.title);

                all_items.push(ListItem::new(Line::from(Span::styled(content, style))));
            }
        }
        Some("Code") => {
            // Group lessons by type (code bigrams vs language-specific symbols)
            use crate::content::bigram::BigramType;
            use crate::content::code_symbols::ProgrammingLanguage;
            use crate::content::lesson::LessonType;

            #[derive(Debug, PartialEq, Clone, Copy)]
            enum CodeGroupType {
                CodeBigrams,
                TypeScript,
                Rust,
                Python,
            }

            let mut current_group: Option<CodeGroupType> = None;

            for (i, lesson) in lessons.iter().enumerate() {
                // Detect code group type from lesson type
                let lesson_group = match &lesson.lesson_type {
                    LessonType::Bigram {
                        bigram_type: BigramType::Code,
                        ..
                    } => Some(CodeGroupType::CodeBigrams),
                    LessonType::CodeSymbols {
                        language: ProgrammingLanguage::TypeScript,
                        ..
                    } => Some(CodeGroupType::TypeScript),
                    LessonType::CodeSymbols {
                        language: ProgrammingLanguage::Rust,
                        ..
                    } => Some(CodeGroupType::Rust),
                    LessonType::CodeSymbols {
                        language: ProgrammingLanguage::Python,
                        ..
                    } => Some(CodeGroupType::Python),
                    _ => None,
                };

                // Add separator when group changes
                if lesson_group != current_group && lesson_group.is_some() {
                    current_group = lesson_group;

                    // Add blank line before separator (except for first group)
                    if i > 0 {
                        all_items.push(ListItem::new(Line::from("")));
                    }

                    // Add group separator
                    let group_name = match current_group {
                        Some(CodeGroupType::CodeBigrams) => "CODE PATTERNS",
                        Some(CodeGroupType::TypeScript) => "TYPESCRIPT",
                        Some(CodeGroupType::Rust) => "RUST",
                        Some(CodeGroupType::Python) => "PYTHON",
                        None => "",
                    };

                    all_items.push(ListItem::new(Line::from(Span::styled(
                        format!("─── {} ───", group_name),
                        Style::default()
                            .fg(Color::Magenta)
                            .add_modifier(Modifier::BOLD),
                    ))));
                }

                // Add lesson item
                let style = if i == selected {
                    Style::default()
                        .fg(Color::Yellow)
                        .add_modifier(Modifier::BOLD)
                } else {
                    Style::default().fg(Color::White)
                };

                let prefix = if i == selected { "▶ " } else { "  " };
                let content = format!("{}{}. {}", prefix, i + 1, lesson.title);

                all_items.push(ListItem::new(Line::from(Span::styled(content, style))));
            }
        }
        _ => {
            // Standard rendering for other categories (Key Training, Adaptive)
            for (i, lesson) in lessons.iter().enumerate() {
                let style = if i == selected {
                    Style::default()
                        .fg(Color::Yellow)
                        .add_modifier(Modifier::BOLD)
                } else {
                    Style::default().fg(Color::White)
                };

                let prefix = if i == selected { "▶ " } else { "  " };
                let content = format!("{}{}. {}", prefix, i + 1, lesson.title);

                all_items.push(ListItem::new(Line::from(Span::styled(content, style))));
            }
        }
    }

    // Calculate visible slice based on scroll offset
    let total_items = all_items.len();
    let visible_start = scroll_offset.min(total_items.saturating_sub(1));
    let visible_end = (visible_start + menu_area_height).min(total_items);
    let visible_items: Vec<ListItem> = all_items
        .into_iter()
        .skip(visible_start)
        .take(visible_end - visible_start)
        .collect();

    // Add scroll indicator to title
    let scroll_indicator = if total_items > menu_area_height {
        format!(
            " (showing {}-{} of {})",
            visible_start + 1,
            visible_end,
            total_items
        )
    } else {
        String::new()
    };

    let title = format!("Typing Lessons{}", scroll_indicator);

    let list = List::new(visible_items).block(
        Block::default()
            .title(title)
            .borders(Borders::ALL)
            .padding(ratatui::widgets::Padding::new(1, 1, 1, 0)),
    );

    f.render_widget(list, chunks[1]);

    // Instructions
    let instructions = vec![
        Line::from(""),
        Line::from(Span::styled(
            "Use ↑/↓ or j/k to navigate  •  Press Enter/Space or 1-9 to select  •  ESC to go back",
            Style::default().fg(Color::Gray),
        )),
    ];

    let instructions_widget = Paragraph::new(instructions).alignment(Alignment::Center);

    f.render_widget(instructions_widget, chunks[2]);
}

/// Rendu du menu de sélection de durée
pub fn render_duration_menu(f: &mut Frame, selected: usize) {
    use crate::engine::SessionDuration;

    let durations = SessionDuration::all();

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(2)
        .constraints([
            Constraint::Length(3), // Header
            Constraint::Min(10),   // Duration list
            Constraint::Length(3), // Instructions
        ])
        .split(f.area());

    // Header
    let header = Paragraph::new("TYPER CLI - Select Session Duration")
        .style(
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        )
        .alignment(Alignment::Center)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .padding(ratatui::widgets::Padding::horizontal(1)),
        );
    f.render_widget(header, chunks[0]);

    // Duration list
    let items: Vec<ListItem> = durations
        .iter()
        .enumerate()
        .map(|(i, duration): (usize, &SessionDuration)| {
            let style = if i == selected {
                Style::default()
                    .fg(Color::Yellow)
                    .add_modifier(Modifier::BOLD)
            } else {
                Style::default().fg(Color::White)
            };

            let prefix = if i == selected { "▶ " } else { "  " };
            let content = format!("{}{}", prefix, duration.label());

            ListItem::new(Line::from(Span::styled(content, style)))
        })
        .collect();

    let list = List::new(items).block(
        Block::default()
            .title("Duration")
            .borders(Borders::ALL)
            .padding(ratatui::widgets::Padding::new(1, 1, 1, 0)),
    );

    f.render_widget(list, chunks[1]);

    // Instructions
    let instructions = vec![
        Line::from(""),
        Line::from(Span::styled(
            "Use ↑/↓ or j/k to navigate  •  Press Enter/Space to start  •  ESC to go back",
            Style::default().fg(Color::Gray),
        )),
    ];

    let instructions_widget = Paragraph::new(instructions).alignment(Alignment::Center);

    f.render_widget(instructions_widget, chunks[2]);
}

/// Render lesson type category menu
pub fn render_lesson_type_menu(
    f: &mut Frame,
    categories: &[crate::content::LessonCategory],
    selected: usize,
) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(2)
        .constraints([
            Constraint::Length(3), // Header
            Constraint::Min(10),   // Menu
            Constraint::Length(3), // Instructions
        ])
        .split(f.area());

    // Header
    let header = Paragraph::new("TYPER CLI - Select Lesson Type")
        .style(
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        )
        .alignment(Alignment::Center)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .padding(ratatui::widgets::Padding::horizontal(1)),
        );
    f.render_widget(header, chunks[0]);

    // Build category menu items (two-line format)
    let mut items: Vec<ListItem> = Vec::new();

    for (i, category) in categories.iter().enumerate() {
        let is_selected = i == selected;

        // First line: number and name
        let name_style = if is_selected {
            Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::BOLD)
        } else {
            Style::default().fg(category.color)
        };

        let prefix = if is_selected { "▶ " } else { "  " };
        let name_line = format!("{}{}. {}", prefix, i + 1, category.name);

        items.push(ListItem::new(Line::from(Span::styled(
            name_line, name_style,
        ))));

        // Second line: description
        let description_line = format!("   {}", category.description);
        items.push(ListItem::new(Line::from(Span::styled(
            description_line,
            Style::default().fg(Color::Gray),
        ))));

        // Blank line between categories (except after last)
        if i < categories.len() - 1 {
            items.push(ListItem::new(Line::from("")));
        }
    }

    let list = List::new(items).block(
        Block::default()
            .title("Lesson Categories")
            .borders(Borders::ALL)
            .padding(ratatui::widgets::Padding::new(1, 1, 1, 0)),
    );

    f.render_widget(list, chunks[1]);

    // Instructions
    let instructions = vec![
        Line::from(""),
        Line::from(Span::styled(
            "Use ↑/↓ or j/k to navigate  •  Press Enter/Space or 1-5 to select  •  Press 's' for Statistics  •  ESC to quit",
            Style::default().fg(Color::Gray),
        )),
    ];

    let instructions_widget = Paragraph::new(instructions).alignment(Alignment::Center);

    f.render_widget(instructions_widget, chunks[2]);
}

/// Rendu de l'écran de fin
pub fn render_results(
    f: &mut Frame,
    wpm: f64,
    accuracy: f64,
    duration: std::time::Duration,
    error_count: usize,
) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(4)
        .constraints([
            Constraint::Length(3), // Title
            Constraint::Length(8), // Results
            Constraint::Length(2), // Instructions
        ])
        .split(f.area());

    // Titre
    let title = Paragraph::new("Session Complete!")
        .style(
            Style::default()
                .fg(Color::Green)
                .add_modifier(Modifier::BOLD),
        )
        .alignment(Alignment::Center)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .padding(ratatui::widgets::Padding::horizontal(1)),
        );

    f.render_widget(title, chunks[0]);

    // Résultats
    let results_text = vec![
        Line::from(""),
        Line::from(Span::styled(
            format!("WPM: {:.1}", wpm),
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        )),
        Line::from(Span::styled(
            format!("Accuracy: {:.1}%", accuracy),
            Style::default().fg(Color::Green),
        )),
        Line::from(Span::styled(
            format!("Errors: {}", error_count),
            Style::default().fg(Color::Red),
        )),
        Line::from(Span::styled(
            format!(
                "Time: {:02}:{:02}",
                duration.as_secs() / 60,
                duration.as_secs() % 60
            ),
            Style::default().fg(Color::Yellow),
        )),
    ];

    let results = Paragraph::new(results_text)
        .alignment(Alignment::Center)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .padding(ratatui::widgets::Padding::horizontal(1)),
        );

    f.render_widget(results, chunks[1]);

    // Instructions
    let instructions = Paragraph::new("Press ESC to return to menu  •  Press 'r' to restart")
        .style(Style::default().fg(Color::Gray))
        .alignment(Alignment::Center);

    f.render_widget(instructions, chunks[2]);
}

/// Render statistics and performance analytics page
pub fn render_statistics(
    f: &mut Frame,
    stats: &Stats,
    keyboard_layout: &AzertyLayout,
    keyboard_config: &KeyboardConfig,
) {
    // Check if we have analytics data
    if let Some(analytics) = &stats.adaptive_analytics {
        render_statistics_with_data(f, stats, analytics, keyboard_layout, keyboard_config);
    } else {
        render_statistics_placeholder(f);
    }
}

/// Render statistics placeholder when no data exists
fn render_statistics_placeholder(f: &mut Frame) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(4)
        .constraints([
            Constraint::Length(3), // Header
            Constraint::Min(10),   // Content
            Constraint::Length(3), // Instructions
        ])
        .split(f.area());

    // Header
    let header = Paragraph::new("TYPER CLI - Statistics")
        .style(
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        )
        .alignment(Alignment::Center)
        .block(Block::default().borders(Borders::ALL));

    f.render_widget(header, chunks[0]);

    // Content - placeholder message
    let content_lines = vec![
        Line::from(""),
        Line::from(""),
        Line::from(Span::styled(
            "No Statistics Available",
            Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::BOLD),
        )),
        Line::from(""),
        Line::from("Complete your first typing session to"),
        Line::from("start tracking your performance!"),
        Line::from(""),
        Line::from(""),
        Line::from("Progress tracking includes:"),
        Line::from("  • Per-key accuracy and speed"),
        Line::from("  • Weakness identification"),
        Line::from("  • Mastery level progression"),
        Line::from("  • Common mistype patterns"),
    ];

    let content = Paragraph::new(content_lines)
        .alignment(Alignment::Center)
        .block(Block::default().borders(Borders::ALL));

    f.render_widget(content, chunks[1]);

    // Instructions
    let instructions = Paragraph::new("Press ESC to return to menu")
        .style(Style::default().fg(Color::Gray))
        .alignment(Alignment::Center);

    f.render_widget(instructions, chunks[2]);
}

/// Render statistics with actual data
fn render_statistics_with_data(
    f: &mut Frame,
    stats: &Stats,
    analytics: &AdaptiveAnalytics,
    keyboard_layout: &AzertyLayout,
    keyboard_config: &KeyboardConfig,
) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(2)
        .constraints([
            Constraint::Length(3), // Header
            Constraint::Min(20),   // Content
            Constraint::Length(3), // Instructions
        ])
        .split(f.area());

    // Header
    let header = Paragraph::new("TYPER CLI - Statistics & Performance")
        .style(
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        )
        .alignment(Alignment::Center)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .padding(ratatui::widgets::Padding::horizontal(1)),
        );

    f.render_widget(header, chunks[0]);

    // Content area - split horizontally (40% left / 60% right)
    let content_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(40), Constraint::Percentage(60)])
        .split(chunks[1]);

    // Left column - split vertically for different stats sections
    let left_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(8),  // Overall stats
            Constraint::Length(20), // Mastery breakdown (4 levels x 2 lines + spacing + borders)
            Constraint::Length(8),  // Weaknesses (reduced by 2 lines)
            Constraint::Min(10),    // Common mistypes (increased)
        ])
        .split(content_chunks[0]);

    // Render left column sections
    render_overall_stats_block(f, stats, analytics, left_chunks[0]);
    render_mastery_breakdown(f, analytics, left_chunks[1]);
    render_weaknesses_list(f, analytics, left_chunks[2]);
    render_common_mistypes(f, analytics, left_chunks[3]);

    // Render keyboard heatmap on the right
    render_keyboard_with_heatmap(
        f,
        keyboard_layout,
        keyboard_config,
        analytics,
        content_chunks[1],
    );

    // Instructions
    let session_count = analytics.total_sessions;
    let instructions_text = format!(
        "ESC to return  •  Analyzing {} session{}",
        session_count,
        if session_count == 1 { "" } else { "s" }
    );

    let instructions = Paragraph::new(instructions_text)
        .style(Style::default().fg(Color::Gray))
        .alignment(Alignment::Center);

    f.render_widget(instructions, chunks[2]);
}

/// Render overall performance statistics
fn render_overall_stats_block(
    f: &mut Frame,
    stats: &Stats,
    analytics: &AdaptiveAnalytics,
    area: Rect,
) {
    let session_count = stats.session_count();
    let total_keystrokes = analytics.total_keystrokes;
    let avg_wpm = stats.average_wpm();
    let avg_accuracy = stats.average_accuracy();

    let stats_lines = vec![
        Line::from(vec![
            Span::raw("Sessions:     "),
            Span::styled(
                format!("{}", session_count),
                Style::default().fg(Color::Yellow),
            ),
        ]),
        Line::from(vec![
            Span::raw("Total Keys:   "),
            Span::styled(
                format!("{}", total_keystrokes),
                Style::default().fg(Color::Yellow),
            ),
        ]),
        Line::from(vec![
            Span::raw("Avg WPM:      "),
            Span::styled(
                format!("{:.1}", avg_wpm),
                Style::default().fg(Color::Yellow),
            ),
        ]),
        Line::from(vec![
            Span::raw("Avg Accuracy: "),
            Span::styled(
                format!("{:.1}%", avg_accuracy),
                Style::default().fg(Color::Yellow),
            ),
        ]),
    ];

    let block = Paragraph::new(stats_lines)
        .block(
            Block::default()
                .title("Overall Performance")
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::Cyan))
                .padding(ratatui::widgets::Padding::new(1, 1, 1, 0)),
        )
        .alignment(Alignment::Left);

    f.render_widget(block, area);
}

/// Render mastery level breakdown
fn render_mastery_breakdown(f: &mut Frame, analytics: &AdaptiveAnalytics, area: Rect) {
    // Collect keys per mastery level
    let mut keys_by_level: HashMap<MasteryLevel, Vec<char>> = HashMap::new();
    for (key, stats) in &analytics.key_stats {
        keys_by_level
            .entry(stats.mastery_level)
            .or_default()
            .push(*key);
    }

    // Sort keys alphabetically for each level
    for keys in keys_by_level.values_mut() {
        keys.sort();
    }

    // Get sorted keys for each level
    let mastered_keys = keys_by_level
        .get(&MasteryLevel::Mastered)
        .cloned()
        .unwrap_or_default();
    let proficient_keys = keys_by_level
        .get(&MasteryLevel::Proficient)
        .cloned()
        .unwrap_or_default();
    let learning_keys = keys_by_level
        .get(&MasteryLevel::Learning)
        .cloned()
        .unwrap_or_default();
    let beginner_keys = keys_by_level
        .get(&MasteryLevel::Beginner)
        .cloned()
        .unwrap_or_default();

    // Helper function to format key list
    let format_key_list = |keys: &[char]| -> String {
        if keys.is_empty() {
            return String::from("  (none)");
        }
        let display_keys: Vec<String> = keys.iter().map(|k| k.to_string()).collect();
        format!("  {}", display_keys.join(" "))
    };

    let mastery_lines = vec![
        Line::from(vec![
            Span::styled("■ ", Style::default().fg(Color::Green)),
            Span::styled("Mastered:   ", Style::default().fg(Color::Green)),
            Span::styled(
                format!("{} keys", mastered_keys.len()),
                Style::default().fg(Color::Green),
            ),
        ]),
        Line::from(format_key_list(&mastered_keys)),
        Line::from(""),
        Line::from(vec![
            Span::styled("■ ", Style::default().fg(Color::Yellow)),
            Span::styled("Proficient: ", Style::default().fg(Color::Yellow)),
            Span::styled(
                format!("{} keys", proficient_keys.len()),
                Style::default().fg(Color::Yellow),
            ),
        ]),
        Line::from(format_key_list(&proficient_keys)),
        Line::from(""),
        Line::from(vec![
            Span::styled("■ ", Style::default().fg(Color::LightRed)),
            Span::styled("Learning:   ", Style::default().fg(Color::LightRed)),
            Span::styled(
                format!("{} keys", learning_keys.len()),
                Style::default().fg(Color::LightRed),
            ),
        ]),
        Line::from(format_key_list(&learning_keys)),
        Line::from(""),
        Line::from(vec![
            Span::styled("■ ", Style::default().fg(Color::Blue)),
            Span::styled("Beginner:   ", Style::default().fg(Color::Blue)),
            Span::styled(
                format!("{} keys", beginner_keys.len()),
                Style::default().fg(Color::Blue),
            ),
        ]),
        Line::from(format_key_list(&beginner_keys)),
    ];

    let block = Paragraph::new(mastery_lines)
        .block(
            Block::default()
                .title("Mastery Levels")
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::Cyan))
                .padding(ratatui::widgets::Padding::new(1, 1, 1, 0)),
        )
        .alignment(Alignment::Left);

    f.render_widget(block, area);
}

/// Render top weaknesses list
fn render_weaknesses_list(f: &mut Frame, analytics: &AdaptiveAnalytics, area: Rect) {
    let mut weak_keys: Vec<_> = analytics
        .key_stats
        .iter()
        .filter(|(_, stats)| stats.accuracy() < 80.0 && stats.total_attempts >= 5)
        .collect();

    weak_keys.sort_by(|a, b| {
        a.1.accuracy()
            .partial_cmp(&b.1.accuracy())
            .unwrap_or(std::cmp::Ordering::Equal)
    });

    let mut weakness_lines = vec![];

    if weak_keys.is_empty() {
        weakness_lines.push(Line::from(Span::styled(
            "No weaknesses - Great job!",
            Style::default()
                .fg(Color::Green)
                .add_modifier(Modifier::BOLD),
        )));
    } else {
        for (i, (key, stats)) in weak_keys.iter().take(10).enumerate() {
            let accuracy = stats.accuracy();
            let color = if accuracy < 50.0 {
                Color::Red
            } else if accuracy < 70.0 {
                Color::LightRed
            } else {
                Color::Yellow
            };

            weakness_lines.push(Line::from(vec![
                Span::raw(format!("#{} ", i + 1)),
                Span::styled(
                    format!("'{}' - {:.0}% acc", key, accuracy),
                    Style::default().fg(color),
                ),
                Span::raw(format!(" ({} errors)", stats.error_count)),
                if i == 0 {
                    Span::styled(
                        " ← WEAKEST",
                        Style::default().fg(Color::Red).add_modifier(Modifier::BOLD),
                    )
                } else {
                    Span::raw("")
                },
            ]));
        }
    }

    let block = Paragraph::new(weakness_lines)
        .block(
            Block::default()
                .title("Weakest Keys (Lower = More Practice)")
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::Cyan))
                .padding(ratatui::widgets::Padding::new(1, 1, 1, 0)),
        )
        .alignment(Alignment::Left);

    f.render_widget(block, area);
}

/// Render common mistype patterns
fn render_common_mistypes(f: &mut Frame, analytics: &AdaptiveAnalytics, area: Rect) {
    let mut all_mistypes = Vec::new();
    for (expected, key_stats) in &analytics.key_stats {
        for (typed, count) in &key_stats.mistype_map {
            all_mistypes.push((expected, typed, count));
        }
    }

    all_mistypes.sort_by(|a, b| b.2.cmp(a.2));

    let mut mistype_lines = vec![];

    if all_mistypes.is_empty() || all_mistypes.len() < 5 {
        mistype_lines.push(Line::from(Span::styled(
            "Insufficient data",
            Style::default().fg(Color::Gray),
        )));
    } else {
        // Split into two columns: items 1-5 on left, 6-10 on right
        let top_10: Vec<_> = all_mistypes.iter().take(10).collect();
        let max_rows = top_10.len().div_ceil(2);

        for i in 0..max_rows {
            let left_item = top_10.get(i);
            let right_item = top_10.get(i + max_rows);

            let mut spans = Vec::new();

            // Left column
            if let Some((expected, typed, count)) = left_item {
                spans.push(Span::raw(format!(
                    "'{}' → '{}': {} times",
                    expected, typed, count
                )));
            }

            // Spacing between columns
            if right_item.is_some() {
                spans.push(Span::raw("   "));
            }

            // Right column
            if let Some((expected, typed, count)) = right_item {
                spans.push(Span::raw(format!(
                    "'{}' → '{}': {} times",
                    expected, typed, count
                )));
            }

            mistype_lines.push(Line::from(spans));
        }
    }

    let block = Paragraph::new(mistype_lines)
        .block(
            Block::default()
                .title("Common Mistypes")
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::Cyan))
                .padding(ratatui::widgets::Padding::new(1, 1, 1, 0)),
        )
        .alignment(Alignment::Left);

    f.render_widget(block, area);
}

/// Render keyboard for statistics page
fn render_keyboard_with_heatmap(
    f: &mut Frame,
    keyboard_layout: &AzertyLayout,
    keyboard_config: &KeyboardConfig,
    analytics: &AdaptiveAnalytics,
    area: Rect,
) {
    // Create a custom config for statistics context
    let mut stats_config = keyboard_config.clone();
    stats_config.show_heatmap = true; // Always show heatmap in stats
    stats_config.show_finger_colors = false; // No finger colors in stats (heatmap takes priority)
    stats_config.show_footer_shortcuts = false; // Don't show footer (we have our own instructions)

    // render_keyboard signature: (f, area, layout, next_char, analytics, config)
    render_keyboard(
        f,
        area,
        keyboard_layout,
        None, // No next char highlighting
        &Some(analytics.clone()),
        &stats_config,
    );
}
