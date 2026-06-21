use super::*;

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
    let instructions = Paragraph::new("h: History  d: Details  e: Export  ESC to return to menu")
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
        "h: History  d: Details  e: Export  ESC to return  •  Analyzing {} session{}",
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

/// Group practiced keys by mastery level, each list sorted alphabetically.
/// Pure (no rendering) so it can be unit-tested.
fn keys_for_level(analytics: &AdaptiveAnalytics, level: MasteryLevel) -> Vec<char> {
    let mut keys: Vec<char> = analytics
        .key_stats
        .iter()
        .filter(|(_, stats)| stats.mastery_level == level)
        .map(|(key, _)| *key)
        .collect();
    keys.sort();
    keys
}

/// Render mastery level breakdown
fn render_mastery_breakdown(f: &mut Frame, analytics: &AdaptiveAnalytics, area: Rect) {
    let mastered_keys = keys_for_level(analytics, MasteryLevel::Mastered);
    let proficient_keys = keys_for_level(analytics, MasteryLevel::Proficient);
    let learning_keys = keys_for_level(analytics, MasteryLevel::Learning);
    let beginner_keys = keys_for_level(analytics, MasteryLevel::Beginner);

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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::engine::analytics::KeyStats;

    fn key(c: char, level: MasteryLevel) -> KeyStats {
        let mut k = KeyStats::new(c);
        k.mastery_level = level;
        k
    }

    #[test]
    fn keys_for_level_groups_and_sorts() {
        let mut analytics = AdaptiveAnalytics::default();
        analytics
            .key_stats
            .insert('f', key('f', MasteryLevel::Mastered));
        analytics
            .key_stats
            .insert('a', key('a', MasteryLevel::Mastered));
        analytics
            .key_stats
            .insert('j', key('j', MasteryLevel::Learning));

        assert_eq!(
            keys_for_level(&analytics, MasteryLevel::Mastered),
            vec!['a', 'f'] // sorted
        );
        assert_eq!(
            keys_for_level(&analytics, MasteryLevel::Learning),
            vec!['j']
        );
        assert!(keys_for_level(&analytics, MasteryLevel::Beginner).is_empty());
    }
}
