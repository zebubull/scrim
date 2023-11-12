use std::env::args;
use std::path::Path;

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

    let args: Vec<_> = args().collect();
    let path = if args.len() > 1 {
        Some(args[1..].join(" "))
    } else {
        None
    };

    app.path = path.clone();

    let backend = CrosstermBackend::new(std::io::stdout());
    let terminal = Terminal::new(backend)?;
    app.update_viewport_height(terminal.size()?.height);
    let events = EventHandler::new(1000 / 30);
    let mut tui = Tui::new(terminal, events);

    // Load player data
    if path.is_some() {
        let file = &format!("{}.player", path.as_ref().unwrap());
        let path = Path::new(file);
        if path.exists() {
            app.load_player(path)?;
        }
    }

    // App main loop
    tui.enter()?;
    while !app.should_quit {
        // Handle events
        match tui.events.next().unwrap() {
            Event::Tick => tui.draw(&mut app)?,
            Event::Key(key_event) => update(&mut app, key_event)?,
            Event::Mouse(_) => {},
            Event::Resize(_, y) => app.update_viewport_height(y),
        };
    }

    // Quit the app
    tui.exit().unwrap();

    if path.is_some() || !app.player.name.is_empty() {
        app.save_player()?;
    }

    Ok(())
}