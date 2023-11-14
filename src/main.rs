use std::env::args;
use std::path::{Path, PathBuf};

use scrim::app::App;
use scrim::event::{Event, EventHandler};
use scrim::lookup::Lookup;
use scrim::tui::Tui;
use scrim::update::update;

use color_eyre::eyre::Result;
use ratatui::{backend::CrosstermBackend, Terminal};

fn main() -> Result<()> {
    // Create app and initialize TUI
    color_eyre::install()?;
    let mut app = App::new();

    let args: Vec<_> = args().collect();
    let path = if args.len() > 1 { Some(&args[1]) } else { None };

    app.path = path.cloned();

    let mut lookup_path = PathBuf::new();
    #[cfg(debug_assertions)]
    {
        lookup_path.push("lookups/");
    }

    #[cfg(not(debug_assertions))]
    {
        lookup_path.push(home::home_dir().unwrap());
        lookup_path.push(".scrim/")
    }
    app.lookup = Lookup::new(lookup_path);

    let backend = CrosstermBackend::new(std::io::stdout());
    let terminal = Terminal::new(backend)?;
    app.update_viewport_height(terminal.size()?.height)?;
    let events = EventHandler::new();
    let mut tui = Tui::new(terminal, events);

    // Load player data
    if path.is_some() {
        let path = Path::new(path.as_ref().unwrap());
        if path.exists() {
            app.load_player(path)?;
        }
    }

    // App main loop
    tui.enter()?;
    while !app.should_quit {
        tui.draw(&mut app)?;
        // Handle events
        match tui.events.next().unwrap() {
            // Tick event is currently unused
            Event::Tick => {}
            Event::Key(key_event) => update(&mut app, key_event)?,
            Event::Mouse(_) => {}
            Event::Resize(_, y) => app.update_viewport_height(y)?,
        };
    }

    // Quit the app
    tui.exit().unwrap();

    if path.is_some() || !app.player.name.is_empty() {
        app.save_player()?;
    }

    Ok(())
}
