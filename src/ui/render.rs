use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, Paragraph},
    Frame,
};

use crate::content::Lesson;
use crate::engine::analytics::AdaptiveAnalytics;
use crate::engine::TypingSession;
use crate::keyboard::AzertyLayout;
use crate::ui::keyboard::{render_keyboard, render_keyboard_compact, KeyboardConfig};

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
            return (line_idx, char_pos - char_count);
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
                Constraint::Length(10), // Content (5 + 5 lines, no margin)
                Constraint::Length(12), // Keyboard (follows content)
                Constraint::Min(0),     // Spacer (absorbs remaining space)
                Constraint::Length(3),  // Instructions (bottom)
            ]
        } else if terminal_height >= 23 {
            // Full keyboard without shift line
            vec![
                Constraint::Length(3),  // Header
                Constraint::Length(3),  // Stats (moved after header)
                Constraint::Length(10), // Content (5 + 5 lines, no margin)
                Constraint::Length(10), // Keyboard (follows content)
                Constraint::Min(0),     // Spacer
                Constraint::Length(3),  // Instructions (bottom)
            ]
        } else {
            // Compact keyboard
            vec![
                Constraint::Length(3),  // Header
                Constraint::Length(3),  // Stats (moved after header)
                Constraint::Length(10), // Content (5 + 5 lines, no margin)
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
        .block(Block::default().borders(Borders::ALL));

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
            Constraint::Length(5), // Expected text (3 lines + borders)
            Constraint::Length(5), // User input (3 lines + borders)
        ])
        .split(area);

    // Expected text - 3-line sliding window with character-level styling
    let window = extract_visible_window(session, terminal_width);
    let expected_lines = create_styled_expected_text(session, &window);

    let expected_text = Paragraph::new(expected_lines)
        .block(Block::default().title("Text to type").borders(Borders::ALL));

    f.render_widget(expected_text, chunks[0]);

    // User input - multiline colored display
    let user_input_lines = create_colored_input_multiline(session, terminal_width);
    let input_widget = Paragraph::new(user_input_lines)
        .block(Block::default().title("Your input").borders(Borders::ALL));

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
        .block(Block::default().borders(Borders::ALL));

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
pub fn render_menu(f: &mut Frame, lessons: &[Lesson], selected: usize, scroll_offset: usize) {
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
    let header = Paragraph::new("TYPER CLI - Select a Lesson")
        .style(
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        )
        .alignment(Alignment::Center)
        .block(Block::default().borders(Borders::ALL));
    f.render_widget(header, chunks[0]);

    // Calculate visible area height (minus borders and padding)
    let menu_area_height = chunks[1].height.saturating_sub(2) as usize;

    // Build complete items list with category separators
    let mut all_items: Vec<ListItem> = Vec::new();

    // Determine if adaptive mode is present (first lesson)
    let has_adaptive = !lessons.is_empty()
        && matches!(
            lessons[0].lesson_type,
            crate::content::lesson::LessonType::Adaptive
        );

    // ADAPTIVE section header (if present - first lesson)
    if has_adaptive {
        all_items.push(ListItem::new(Line::from(Span::styled(
            "━━━ ADAPTIVE ━━━",
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        ))));
    }

    // Add all lessons with category separators
    for (i, lesson) in lessons.iter().enumerate() {
        // Add FINGER TRAINING separator (after adaptive if present, otherwise first)
        let finger_training_index = if has_adaptive { 1 } else { 0 };
        if i == finger_training_index {
            if has_adaptive {
                all_items.push(ListItem::new(Line::from("")));
            }
            all_items.push(ListItem::new(Line::from(Span::styled(
                "━━━ FINGER TRAINING ━━━",
                Style::default()
                    .fg(Color::Green)
                    .add_modifier(Modifier::BOLD),
            ))));
        }

        // Add PRIMARY separator (after finger training - 24 lessons later)
        let primary_index = if has_adaptive { 25 } else { 24 };
        if i == primary_index {
            all_items.push(ListItem::new(Line::from("")));
            all_items.push(ListItem::new(Line::from(Span::styled(
                "━━━ PRIMARY - Key Training ━━━",
                Style::default()
                    .fg(Color::Cyan)
                    .add_modifier(Modifier::BOLD),
            ))));
        }

        // Add SECONDARY separator (after primary - 25 lessons later)
        let secondary_index = if has_adaptive { 50 } else { 49 };
        if i == secondary_index {
            all_items.push(ListItem::new(Line::from("")));
            all_items.push(ListItem::new(Line::from(Span::styled(
                "━━━ SECONDARY - Programming & Languages ━━━",
                Style::default()
                    .fg(Color::Cyan)
                    .add_modifier(Modifier::BOLD),
            ))));
        }

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

    let list = List::new(visible_items).block(Block::default().title(title).borders(Borders::ALL));

    f.render_widget(list, chunks[1]);

    // Instructions
    let instructions = vec![
        Line::from(""),
        Line::from(Span::styled(
            "Use ↑/↓ or j/k to navigate  •  Press Enter/Space or 1-6 to select  •  ESC to quit",
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
        .block(Block::default().borders(Borders::ALL));
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

    let list = List::new(items).block(Block::default().title("Duration").borders(Borders::ALL));

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
        .block(Block::default().borders(Borders::ALL));

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
        .block(Block::default().borders(Borders::ALL));

    f.render_widget(results, chunks[1]);

    // Instructions
    let instructions = Paragraph::new("Press ESC to return to menu  •  Press 'r' to restart")
        .style(Style::default().fg(Color::Gray))
        .alignment(Alignment::Center);

    f.render_widget(instructions, chunks[2]);
}
