use std::cmp::{min, max};
use crossterm::event::{KeyCode, KeyEvent};
use crate::app::{App, Selected, ControlType, Tab};
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

        match key_event.code {
            KeyCode::Char('q') => {
                app.should_quit = true;
                return Ok(());
            },
            KeyCode::Esc => {
                app.selected = None;
                return Ok(());
            },
            KeyCode::Char('S') => app.save_player()?,
            KeyCode::Char('1') => app.update_tab(Tab::Notes),
            KeyCode::Char('2') => app.update_tab(Tab::Inventory),
            KeyCode::Char('3') => app.update_tab(Tab::Spells),
            _ => {}
        }

        match app.selected {
            Some(Selected::TopBarItem(item)) => {
                match key_event.code {
                    KeyCode::Char('h') => { 
                        let item = max(0, item - 1);
                        app.selected = Some(Selected::TopBarItem(item));
                    },
                    KeyCode::Char('l') => {
                        let item = min(4, item + 1);
                        app.selected = Some(Selected::TopBarItem(item));
                    },
                    KeyCode::Enter => { app.editing = true; },
                    _ => {}
                }
            },
            Some(Selected::StatItem(item)) => {
                match key_event.code {
                    KeyCode::Char('k') => { 
                        let item = max(0, item - 1);
                        app.selected = Some(Selected::StatItem(item));
                    },
                    KeyCode::Char('j') => {
                        let item = min(5, item + 1);
                        app.selected = Some(Selected::StatItem(item));
                    },
                    KeyCode::Enter => { app.editing = true; },
                    _ => {}
                }
            },
            Some(Selected::InfoItem(item)) => {
                match key_event.code {
                    KeyCode::Char('h') => { 
                        let item = max(0, item - 1);
                        app.selected = Some(Selected::InfoItem(item));
                    },
                    KeyCode::Char('l') => {
                        let item = min(6, item + 1);
                        app.selected = Some(Selected::InfoItem(item));
                    },
                    KeyCode::Enter => { app.editing = true; },
                    _ => {}
                }
            },
            Some(Selected::TabItem(item)) => {
                match key_event.code {
                    KeyCode::Char('k') => { 
                        let item = max(0, item - 1);
                        app.selected = Some(Selected::TabItem(item));
                        if (item as u16) < app.vscroll {
                            app.vscroll -= 1;
                        }
                    },
                    KeyCode::Char('j') => {
                        let item = min(app.current_tab_len() as i16 - 1, item + 1);
                        app.selected = Some(Selected::TabItem(item));
                        if item as u16 >= app.viewport_height + app.vscroll {
                            app.vscroll += 1;
                        }
                    },
                    KeyCode::Char('a') => {
                        app.add_item_to_tab()?;
                        app.editing = true;
                    },
                    KeyCode::Char('i') if app.can_edit_tab() => {
                        app.insert_item_to_tab()?;
                        app.editing = true;
                    },
                    KeyCode::Char('d') if app.can_edit_tab() => {
                        app.delete_item_from_tab()?;
                    }
                    KeyCode::Enter if app.can_edit_tab() => { app.editing = true; },
                    _ => {}
                }
            }
            None => {
                match key_event.code {
                    KeyCode::Char('b') => app.selected = Some(Selected::TopBarItem(0)),
                    KeyCode::Char('s') => app.selected = Some(Selected::StatItem(0)),
                    KeyCode::Char('i') => app.selected = Some(Selected::InfoItem(0)),
                    KeyCode::Char('t') => app.selected = Some(Selected::TabItem(app.vscroll as i16)),
                    KeyCode::Char('k') => { 
                        app.vscroll = app.vscroll.checked_sub(1).unwrap_or(0);
                    },
                    KeyCode::Char('j') => {
                        app.vscroll = std::cmp::min(app.vscroll + 1, app.current_tab_len().checked_sub(app.viewport_height as usize).unwrap_or(0) as u16);
                    },
                    _ => {}
                }
            }
        }

        app.update_selected_type();
    }
    Ok(())
}