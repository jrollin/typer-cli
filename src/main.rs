mod app;
mod content;
mod data;
mod engine;
mod keyboard;
mod ui;

use app::App;

fn main() -> std::io::Result<()> {
    let mut terminal = ratatui::init();
    let result = run_app(&mut terminal);
    ratatui::restore();
    result
}

fn run_app(terminal: &mut ratatui::DefaultTerminal) -> std::io::Result<()> {
    let mut app = App::new()?;
    app.run(terminal)
}
