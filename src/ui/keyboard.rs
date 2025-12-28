use ratatui::{
    layout::{Alignment, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, Paragraph},
    Frame,
};

use crate::engine::analytics::AdaptiveAnalytics;
use crate::keyboard::{AzertyLayout, Hand, Key, RowType};

/// Keyboard display configuration
pub struct KeyboardConfig {
    pub _show_shift_indicators: bool,
    pub show_heatmap: bool,
    pub show_finger_colors: bool,
    pub _compact_mode: bool,
}

impl Default for KeyboardConfig {
    fn default() -> Self {
        Self {
            _show_shift_indicators: true,
            show_heatmap: true,
            show_finger_colors: true,
            _compact_mode: false,
        }
    }
}

/// Calculate color based on accuracy heatmap
fn get_accuracy_color(accuracy: f64) -> Color {
    if accuracy >= 90.0 {
        Color::Green // Mastered: 90-100%
    } else if accuracy >= 80.0 {
        Color::Yellow // Good: 80-90%
    } else if accuracy >= 70.0 {
        Color::LightRed // Learning: 70-80%
    } else {
        Color::Red // Weak: <70%
    }
}

/// Get accuracy for a specific key from analytics
fn get_key_accuracy(key: char, analytics: &Option<AdaptiveAnalytics>) -> Option<f64> {
    analytics
        .as_ref()?
        .key_stats
        .get(&key)
        .map(|stats| stats.accuracy())
}

/// Determine if a shift key should be highlighted based on which hand presses the next key
/// Smart shift highlighting: use opposite hand for shift (ergonomic typing technique)
fn should_highlight_shift(
    next_char: Option<char>,
    requires_shift: bool,
    layout: &AzertyLayout,
    is_left_shift: bool,
) -> bool {
    if !requires_shift {
        return false;
    }

    if let Some(c) = next_char {
        if let Some(base_key) = layout.get_base_key(c) {
            if let Some(key) = layout.find_key(base_key) {
                match key.finger.hand() {
                    Hand::Right => return is_left_shift, // Right hand → use left shift
                    Hand::Left => return !is_left_shift, // Left hand → use right shift
                    Hand::Either => return true,         // Spacebar → both shifts valid
                }
            }
        }
    }

    false
}

/// Render a single key with styling
fn render_key(
    key: &Key,
    _row_type: RowType,
    next_char: Option<char>,
    requires_shift: bool,
    analytics: &Option<AdaptiveAnalytics>,
    config: &KeyboardConfig,
) -> Span<'static> {
    let is_highlighted = Some(key.base) == next_char
        || (requires_shift
            && key.shift_variant.is_some()
            && Some(key.base)
                == next_char.and_then(|c| {
                    // Check if this is the base key for the shift variant
                    if key.shift_variant == Some(c) {
                        Some(key.base)
                    } else {
                        None
                    }
                }));

    // Determine display character
    // Always show base character (e.g., [1] [2] [3], NOT [&] [é] ["])
    let display_char = key.base;

    // Build style with priority system
    let style = if is_highlighted {
        // Priority 1: Next key to press: reverse video (black on cyan)
        Style::default()
            .fg(Color::Black)
            .bg(Color::Cyan)
            .add_modifier(Modifier::BOLD)
    } else if config.show_heatmap {
        // Priority 2: Heatmap overlay
        if let Some(accuracy) = get_key_accuracy(key.base, analytics) {
            Style::default().fg(get_accuracy_color(accuracy))
        } else if config.show_finger_colors {
            // Fallback to finger color if no analytics data
            Style::default().fg(key.finger.color())
        } else {
            Style::default().fg(Color::White) // No data
        }
    } else if config.show_finger_colors {
        // Priority 3: Finger color baseline
        Style::default().fg(key.finger.color())
    } else {
        Style::default().fg(Color::White)
    };

    // Format: [x] - 3 characters wide
    let text = format!("[{}]", display_char);

    Span::styled(text, style)
}

