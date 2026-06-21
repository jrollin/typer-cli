use super::*;

/// Build a styled menu list item for a lesson, highlighting the selected row.
fn render_lesson_item(index: usize, title: &str, selected: bool) -> ListItem<'static> {
    let style = if selected {
        Style::default()
            .fg(Color::Yellow)
            .add_modifier(Modifier::BOLD)
    } else {
        Style::default().fg(Color::White)
    };
    let prefix = if selected { "▶ " } else { "  " };
    let content = format!("{}{}. {}", prefix, index + 1, title);
    ListItem::new(Line::from(Span::styled(content, style)))
}

/// Rendu du menu de sélection de leçon
pub fn render_menu(
    f: &mut Frame,
    lessons: &[&Lesson],
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

                all_items.push(render_lesson_item(i, &lesson.title, i == selected));
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

                all_items.push(render_lesson_item(i, &lesson.title, i == selected));
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

                all_items.push(render_lesson_item(i, &lesson.title, i == selected));
            }
        }
        Some("Custom") => {
            // Custom lessons with helpful message when empty
            if lessons.is_empty() {
                // Display instruction message when no custom lessons found
                all_items.push(ListItem::new(Line::from("")));
                all_items.push(ListItem::new(Line::from(Span::styled(
                    "No custom lessons found.",
                    Style::default()
                        .fg(Color::Yellow)
                        .add_modifier(Modifier::BOLD),
                ))));
                all_items.push(ListItem::new(Line::from("")));
                all_items.push(ListItem::new(Line::from(Span::styled(
                    "Create custom lessons by adding markdown files to:",
                    Style::default().fg(Color::White),
                ))));
                all_items.push(ListItem::new(Line::from("")));
                all_items.push(ListItem::new(Line::from(Span::styled(
                    "  • ~/.config/typer-cli/custom/",
                    Style::default().fg(Color::Cyan),
                ))));
                all_items.push(ListItem::new(Line::from(Span::styled(
                    "  • ./custom/ (current directory)",
                    Style::default().fg(Color::Cyan),
                ))));
                all_items.push(ListItem::new(Line::from("")));
                all_items.push(ListItem::new(Line::from(Span::styled(
                    "Example file format:",
                    Style::default().fg(Color::White),
                ))));
                all_items.push(ListItem::new(Line::from("")));
                all_items.push(ListItem::new(Line::from(Span::styled(
                    "  ---",
                    Style::default().fg(Color::Gray),
                ))));
                all_items.push(ListItem::new(Line::from(Span::styled(
                    "  title: My Lesson",
                    Style::default().fg(Color::Gray),
                ))));
                all_items.push(ListItem::new(Line::from(Span::styled(
                    "  description: Practice custom content",
                    Style::default().fg(Color::Gray),
                ))));
                all_items.push(ListItem::new(Line::from(Span::styled(
                    "  ---",
                    Style::default().fg(Color::Gray),
                ))));
                all_items.push(ListItem::new(Line::from("")));
                all_items.push(ListItem::new(Line::from(Span::styled(
                    "  Your custom text to practice goes here.",
                    Style::default().fg(Color::Gray),
                ))));
                all_items.push(ListItem::new(Line::from("")));
                all_items.push(ListItem::new(Line::from(Span::styled(
                    "Restart the app after adding files.",
                    Style::default()
                        .fg(Color::Yellow)
                        .add_modifier(Modifier::ITALIC),
                ))));
            } else {
                // Display custom lessons normally
                for (i, lesson) in lessons.iter().enumerate() {
                    all_items.push(render_lesson_item(i, &lesson.title, i == selected));
                }
            }
        }
        _ => {
            // Standard rendering for other categories (Key Training, Adaptive)
            for (i, lesson) in lessons.iter().enumerate() {
                all_items.push(render_lesson_item(i, &lesson.title, i == selected));
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
    layout_variant: crate::keyboard::LayoutVariant,
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
    let layout_name = match layout_variant {
        crate::keyboard::LayoutVariant::Mac => "Mac",
        crate::keyboard::LayoutVariant::Pc => "PC",
    };
    let hint = format!(
        "↑/↓ j/k navigate  •  Enter/1-5 select  •  [s] Statistics  •  [p] Preferences (Layout: {})  •  ESC quit",
        layout_name
    );
    let instructions = vec![
        Line::from(""),
        Line::from(Span::styled(hint, Style::default().fg(Color::Gray))),
    ];

    let instructions_widget = Paragraph::new(instructions).alignment(Alignment::Center);

    f.render_widget(instructions_widget, chunks[2]);
}
