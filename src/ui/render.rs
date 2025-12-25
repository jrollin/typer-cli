use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

use crate::engine::TypingSession;

/// Rendu de l'interface principale
pub fn render(f: &mut Frame, session: &TypingSession, wpm: f64, accuracy: f64) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),  // Header
            Constraint::Min(8),     // Content
            Constraint::Length(3),  // Stats
        ])
        .split(f.area());

    render_header(f, chunks[0]);
    render_typing_area(f, chunks[1], session);
    render_stats(f, chunks[2], wpm, accuracy, session.duration());
}

/// Rendu du header
fn render_header(f: &mut Frame, area: Rect) {
    let title = Paragraph::new("TYPER CLI - Home Row Practice")
        .style(Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD))
        .alignment(Alignment::Center)
        .block(Block::default().borders(Borders::ALL));

    f.render_widget(title, area);
}

/// Rendu de la zone de typing
fn render_typing_area(f: &mut Frame, area: Rect, session: &TypingSession) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(2)
        .constraints([
            Constraint::Length(3),  // Expected text
            Constraint::Length(3),  // User input
        ])
        .split(area);

    // Texte attendu
    let expected_text = Paragraph::new(session.content.as_str())
        .style(Style::default().fg(Color::Gray))
        .block(Block::default().title("Text to type").borders(Borders::ALL));

    f.render_widget(expected_text, chunks[0]);

    // Saisie utilisateur avec coloration
    let user_input = create_colored_input(session);
    let input_widget = Paragraph::new(user_input)
        .block(Block::default().title("Your input").borders(Borders::ALL));

    f.render_widget(input_widget, chunks[1]);
}

/// Créer le texte coloré de l'input utilisateur
fn create_colored_input(session: &TypingSession) -> Line<'static> {
    let mut spans = Vec::new();

    for input in session.inputs.iter() {
        let color = if input.is_correct {
            Color::Green
        } else {
            Color::Red
        };

        // Render typed character with color - use visible symbol for spaces
        let display_char = if input.typed == ' ' {
            '·'  // Use middle dot to make spaces visible
        } else {
            input.typed
        };

        spans.push(Span::styled(
            display_char.to_string(),
            Style::default().fg(color),
        ));
    }

    // Ajouter le curseur
    if !session.is_complete() {
        spans.push(Span::styled(
            "█",
            Style::default()
                .fg(Color::White)
                .add_modifier(Modifier::SLOW_BLINK),
        ));
    }

    Line::from(spans)
}

/// Rendu des statistiques
fn render_stats(f: &mut Frame, area: Rect, wpm: f64, accuracy: f64, duration: std::time::Duration) {
    let stats_text = format!(
        " WPM: {:.0}  │  Accuracy: {:.1}%  │  Time: {:02}:{:02}",
        wpm,
        accuracy,
        duration.as_secs() / 60,
        duration.as_secs() % 60
    );

    let stats = Paragraph::new(stats_text)
        .style(Style::default().fg(Color::Yellow))
        .alignment(Alignment::Center)
        .block(Block::default().borders(Borders::ALL));

    f.render_widget(stats, area);
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
            Constraint::Length(3),  // Title
            Constraint::Length(8),  // Results
            Constraint::Length(2),  // Instructions
        ])
        .split(f.area());

    // Titre
    let title = Paragraph::new("Session Complete!")
        .style(Style::default().fg(Color::Green).add_modifier(Modifier::BOLD))
        .alignment(Alignment::Center)
        .block(Block::default().borders(Borders::ALL));

    f.render_widget(title, chunks[0]);

    // Résultats
    let results_text = vec![
        Line::from(""),
        Line::from(Span::styled(
            format!("WPM: {:.1}", wpm),
            Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD),
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
            format!("Time: {:02}:{:02}", duration.as_secs() / 60, duration.as_secs() % 60),
            Style::default().fg(Color::Yellow),
        )),
    ];

    let results = Paragraph::new(results_text)
        .alignment(Alignment::Center)
        .block(Block::default().borders(Borders::ALL));

    f.render_widget(results, chunks[1]);

    // Instructions
    let instructions = Paragraph::new("Press 'q' to quit, 'r' to restart")
        .style(Style::default().fg(Color::Gray))
        .alignment(Alignment::Center);

    f.render_widget(instructions, chunks[2]);
}
