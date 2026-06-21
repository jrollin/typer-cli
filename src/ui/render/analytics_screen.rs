use super::*;

/// Render analytics history with ASCII charts
pub fn render_analytics_history(f: &mut Frame, stats: &Stats) {
    use crate::ui::analytics::{
        calculate_daily_goal_progress, calculate_daily_progress, calculate_monthly_progress,
        calculate_weekly_progress, create_daily_wpm_chart, create_goal_progress_display,
        create_monthly_wpm_chart, create_weekly_wpm_chart, get_session_chart_stats,
    };

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3), // Header
            Constraint::Min(0),    // Content
            Constraint::Length(3), // Instructions
        ])
        .split(f.area());

    // Header
    let header = Paragraph::new("Analytics History")
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::Cyan)),
        )
        .alignment(Alignment::Center);
    f.render_widget(header, chunks[0]);

    // Content area with charts
    let content = if stats.sessions.is_empty() {
        vec![Line::from(Span::styled(
            "No session data available. Complete some lessons first!",
            Style::default().fg(Color::Yellow),
        ))]
    } else {
        let mut lines = Vec::new();

        // Get recent sessions for charting
        let recent_sessions = get_session_chart_stats(stats);

        if !recent_sessions.is_empty() {
            // Daily Progress Chart
            let daily_progress = calculate_daily_progress(&recent_sessions);
            if !daily_progress.is_empty() {
                let daily_chart = create_daily_wpm_chart(&daily_progress);
                lines.extend(daily_chart.into_iter().map(Line::from));
                lines.push(Line::from(""));
            }

            // Add spacing
            lines.push(Line::from(""));

            // Weekly Progress Chart
            let weekly_progress = calculate_weekly_progress(&recent_sessions);
            if !weekly_progress.is_empty() {
                let weekly_chart = create_weekly_wpm_chart(&weekly_progress);
                lines.extend(weekly_chart.into_iter().map(Line::from));
                lines.push(Line::from(""));
            }

            // Add spacing
            lines.push(Line::from(""));

            // Monthly Progress Chart
            let monthly_progress = calculate_monthly_progress(&recent_sessions);
            if !monthly_progress.is_empty() {
                let monthly_chart = create_monthly_wpm_chart(&monthly_progress);
                lines.extend(monthly_chart.into_iter().map(Line::from));
                lines.push(Line::from(""));
            }

            // Add spacing
            lines.push(Line::from(""));

            // Goal Progress Display
            let (current_wpm, progress_percent, status, details) =
                calculate_daily_goal_progress(&stats.sessions, 50.0); // Default goal: 50 WPM
            let goal_display = create_goal_progress_display(
                current_wpm,
                50.0,
                progress_percent,
                &status,
                &details,
            );
            lines.extend(goal_display.into_iter().map(Line::from));
            lines.push(Line::from(""));
            lines.push(Line::from(""));

            // Add session summary
            lines.push(Line::from(""));
            lines.push(Line::from(Span::styled(
                format!("Showing {} most recent sessions", recent_sessions.len()),
                Style::default().fg(Color::Gray),
            )));
        } else {
            lines.push(Line::from("No session data available for charting."));
        }

        lines
    };

    let content_area = chunks[1];
    let content_block = Paragraph::new(content)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::Cyan))
                .padding(ratatui::widgets::Padding::new(1, 1, 1, 0)),
        )
        .alignment(Alignment::Left);
    f.render_widget(content_block, content_area);

    // Instructions
    let instructions = Paragraph::new("o: Overview  d: Details  e: Export  q: Back to Menu")
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::Cyan)),
        )
        .alignment(Alignment::Center);
    f.render_widget(instructions, chunks[2]);
}

