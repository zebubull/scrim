use scrim::app::App;
use scrim::tui::Tui;
use scrim::update::update;
use scrim::event::{Event, EventHandler};

use color_eyre::eyre::Result;
use ratatui::{backend::CrosstermBackend, Terminal};

fn main() -> Result<()> {
    // Create app and initialize TUI
    color_eyre::install()?;
    let mut app = App::new();

    let backend = CrosstermBackend::new(std::io::stdout());
    let terminal = Terminal::new(backend)?;
    let events = EventHandler::new(1000 / 30);
    let mut tui = Tui::new(terminal, events);

    // Load player data
    if std::path::Path::new("save.json").exists() {
        app.load_player("save.json")?;
    }

    // App main loop
    tui.enter()?;
    while !app.should_quit {
        // Handle events
        match tui.events.next().unwrap() {
            Event::Tick => tui.draw(&mut app)?,
            Event::Key(key_event) => update(&mut app, key_event)?,
            Event::Mouse(_) => {},
            Event::Resize(_, _) => {},
        };
    }

    // Quit the app
    tui.exit().unwrap();

    let data = serde_json::to_string(&app.player)?;
    std::fs::write("save.json", data)?;

    Ok(())
}