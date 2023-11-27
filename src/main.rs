use std::env::args;
use std::path::PathBuf;

use scrim::core::App;
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

    println!("loading lookup tables...");
    let mut lookup = Lookup::new(lookup_path);

    lookup.load()?;

    let backend = CrosstermBackend::new(std::io::stdout());
    let terminal = Terminal::new(backend)?;
    app.update_viewport_height(terminal.size()?.height);
    const AUTOSAVE_MINS: u64 = 5;
    let events = EventHandler::new(AUTOSAVE_MINS * 60 * 1000);
    let mut tui = Tui::new(terminal, events);

    // Load player data
    if path.is_some() {
        let path = PathBuf::from(path.as_ref().unwrap());
        if path.exists() {
            match app.load_player(path) {
                Ok(_) => {}
                Err(e) => app.show_error(strip_ansi_escapes::strip_str(&format!("{:?}", e))),
            }
        }
    }

    // App main loop
    tui.enter()?;
    while !app.should_quit {
        tui.draw(&mut app)?;
        // Handle events
        let res = match tui.events.next().unwrap() {
            Event::Tick => app.save_player(),
            Event::Key(key_event) => update(&mut app, &lookup, key_event),
            Event::Mouse(_) => Ok(()),
            Event::Resize(_, y) => {
                app.update_viewport_height(y);
                Ok(())
            }
        };

        match res {
            Ok(_) => {}
            Err(e) => app.show_error(strip_ansi_escapes::strip_str(&format!("{:?}", e))),
        }
    }

    // Quit the app
    tui.exit().unwrap();

    Ok(())
}