/// Render a keyboard row
fn render_keyboard_row<'a>(
    row: &crate::keyboard::KeyboardRow,
    next_char: Option<char>,
    requires_shift: bool,
    analytics: &Option<AdaptiveAnalytics>,
    config: &KeyboardConfig,
    layout: &AzertyLayout,
) -> Line<'a> {
    let mut spans = Vec::new();

    // Add center padding
    spans.push(Span::raw("              ")); // 14 spaces

    // Add modifier key box at start of row (Tab, Caps, Shift)
    match row.row_type {
        RowType::Top => {
            spans.push(Span::styled("[Tab] ", Style::default().fg(Color::DarkGray)));
        }
        RowType::Home => {
            spans.push(Span::styled(
                "[Caps] ",
                Style::default().fg(Color::DarkGray),
            ));
        }
        RowType::Bottom => {
            // Left Shift - highlight only for right-hand keys (smart shift highlighting)
            let highlight_left = should_highlight_shift(next_char, requires_shift, layout, true);
            let shift_style = if highlight_left {
                Style::default()
                    .fg(Color::Black)
                    .bg(Color::Cyan)
                    .add_modifier(Modifier::BOLD)
            } else {
                Style::default().fg(Color::DarkGray)
            };
            spans.push(Span::styled("[ ⇧ ] ", shift_style));
        }
        _ => {}
    }

    // Render keys
    for (i, key) in row.keys.iter().enumerate() {
        // Special handling for modifier row keys
        if row.row_type == RowType::Modifier {
            match i {
                0 => {
                    spans.push(Span::styled(
                        "[Ctrl] ",
                        Style::default().fg(Color::DarkGray),
                    ));
                }
                1 => {
                    spans.push(Span::styled("[⌘] ", Style::default().fg(Color::DarkGray)));
                }
                2 => {
                    spans.push(Span::styled("[⌥] ", Style::default().fg(Color::DarkGray)));
                }
                3 => {
                    // Space key - can be highlighted
                    let is_highlighted = next_char == Some(' ');
                    let style = if is_highlighted {
                        Style::default()
                            .fg(Color::Black)
                            .bg(Color::Cyan)
                            .add_modifier(Modifier::BOLD)
                    } else {
                        Style::default().fg(Color::White)
                    };
                    spans.push(Span::styled("[        Space        ] ", style));
                }
                4 => {
                    spans.push(Span::styled("[Alt] ", Style::default().fg(Color::DarkGray)));
                }
                5 => {
                    spans.push(Span::styled("[Fn1] ", Style::default().fg(Color::DarkGray)));
                }
                6 => {
                    spans.push(Span::styled("[Fn2]", Style::default().fg(Color::DarkGray)));
                }
                _ => {}
            }
        }
        // Special handling for space bar (legacy, now in modifier row)
        else if row.row_type == RowType::Space && key.base == ' ' {
            let is_highlighted = next_char == Some(' ');
            let style = if is_highlighted {
                Style::default()
                    .fg(Color::Black)
                    .bg(Color::Cyan)
                    .add_modifier(Modifier::BOLD)
            } else {
                Style::default().fg(Color::White)
            };
            // Space bar: 7 keys width, moved left by 2 keys from previous position
            // 12 - 8 = 4 spaces offset
            spans.push(Span::raw("    ")); // 4 additional spaces (moved left by 2 keys)
            spans.push(Span::styled("[        Space        ]", style)); // ~7 key widths
        } else if key.base == '\n' {
            // Special handling for Enter key - show arrow [←] on home row only
            if row.row_type == RowType::Home {
                let is_highlighted = next_char == Some('\n');
                let style = if is_highlighted {
                    Style::default()
                        .fg(Color::Black)
                        .bg(Color::Cyan)
                        .add_modifier(Modifier::BOLD)
                } else {
                    Style::default().fg(Color::DarkGray) // Grey when not highlighted
                };
                spans.push(Span::raw(" "));
                spans.push(Span::styled("[←]", style));
            }
            // Skip Enter on top row (don't render anything)
        } else if key.base == '\0' {
            // Skip null placeholders (don't render)
        } else {
            let key_span = render_key(
                key,
                row.row_type,
                next_char,
                requires_shift,
                analytics,
                config,
            );
            spans.push(key_span);

            // Add space between keys (except after last key)
            if i < row.keys.len() - 1 {
                spans.push(Span::raw(" "));
            }
        }
    }

    // Add Right Shift key at end of bottom row
    if row.row_type == RowType::Bottom {
        // Right Shift - highlight only for left-hand keys (smart shift highlighting)
        let highlight_right = should_highlight_shift(next_char, requires_shift, layout, false);
        let shift_style = if highlight_right {
            Style::default()
                .fg(Color::Black)
                .bg(Color::Cyan)
                .add_modifier(Modifier::BOLD)
        } else {
            Style::default().fg(Color::DarkGray)
        };
        spans.push(Span::raw(" "));
        spans.push(Span::styled("[ ⇧ ]", shift_style));
    }

    Line::from(spans)
}

