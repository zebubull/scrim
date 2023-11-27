use std::path::PathBuf;

use crate::{
    core::{App, ControlType, LookupResult, Selected, Tab},
    lookup::Lookup,
    player::{class::Class, skills::ProficiencyLevel},
};
use color_eyre::eyre::Result;
use crossterm::event::{KeyCode, KeyEvent};

/// Process the given key event and update that app's state accordingly.
pub fn update(app: &mut App, lookup: &mut Lookup, key_event: KeyEvent) -> Result<()> {
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
                    KeyCode::Tab if !app.current_tab().is_empty() => {
                        if let Some(Selected::TabItem) = app.selected {
                            app.complete_current_selection(lookup)?;
                            app.selected = Some(Selected::Completion(app.tab_scroll().get_line()));
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
            }
            Some(ControlType::CycleRecalc(value, min, max)) => {
                match key_event.code {
                    KeyCode::Char('j') => *value = std::cmp::max(min, value.saturating_sub(1)),
                    KeyCode::Char('k') => *value = std::cmp::min(max, value.saturating_add(1)),
                    _ => {}
                };
                app.player.recalculate();
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
                    Some(Selected::ItemLookup(_)) => Some(Selected::TabItem),
                    _ => None,
                };
                app.current_lookup = None;
                return Ok(());
            }
            KeyCode::Char('q') if app.current_lookup.is_none() => 'quit: {
                if let Some(Selected::Quitting) = app.selected {
                    break 'quit;
                }
                if app.selected.is_none() {
                    app.selected = Some(Selected::Quitting);
                    app.current_lookup = None;
                } else {
                    app.selected = None;
                }
                return Ok(());
            }
            KeyCode::Char('S') => app.save_player()?,
            KeyCode::Char('1') => app.update_tab(Tab::Notes)?,
            KeyCode::Char('2') => app.update_tab(Tab::Inventory)?,
            KeyCode::Char('3') => app.update_tab(Tab::Spells)?,
            _ => {}
        }

        match app.selected {
            Some(Selected::TopBarItem) => match key_event.code {
                KeyCode::Char('h') => app.index = app.index.saturating_sub(1),
                KeyCode::Char('l') => app.index = std::cmp::min(4, app.index + 1),
                KeyCode::Enter => app.editing = true,
                _ => {}
            },
            Some(Selected::StatItem) => match key_event.code {
                KeyCode::Char('k') => app.index = app.index.saturating_sub(1),
                KeyCode::Char('j') => app.index = std::cmp::min(5, app.index + 1),
                KeyCode::Enter => app.editing = true,
                _ => {}
            },
            Some(Selected::InfoItem) => match key_event.code {
                KeyCode::Char('h') => app.index = app.index.saturating_sub(1),
                KeyCode::Char('l') => app.index = std::cmp::min(6, app.index + 1),
                KeyCode::Char('r') if app.index == 0 => app.player.hp = app.player.max_hp,
                KeyCode::Enter => app.editing = true,
                _ => {}
            },
            Some(Selected::TabItem) => match key_event.code {
                KeyCode::Char('k') => app.tab_scroll_mut().scroll_up(1),
                KeyCode::Char('j') => app.tab_scroll_mut().scroll_down(1),
                KeyCode::Char('K') => app.tab_scroll_mut().scroll_up(10),
                KeyCode::Char('J') => app.tab_scroll_mut().scroll_down(10),
                KeyCode::Char('a') => {
                    app.append_item_to_tab()?;
                    app.editing = true;
                }
                KeyCode::Char('i') => {
                    app.insert_item_to_tab()?;
                    app.editing = true;
                }
                KeyCode::Char('d') if !app.current_tab().is_empty() => {
                    app.delete_item_from_tab()?;
                }
                KeyCode::Enter if !app.current_tab().is_empty() => {
                    app.editing = true;
                }
                KeyCode::Char('l') if !app.current_tab().is_empty() => {
                    app.lookup_current_selection(lookup)?
                }
                KeyCode::Tab if !app.current_tab().is_empty() => {
                    app.complete_current_selection(lookup)?;
                    app.selected = Some(Selected::Completion(app.tab_scroll().get_line()));
                    app.editing = false;
                }
                _ => {}
            },
            Some(Selected::Completion(_)) => match key_event.code {
                KeyCode::Char('k') => app.popup_scroll_mut().scroll_up(1),
                KeyCode::Char('j') => app.popup_scroll_mut().scroll_down(1),
                KeyCode::Char('K') => app.popup_scroll_mut().scroll_up(10),
                KeyCode::Char('J') => app.popup_scroll_mut().scroll_down(10),
                KeyCode::Enter => {
                    app.finish_completion();
                    app.current_lookup = None;
                    app.selected = Some(Selected::TabItem)
                }
                KeyCode::Char('q') => {
                    app.selected = Some(Selected::TabItem);
                    app.editing = true;
                    app.current_lookup = None;
                }
                _ => {}
            },
            Some(Selected::Quitting) => match key_event.code {
                KeyCode::Char('q' | 'n') => app.selected = None,
                KeyCode::Char('s') => app.quit(),
                KeyCode::Char('y') => {
                    if app.player.name.is_empty() {
                        app.player.name = String::from("player");
                    }
                    app.save_player()?;
                    app.quit();
                }
                _ => {}
            },
            Some(Selected::ItemLookup(_)) => match key_event.code {
                KeyCode::Char('k') => app.popup_scroll_mut().move_up(1),
                KeyCode::Char('j') => app.popup_scroll_mut().move_down(1),
                KeyCode::Char('K') => app.popup_scroll_mut().move_up(10),
                KeyCode::Char('J') => app.popup_scroll_mut().move_down(10),
                KeyCode::Char('q') => {
                    app.selected = Some(Selected::TabItem);
                    app.current_lookup = None;
                }
                _ => {}
            },
            Some(Selected::ClassLookup) => match key_event.code {
                KeyCode::Char('k') => app.popup_scroll_mut().move_up(1),
                KeyCode::Char('j') => app.popup_scroll_mut().move_down(1),
                KeyCode::Char('K') => app.popup_scroll_mut().move_up(10),
                KeyCode::Char('J') => app.popup_scroll_mut().move_down(10),
                KeyCode::Char('q') => {
                    app.selected = None;
                    app.current_lookup = None;
                }
                _ => {}
            },
            Some(Selected::SpellSlots) => {
                let idx = app.popup_scroll().get_line();
                let remaining = match app.player.class {
                    Class::Warlock => &mut app.player.spell_slots_remaining.warlock,
                    _ => &mut app.player.spell_slots_remaining[idx as usize],
                };
                let total = match app.player.class {
                    Class::Warlock => &mut app.player.spell_slots.warlock,
                    _ => &mut app.player.spell_slots[idx as usize],
                };
                match key_event.code {
                    KeyCode::Char('a') => *remaining = std::cmp::min(*remaining + 1, *total),
                    KeyCode::Char('x') => *remaining = remaining.saturating_sub(1),
                    KeyCode::Char('A') => *total += 1,
                    KeyCode::Char('X') => {
                        *total = total.saturating_sub(1);
                        *remaining = std::cmp::min(*remaining, *total);
                    }
                    KeyCode::Char('k') if app.player.class != Class::Warlock => {
                        app.popup_scroll_mut().scroll_up(1)
                    }
                    KeyCode::Char('j') if app.player.class != Class::Warlock => {
                        app.popup_scroll_mut().scroll_down(1)
                    }
                    KeyCode::Char('r') => {
                        app.player.spell_slots_remaining = app.player.spell_slots.clone();
                    }
                    _ => {}
                }
            }
            Some(Selected::Funds) => match key_event.code {
                KeyCode::Char('a') => {
                    let fundage = app.player.funds.nth_mut(app.popup_scroll().get_line());
                    *fundage += 1;
                }
                KeyCode::Char('x') => {
                    let fundage = app.player.funds.nth_mut(app.popup_scroll().get_line());
                    *fundage = fundage.saturating_sub(1);
                }
                KeyCode::Char('A') => {
                    let fundage = app.player.funds.nth_mut(app.popup_scroll().get_line());
                    *fundage += 10;
                }
                KeyCode::Char('X') => {
                    let fundage = app.player.funds.nth_mut(app.popup_scroll().get_line());
                    *fundage = fundage.saturating_sub(10);
                }
                KeyCode::Char('k') => app.popup_scroll_mut().scroll_up(1),
                KeyCode::Char('j') => app.popup_scroll_mut().scroll_down(1),
                _ => {}
            },
            Some(Selected::FreeLookup) => match key_event.code {
                KeyCode::Backspace => {
                    app.lookup_buffer.pop();
                }
                KeyCode::Char(c) => {
                    app.lookup_buffer.push(c);
                }
                KeyCode::Tab | KeyCode::Enter => {
                    app.current_lookup =
                        Some(app.get_completion(&app.lookup_buffer.clone(), lookup));
                    app.selected = Some(Selected::FreeLookupSelect);
                }
                _ => {}
            },
            Some(Selected::FreeLookupSelect) => match key_event.code {
                KeyCode::Char('k') => app.popup_scroll_mut().scroll_up(1),
                KeyCode::Char('j') => app.popup_scroll_mut().scroll_down(1),
                KeyCode::Char('K') => app.popup_scroll_mut().scroll_up(10),
                KeyCode::Char('J') => app.popup_scroll_mut().scroll_down(10),
                KeyCode::Char('q') => {
                    app.selected = None;
                    app.current_lookup = None;
                }
                KeyCode::Enter => {
                    app.selected = Some(Selected::ClassLookup);
                    let options = match app.current_lookup {
                        Some(LookupResult::Completion(ref vec)) => vec,
                        Some(LookupResult::Invalid(_)) => return Ok(()),
                        _ => unreachable!(),
                    };
                    if !options.is_empty() {
                        app.current_lookup = Some(LookupResult::Success(
                            options[app.popup_scroll().get_line() as usize].clone(),
                        ));
                        app.popup_scroll_mut().clear_max();
                    } else {
                        app.selected = None;
                        app.current_lookup = None;
                    }
                }
                _ => {}
            },
            Some(Selected::Proficiency) => {
                let idx = app.popup_scroll().get_line();
                match key_event.code {
                    KeyCode::Char('k') => app.popup_scroll_mut().scroll_up(1),
                    KeyCode::Char('j') => app.popup_scroll_mut().scroll_down(1),
                    KeyCode::Char('K') => app.popup_scroll_mut().scroll_up(10),
                    KeyCode::Char('J') => app.popup_scroll_mut().scroll_down(10),
                    KeyCode::Char('p') => {
                        app.player.skills[idx as usize] = ProficiencyLevel::Normal
                    }
                    KeyCode::Char('n') => app.player.skills[idx as usize] = ProficiencyLevel::None,
                    KeyCode::Char('e') => {
                        app.player.skills[idx as usize] = ProficiencyLevel::Double
                    }
                    KeyCode::Char('h') => app.player.skills[idx as usize] = ProficiencyLevel::Half,
                    KeyCode::Char('q') | KeyCode::Enter => {
                        app.selected = None;
                        app.current_lookup = None;
                    }
                    _ => {}
                }
            }
            Some(Selected::Load) => match key_event.code {
                KeyCode::Char('k') => app.popup_scroll_mut().scroll_up(1),
                KeyCode::Char('j') => app.popup_scroll_mut().scroll_down(1),
                KeyCode::Char('K') => app.popup_scroll_mut().scroll_up(10),
                KeyCode::Char('J') => app.popup_scroll_mut().scroll_down(10),
                KeyCode::Char('q') => {
                    app.selected = None;
                    app.current_lookup = None;
                }
                KeyCode::Enter => {
                    app.selected = None;
                    app.tab_scroll_mut().reset();
                    app.update_tab(Tab::Notes)?;
                    let p = match &app.current_lookup {
                        Some(LookupResult::Files(ref f)) => {
                            &f[app.popup_scroll().get_line() as usize]
                        }
                        _ => unreachable!(),
                    };
                    app.load_player(PathBuf::from(p))?;
                    app.current_lookup = None;
                }
                _ => {}
            },
            Some(Selected::Error) => match key_event.code {
                KeyCode::Char('k') => app.popup_scroll_mut().scroll_up(1),
                KeyCode::Char('j') => app.popup_scroll_mut().scroll_down(1),
                KeyCode::Char('K') => app.popup_scroll_mut().scroll_up(10),
                KeyCode::Char('J') => app.popup_scroll_mut().scroll_down(10),
                KeyCode::Char('q') | KeyCode::Enter => {
                    app.current_lookup = None;
                    app.selected = None;
                    app.error = None;
                }
                _ => {}
            },
            None => match key_event.code {
                KeyCode::Char('u') => {
                    app.index = 0;
                    app.selected = Some(Selected::TopBarItem)
                }
                KeyCode::Char('s') => {
                    app.index = 0;
                    app.selected = Some(Selected::StatItem)
                }
                KeyCode::Char('i') => {
                    app.index = 0;
                    app.selected = Some(Selected::InfoItem);
                }
                KeyCode::Char('t') => app.selected = Some(Selected::TabItem),
                KeyCode::Char('E') => {
                    app.popup_scroll_mut().reset();
                    app.popup_scroll_mut().set_max(9);
                    app.selected = Some(Selected::SpellSlots)
                }
                KeyCode::Char('F') => {
                    app.popup_scroll_mut().reset();
                    app.popup_scroll_mut().set_max(4);
                    app.selected = Some(Selected::Funds)
                }
                KeyCode::Char('P') => {
                    app.popup_scroll_mut().reset();
                    app.popup_scroll_mut().set_max(18);
                    app.selected = Some(Selected::Proficiency)
                }
                KeyCode::Char('C') => app.lookup_class(lookup)?,
                KeyCode::Char('R') => app.lookup_race(lookup)?,
                KeyCode::Char('L') => {
                    app.selected = Some(Selected::FreeLookup);
                    app.popup_scroll_mut().reset();
                    app.lookup_buffer.clear();
                }
                KeyCode::Char('[') => app.lookup_files()?,
                KeyCode::Char('k') => app.tab_scroll_mut().move_up(1),
                KeyCode::Char('j') => app.tab_scroll_mut().move_down(1),
                KeyCode::Char('K') => app.tab_scroll_mut().move_up(10),
                KeyCode::Char('J') => app.tab_scroll_mut().move_down(10),
                _ => {}
            },
        }
    }
    Ok(())
}
