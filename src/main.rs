pub mod app;
pub mod ui;
pub mod event;
pub mod tui;
pub mod update;

use app::App;
use tui::Tui;
use update::update;
use event::{Event, EventHandler};

use color_eyre::eyre::Result;
use ratatui::{backend::CrosstermBackend, Terminal};

fn main() -> Result<()> {
    // Create app and initialize TUI
    let mut app = App::new();

    let backend = CrosstermBackend::new(std::io::stderr());
    let terminal = Terminal::new(backend)?;
    let events = EventHandler::new(1000 / 30);
    let mut tui = Tui::new(terminal, events);
    tui.enter()?;

    // App main loop
    while !app.should_quit {
        // Handle events
        match tui.events.next()? {
            Event::Tick => tui.draw(&mut app)?,
            Event::Key(key_event) => update(&mut app, key_event),
            Event::Mouse(_) => {},
            Event::Resize(_, _) => {},
        };
    }

    // Quit the app
    tui.exit()?;
    Ok(())
}