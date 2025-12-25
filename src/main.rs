mod app;
mod content;
mod data;
mod engine;
mod keyboard;
mod ui;

use app::App;

fn main() -> std::io::Result<()> {
    // Initialiser le terminal
    let mut terminal = ratatui::init();

    // Créer et lancer l'app
    let result = run_app(&mut terminal);

    // Restaurer le terminal
    ratatui::restore();

    result
}

fn run_app(terminal: &mut ratatui::DefaultTerminal) -> std::io::Result<()> {
    let mut app = App::new()?;
    app.run(terminal)
}
