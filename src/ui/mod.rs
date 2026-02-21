pub mod analytics;
pub mod keyboard;
pub mod render;

pub use render::{
    render, render_analytics_details, render_analytics_export, render_analytics_history,
    render_duration_menu, render_lesson_type_menu, render_menu, render_results, render_statistics,
};
