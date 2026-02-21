mod app;
mod content;
mod data;
mod engine;
mod keyboard;
mod ui;

use app::App;
use clap::Parser;
use keyboard::LayoutVariant;

/// Terminal-based typing trainer for AZERTY keyboards
#[derive(Parser)]
#[command(name = "typer-cli", version, about)]
struct Cli {
    /// Keyboard layout variant
    #[arg(short, long, default_value = "pc")]
    layout: LayoutVariant,
}

fn main() -> std::io::Result<()> {
    let cli = Cli::parse();

    // Initialiser le terminal
    let mut terminal = ratatui::init();

    // Créer et lancer l'app
    let result = run_app(&mut terminal, cli.layout);

    // Restaurer le terminal
    ratatui::restore();

    result
}

fn run_app(
    terminal: &mut ratatui::DefaultTerminal,
    layout_variant: LayoutVariant,
) -> std::io::Result<()> {
    let mut app = App::new(layout_variant)?;
    app.run(terminal)
}
