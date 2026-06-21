use super::*;

/// Structure for visible text window
struct VisibleWindow {
    lines: Vec<String>,
    /// Cumulative character count at start of each visible line (for index translation)
    line_start_indices: Vec<usize>,
}

/// Wrap text to fit terminal width, preserving newlines
fn wrap_text(content: &str, width: usize) -> Vec<String> {
    let mut lines = Vec::new();

    // Split by newlines first to preserve them
    for raw_line in content.split('\n') {
        // Wrap each line if it's too long
        let mut current_line = String::new();

        for word in raw_line.split_whitespace() {
            if current_line.is_empty() {
                current_line = word.to_string();
            } else if current_line.chars().count() + 1 + word.chars().count() <= width {
                // Count columns in chars, not bytes: accented French words (é, è...)
                // are multibyte and would otherwise wrap too early.
                current_line.push(' ');
                current_line.push_str(word);
            } else {
                lines.push(current_line);
                current_line = word.to_string();
            }
        }

        // Always push the line (even if empty) to preserve blank lines
        lines.push(current_line);
    }

    lines
}

/// Find which wrapped line contains a given character position
fn find_cursor_line(lines: &[String], char_pos: usize) -> (usize, usize) {
    let mut char_count = 0;

    for (line_idx, line) in lines.iter().enumerate() {
        let line_len = line.chars().count();

        // Check if cursor is within this line's text
        if char_pos < char_count + line_len {
            return (line_idx, char_pos.saturating_sub(char_count));
        }

        // Check if cursor is on the newline at the end of this line
        if char_pos == char_count + line_len && line_idx < lines.len() - 1 {
            return (line_idx, line_len);
        }

        // Move to next line: add line length + 1 for newline character
        char_count += line_len + 1;
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
    let (cursor_line_idx, _) = find_cursor_line(&lines, cursor_pos);

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

        // Render each character in the line
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

            // Display spaces as dots when they have errors, for visibility
            let display_char = if ch == ' '
                && absolute_index < session.current_index
                && absolute_index < session.inputs.len()
                && !session.inputs[absolute_index].is_correct
            {
                '·' // Show dot for space errors
            } else {
                ch
            };

            spans.push(Span::styled(display_char.to_string(), style));
        }

        // Add newline icon at the end of each line (except the very last line in content)
        let newline_index = line_start_index + line.chars().count();
        if newline_index < session.content_buffer_size {
            // Determine if this position represents a newline in the original content
            if session.char_at(newline_index) == Some('\n') {
                let style = if newline_index < session.current_index {
                    // Already typed - check if correct or incorrect
                    if newline_index < session.inputs.len() {
                        let input = &session.inputs[newline_index];
                        if input.is_correct {
                            Style::default().fg(Color::DarkGray)
                        } else {
                            Style::default().fg(Color::Red)
                        }
                    } else {
                        Style::default().fg(Color::DarkGray)
                    }
                } else if newline_index == session.current_index {
                    // Next character to type - highlight
                    Style::default()
                        .fg(Color::White)
                        .add_modifier(Modifier::BOLD | Modifier::UNDERLINED)
                } else {
                    // Remaining
                    Style::default().fg(Color::White)
                };

                spans.push(Span::styled("↵".to_string(), style));
            }
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
        let next_char = session.char_at(session.current_index);

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
        } else if input.typed == '\n' {
            '↵'
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
