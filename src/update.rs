use std::cmp::{min, max};
use crossterm::event::{KeyCode, KeyEvent};
use crate::app::{App, Selected, ControlType};
use color_eyre::eyre::Result;

pub fn update(app: &mut App, key_event: KeyEvent) -> Result<()> {
    if app.editing {
        if key_event.code == KeyCode::Esc || key_event.code == KeyCode::Enter {
            app.editing = false;
            return Ok(());
        }

        use ControlType::*;
        match app.control_type {
            Some(TextInput) => {
                let text = app.get_current_string()?;
                match key_event.code {
                    KeyCode::Backspace => { text.pop(); },
                    KeyCode::Char(c) => { text.push(c); },
                    _ => {}
                };
            },
            Some(NextPrev) => {
                match key_event.code {
                    KeyCode::Char('j') => { app.cycle_current_prev()? },
                    KeyCode::Char('k') => { app.cycle_current_next()? },
                    _ => {}
                };
            },
            None => unreachable!(),
        };
    } else {
        if key_event.code == KeyCode::Char('q') {
            app.should_quit = true;
            return Ok(());
        }

        if key_event.code == KeyCode::Esc {
            app.selected = None;
            return Ok(());
        }

        match app.selected {
            Some(Selected::TopBarItem(idx)) => {
                match key_event.code {
                    KeyCode::Char('h') => { 
                        let idx = max(0, idx - 1);
                        app.selected = Some(Selected::TopBarItem(idx));
                    },
                    KeyCode::Char('l') => {
                        let idx = min(4, idx + 1);
                        app.selected = Some(Selected::TopBarItem(idx));
                    },
                    KeyCode::Enter => { app.editing = true; },
                    _ => {}
                }
            },
            Some(Selected::StatItem(idx)) => {
                match key_event.code {
                    KeyCode::Char('k') => { 
                        let idx = max(0, idx - 1);
                        app.selected = Some(Selected::StatItem(idx));
                    },
                    KeyCode::Char('j') => {
                        let idx = min(5, idx + 1);
                        app.selected = Some(Selected::StatItem(idx));
                    },
                    KeyCode::Enter => { app.editing = true; },
                    _ => {}
                }
            },
            Some(Selected::InfoItem(idx)) => {
                match key_event.code {
                    KeyCode::Char('h') => { 
                        let idx = max(0, idx - 1);
                        app.selected = Some(Selected::InfoItem(idx));
                    },
                    KeyCode::Char('l') => {
                        let idx = min(1, idx + 1);
                        app.selected = Some(Selected::InfoItem(idx));
                    },
                    KeyCode::Enter => { app.editing = true; },
                    _ => {}
                }
            }
            None => {
                match key_event.code {
                    KeyCode::Char('b') => app.selected = Some(Selected::TopBarItem(0)),
                    KeyCode::Char('s') => app.selected = Some(Selected::StatItem(0)),
                    KeyCode::Char('i') => app.selected = Some(Selected::InfoItem(0)),
                    _ => {}
                }
            }
        }

        app.update_selected_type();
    }
    Ok(())
}