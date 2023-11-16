use crate::{
    app::{App, ControlType, Selected, Tab},
    lookup::Lookup,
};
use color_eyre::eyre::Result;
use crossterm::event::{KeyCode, KeyEvent};

/// Process the given key event and update that app's state accordingly.
pub fn update(app: &mut App, lookup: &Lookup, key_event: KeyEvent) -> Result<()> {
    if app.editing {
        if key_event.code == KeyCode::Esc || key_event.code == KeyCode::Enter {
            app.editing = false;
            return Ok(());
        }

        match app.get_selected_type() {
            Some(ControlType::TextInput(text)) => {
                match key_event.code {
                    KeyCode::Backspace => {
                        text.pop();
                    }
                    KeyCode::Char(c) => {
                        text.push(c);
                    }
                    KeyCode::Tab => {
                        if let Some(Selected::TabItem(item)) = app.selected {
                            app.complete_current_selection(lookup)?;
                            app.selected = Some(Selected::Completion(0, item));
                            app.editing = false;
                        }
                    }
                    _ => {}
                };
            }
            Some(ControlType::Cycle(value, min, max)) => {
                match key_event.code {
                    KeyCode::Char('j') => *value = std::cmp::max(min, value.saturating_sub(1)),
                    KeyCode::Char('k') => *value = std::cmp::min(max, value.saturating_add(1)),
                    _ => {}
                };

                // Stats and may have changed so we need to do this. It might
                // be better to change stat and level to CycleFn but i'm too lazy
                // to check if that's faster and the calculations are relatively light
                app.player.update_stat_dependants();
                app.player.update_level_dependants();
            }
            Some(ControlType::CycleFn(prev, next)) => {
                match key_event.code {
                    // As of right now, all of the cycles should be reversed for natural scrolling
                    KeyCode::Char('j') => next(app),
                    KeyCode::Char('k') => prev(app),
                    _ => {}
                };
            }
            None => unreachable!(),
        };
    } else {
        match key_event.code {
            KeyCode::Esc => {
                // Properly retain the tab panel item if the lookup menu is closing.
                app.selected = match app.selected {
                    Some(Selected::ItemLookup(item)) => Some(Selected::TabItem(item)),
                    _ => None,
                };
                app.current_lookup = None;
                return Ok(());
            }
            KeyCode::Char('q') if app.current_lookup.is_none() => 'quit: {
                if let Some(Selected::Quitting) = app.selected {
                    break 'quit;
                }
                app.selected = Some(Selected::Quitting);
                return Ok(());
            }
            KeyCode::Char('S') => app.save_player()?,
            KeyCode::Char('1') => app.update_tab(Tab::Notes)?,
            KeyCode::Char('2') => app.update_tab(Tab::Inventory)?,
            KeyCode::Char('3') => app.update_tab(Tab::Spells)?,
            _ => {}
        }

        match app.selected {
            Some(Selected::TopBarItem(item)) => match key_event.code {
                KeyCode::Char('h') => {
                    let item = item.saturating_sub(1);
                    app.selected = Some(Selected::TopBarItem(item));
                }
                KeyCode::Char('l') => {
                    let item = std::cmp::min(4, item + 1);
                    app.selected = Some(Selected::TopBarItem(item));
                }
                KeyCode::Enter => {
                    app.editing = true;
                }
                _ => {}
            },
            Some(Selected::StatItem(item)) => match key_event.code {
                KeyCode::Char('k') => {
                    let item = item.saturating_sub(1);
                    app.selected = Some(Selected::StatItem(item));
                }
                KeyCode::Char('j') => {
                    let item = std::cmp::min(5, item + 1);
                    app.selected = Some(Selected::StatItem(item));
                }
                KeyCode::Enter => {
                    app.editing = true;
                }
                _ => {}
            },
            Some(Selected::InfoItem(item)) => match key_event.code {
                KeyCode::Char('h') => {
                    let item = item.saturating_sub(1);
                    app.selected = Some(Selected::InfoItem(item));
                }
                KeyCode::Char('l') => {
                    let item = std::cmp::min(6, item + 1);
                    app.selected = Some(Selected::InfoItem(item));
                }
                KeyCode::Char('r') => {
                    if item == 0 {
                        app.player.hp = app.player.max_hp;
                    }
                }
                KeyCode::Enter => {
                    app.editing = true;
                }
                _ => {}
            },
            Some(Selected::TabItem(item)) => match key_event.code {
                KeyCode::Char('k') => app.update_item_scroll(-1)?,
                KeyCode::Char('j') => app.update_item_scroll(1)?,
                KeyCode::Char('K') => app.update_item_scroll(-10)?,
                KeyCode::Char('J') => app.update_item_scroll(10)?,
                KeyCode::Char('a') => {
                    app.append_item_to_tab()?;
                    app.editing = true;
                }
                KeyCode::Char('i') => {
                    app.insert_item_to_tab()?;
                    app.editing = true;
                }
                KeyCode::Char('d') if app.current_tab().len() > 0 => {
                    app.delete_item_from_tab()?;
                }
                KeyCode::Enter if app.current_tab().len() > 0 => {
                    app.editing = true;
                }
                KeyCode::Char('l') if app.current_tab().len() > 0 => {
                    app.lookup_current_selection(lookup)
                }
                KeyCode::Tab => {
                    app.complete_current_selection(lookup)?;
                    app.selected = Some(Selected::Completion(0, item));
                    app.editing = false;
                }
                _ => {}
            },
            Some(Selected::Completion(_, tab_idx)) => match key_event.code {
                KeyCode::Char('j') => app.update_popup_scroll(1)?,
                KeyCode::Char('k') => app.update_popup_scroll(-1)?,
                KeyCode::Char('J') => app.update_popup_scroll(10)?,
                KeyCode::Char('K') => app.update_popup_scroll(-10)?,
                KeyCode::Enter => {
                    app.finish_completion();
                    app.current_lookup = None;
                    app.selected = Some(Selected::TabItem(tab_idx))
                }
                KeyCode::Char('q') => {
                    app.selected = Some(Selected::TabItem(tab_idx));
                    app.editing = true;
                    app.current_lookup = None;
                }
                _ => {}
            },
            Some(Selected::Quitting) => match key_event.code {
                KeyCode::Char('n') => app.selected = None,
                KeyCode::Char('q') => app.quit(),
                KeyCode::Char('y') => {
                    if app.player.name.is_empty() {
                        app.player.name = String::from("player");
                    }
                    app.save_player()?;
                    app.quit();
                }
                _ => {}
            },
            Some(Selected::ItemLookup(tab_idx)) => match key_event.code {
                KeyCode::Char('j') => app.update_popup_overview_scroll(1),
                KeyCode::Char('k') => app.update_popup_overview_scroll(-1),
                KeyCode::Char('J') => app.update_popup_overview_scroll(10),
                KeyCode::Char('K') => app.update_popup_overview_scroll(-10),
                KeyCode::Char('q') => {
                    app.selected = Some(Selected::TabItem(tab_idx));
                    app.current_lookup = None;
                }
                _ => {}
            },
            Some(Selected::ClassLookup) => match key_event.code {
                KeyCode::Char('j') => app.update_popup_overview_scroll(1),
                KeyCode::Char('k') => app.update_popup_overview_scroll(-1),
                KeyCode::Char('J') => app.update_popup_overview_scroll(10),
                KeyCode::Char('K') => app.update_popup_overview_scroll(-10),
                KeyCode::Char('q') => {
                    app.selected = None;
                    app.current_lookup = None;
                }
                _ => {}
            },
            None => match key_event.code {
                KeyCode::Char('u') => app.selected = Some(Selected::TopBarItem(0)),
                KeyCode::Char('s') => app.selected = Some(Selected::StatItem(0)),
                KeyCode::Char('i') => app.selected = Some(Selected::InfoItem(0)),
                KeyCode::Char('t') => app.selected = Some(Selected::TabItem(app.vscroll)),
                KeyCode::Char('C') => app.lookup_class(&lookup),
                KeyCode::Char('k') => app.update_overview_scroll(-1),
                KeyCode::Char('j') => app.update_overview_scroll(1),
                KeyCode::Char('K') => app.update_overview_scroll(-10),
                KeyCode::Char('J') => app.update_overview_scroll(10),
                _ => {}
            },
        }
    }
    Ok(())
}
