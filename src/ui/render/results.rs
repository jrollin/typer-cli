use super::*;

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
