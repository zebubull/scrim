use crossterm::event::{KeyCode, KeyEvent};

use crate::app::App;

pub fn update(app: &mut App, key_event: KeyEvent) {
    match key_event.code {
        KeyCode::Char('q') => app.quit(),
        KeyCode::Char('j') => app.dec_counter(),
        KeyCode::Char('k') => app.inc_counter(),
        _ => {},
    };
}