/// Render full keyboard display
pub fn render_keyboard(
    f: &mut Frame,
    area: Rect,
    layout: &AzertyLayout,
    next_char: Option<char>,
    analytics: &Option<AdaptiveAnalytics>,
    config: &KeyboardConfig,
) {
    let requires_shift = next_char.map(|c| layout.requires_shift(c)).unwrap_or(false);

    let mut lines = Vec::new();

    // Build keyboard rows
    for row in &layout.rows {
        let line = render_keyboard_row(row, next_char, requires_shift, analytics, config, layout);
        lines.push(line);
    }

    // Add spacing before legend
    lines.push(Line::from(""));

    // Add legend if heatmap is enabled
    if config.show_heatmap {
        let legend1 = Line::from(vec![
            Span::raw(" "),
            Span::styled("■", Style::default().fg(Color::Green)),
            Span::raw(" Mastered (90%+)   "),
            Span::styled("■", Style::default().fg(Color::Yellow)),
            Span::raw(" Good (80-90%)"),
        ]);

        let legend2 = Line::from(vec![
            Span::raw(" "),
            Span::styled("■", Style::default().fg(Color::LightRed)),
            Span::raw(" Learning (70-80%) "),
            Span::styled("■", Style::default().fg(Color::Red)),
            Span::raw(" Weak (<70%)"),
        ]);

        lines.push(legend1);
        lines.push(legend2);
    } else if config.show_finger_colors {
        // Show finger color legend when finger colors enabled and heatmap disabled
        let legend1 = Line::from(vec![
            Span::raw(" Finger Guide: "),
            Span::styled("■", Style::default().fg(Color::Magenta)),
            Span::raw(" L.Pinky "),
            Span::styled("■", Style::default().fg(Color::LightBlue)),
            Span::raw(" L.Ring "),
            Span::styled("■", Style::default().fg(Color::Blue)),
            Span::raw(" L.Middle "),
            Span::styled("■", Style::default().fg(Color::Cyan)),
            Span::raw(" L.Index"),
        ]);

        let legend2 = Line::from(vec![
            Span::raw("               "),
            Span::styled("■", Style::default().fg(Color::Green)),
            Span::raw(" R.Index "),
            Span::styled("■", Style::default().fg(Color::Yellow)),
            Span::raw(" R.Middle "),
            Span::styled("■", Style::default().fg(Color::LightRed)),
            Span::raw(" R.Ring "),
            Span::styled("■", Style::default().fg(Color::Red)),
            Span::raw(" R.Pinky"),
        ]);

        lines.push(legend1);
        lines.push(legend2);
    }

    // Footer hint - always show all available toggles
    lines.push(Line::from(""));
    let footer_text = " Tab: hide keyboard | H: toggle heatmap | F: toggle finger colors";
    lines.push(Line::from(Span::styled(
        footer_text,
        Style::default().fg(Color::DarkGray),
    )));

    let keyboard_widget = Paragraph::new(lines)
        .block(
            Block::default()
                .title("Keyboard Layout")
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded),
        )
        .alignment(Alignment::Left);

    f.render_widget(keyboard_widget, area);
}

/// Render compact keyboard (single-line, next key only)
pub fn render_keyboard_compact(
    f: &mut Frame,
    area: Rect,
    layout: &AzertyLayout,
    next_char: Option<char>,
) {
    let text = if let Some(c) = next_char {
        let requires_shift = layout.requires_shift(c);

        if requires_shift {
            format!(" Next key: [{}] (⇧ Shift)              (Tab to expand)", c)
        } else {
            format!(" Next key: [{}]                     (Tab to expand)", c)
        }
    } else {
        " No active session                (Tab to expand)".to_string()
    };

    let compact_widget = Paragraph::new(text)
        .style(Style::default().fg(Color::Cyan))
        .alignment(Alignment::Left)
        .block(
            Block::default()
                .title("Keyboard")
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded),
        );

    f.render_widget(compact_widget, area);
}
