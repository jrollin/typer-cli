//! Terminal rendering, split by screen concern:
//! - `session`: the live typing screen (text window, cursor, per-key styling)
//! - `menu`: lesson, duration, and category menus
//! - `results`: the post-session results screen
//! - `statistics`: the statistics dashboard and its sub-blocks
//! - `analytics_screen`: analytics history/details/export and settings

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

mod analytics_screen;
mod menu;
mod results;
mod session;
mod statistics;

pub use analytics_screen::{
    render_analytics_details, render_analytics_export, render_analytics_history, render_settings,
};
pub use menu::{render_duration_menu, render_lesson_type_menu, render_menu};
pub use results::render_results;
pub use session::render;
pub use statistics::render_statistics;
