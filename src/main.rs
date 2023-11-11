use scrim::app::App;
use scrim::tui::Tui;
use scrim::update::update;
use scrim::event::{Event, EventHandler};

use color_eyre::eyre::Result;
use ratatui::{backend::CrosstermBackend, Terminal};

fn main() -> Result<()> {
    color_eyre::install()?;
    // Create app and initialize TUI
    let mut app = App::new();

    let backend = CrosstermBackend::new(std::io::stderr());
    let terminal = Terminal::new(backend)?;
    let events = EventHandler::new(1000 / 30);
    let mut tui = Tui::new(terminal, events);
    tui.enter().unwrap();

    // App main loop
    while !app.should_quit {
        // Handle events
        match tui.events.next()? {
            Event::Tick => tui.draw(&mut app).unwrap(),
            Event::Key(key_event) => update(&mut app, key_event).unwrap(),
            Event::Mouse(_) => {},
            Event::Resize(_, _) => {},
        };
    }

    // Quit the app
    tui.exit().unwrap();
    Ok(())
}