/// Render detailed analytics
pub fn render_analytics_details(f: &mut Frame, stats: &Stats) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3), // Header
            Constraint::Min(0),    // Content
            Constraint::Length(3), // Instructions
        ])
        .split(f.area());

    // Header
    let header = Paragraph::new("Detailed Analytics")
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::Cyan)),
        )
        .alignment(Alignment::Center);
    f.render_widget(header, chunks[0]);

    // Content area - placeholder for now
    let content = if let Some(analytics) = &stats.adaptive_analytics {
        vec![
            Line::from("Per-Key Performance Details:"),
            Line::from("Coming soon - detailed analytics implementation..."),
            Line::from(""),
            Line::from(format!("Total Keys Tracked: {}", analytics.key_stats.len())),
            Line::from(format!(
                "Total Bigrams Tracked: {}",
                analytics.bigram_stats.len()
            )),
            Line::from(""),
            Line::from("Weakness Detection:"),
            Line::from("Coming soon - weakness analysis implementation..."),
        ]
    } else {
        vec![Line::from(Span::styled(
            "No detailed analytics available. Complete more sessions first!",
            Style::default().fg(Color::Yellow),
        ))]
    };

    let content_area = chunks[1];
    let content_block = Paragraph::new(content)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::Cyan)),
        )
        .alignment(Alignment::Left);
    f.render_widget(content_block, content_area);

    // Instructions
    let instructions = Paragraph::new("o: Overview  h: History  e: Export  q: Back to Menu")
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::Cyan)),
        )
        .alignment(Alignment::Center);
    f.render_widget(instructions, chunks[2]);
}

/// Render analytics export options
pub fn render_analytics_export(f: &mut Frame, stats: &Stats, export_message: Option<&str>) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3), // Header
            Constraint::Min(0),    // Content
            Constraint::Length(3), // Instructions
        ])
        .split(f.area());

    // Header
    let header = Paragraph::new("Export Analytics")
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::Cyan)),
        )
        .alignment(Alignment::Center);
    f.render_widget(header, chunks[0]);

    // Content area
    let mut content = vec![
        Line::from("Export Options:"),
        Line::from(""),
        Line::from("1. Export Complete Data (JSON)"),
        Line::from("   Export all sessions, key stats, and bigram data"),
        Line::from(""),
        Line::from(format!("Total Sessions: {}", stats.sessions.len())),
        Line::from(format!(
            "Total Keystrokes: {}",
            stats
                .adaptive_analytics
                .as_ref()
                .map(|a| a.total_keystrokes.to_string())
                .unwrap_or_else(|| "N/A".to_string())
        )),
    ];

    if let Some(msg) = export_message {
        content.push(Line::from(""));
        let color = if msg.starts_with("Export failed") {
            Color::Red
        } else {
            Color::Green
        };
        content.push(Line::from(Span::styled(
            msg.to_string(),
            Style::default().fg(color),
        )));
    }

    let content_area = chunks[1];
    let content_block = Paragraph::new(content)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::Cyan)),
        )
        .alignment(Alignment::Left);
    f.render_widget(content_block, content_area);

    // Instructions
    let instructions =
        Paragraph::new("1: Export JSON  o: Overview  h: History  d: Details  q: Back to Menu")
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .border_style(Style::default().fg(Color::Cyan)),
            )
            .alignment(Alignment::Center);
    f.render_widget(instructions, chunks[2]);
}

/// Render the settings screen for layout selection
pub fn render_settings(
    f: &mut Frame,
    selected_layout: usize,
    current_variant: crate::keyboard::LayoutVariant,
) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(2)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(6),
            Constraint::Length(3),
        ])
        .split(f.area());

    let header = Paragraph::new("TYPER CLI - Settings")
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

    let layouts = [
        ("AZERTY Mac", crate::keyboard::LayoutVariant::Mac),
        ("AZERTY PC", crate::keyboard::LayoutVariant::Pc),
    ];

    let items: Vec<ListItem> = layouts
        .iter()
        .enumerate()
        .map(|(i, (name, variant))| {
            let is_selected = i == selected_layout;
            let is_active = *variant == current_variant;
            let prefix = if is_selected { "▶ " } else { "  " };
            let check = if is_active { " [✓]" } else { "" };
            let label = format!("{}{}{}", prefix, name, check);
            let style = if is_selected {
                Style::default()
                    .fg(Color::Yellow)
                    .add_modifier(Modifier::BOLD)
            } else {
                Style::default().fg(Color::White)
            };
            ListItem::new(Line::from(Span::styled(label, style)))
        })
        .collect();

    let list = List::new(items).block(
        Block::default()
            .title("Keyboard Layout")
            .borders(Borders::ALL)
            .padding(ratatui::widgets::Padding::new(1, 1, 1, 0)),
    );
    f.render_widget(list, chunks[1]);

    let instructions = vec![
        Line::from(""),
        Line::from(Span::styled(
            "Use ↑/↓ or j/k to navigate  •  Enter to apply and save  •  ESC to cancel",
            Style::default().fg(Color::Gray),
        )),
    ];
    f.render_widget(
        Paragraph::new(instructions).alignment(Alignment::Center),
        chunks[2],
    );
}
