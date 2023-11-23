use crate::{
    app::{App, ControlType, LookupResult, Selected, Tab},
    lookup::Lookup,
    player::{class::Class, skills::ProficiencyLevel},
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
                    Some(Selected::ItemLookup(item)) => Some(Selected::TabItem(item)),
                    _ => None,
                };
                app.current_lookup = None;
                app.popup_scroll = 0;
                return Ok(());
            }
            KeyCode::Char('q') if app.current_lookup.is_none() => 'quit: {
                if let Some(Selected::Quitting) = app.selected {
                    break 'quit;
                }
                if app.selected.is_none() {
                    app.selected = Some(Selected::Quitting);
                    app.current_lookup = None;
                    app.popup_scroll = 0;
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
                KeyCode::Char('d') if !app.current_tab().is_empty() => {
                    app.delete_item_from_tab()?;
                }
                KeyCode::Enter if !app.current_tab().is_empty() => {
                    app.editing = true;
                }
                KeyCode::Char('l') if !app.current_tab().is_empty() => {
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
            Some(Selected::SpellSlots(idx)) => {
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
                    KeyCode::Char('x') => {
                        *remaining = remaining.saturating_sub(1);
                    }
                    KeyCode::Char('A') => {
                        *total += 1;
                    }
                    KeyCode::Char('X') => {
                        *total = total.saturating_sub(1);
                        *remaining = std::cmp::min(*remaining, *total);
                    }
                    KeyCode::Char('j') if app.player.class != Class::Warlock => {
                        let new_idx = std::cmp::min(8, idx + 1);
                        app.selected = Some(Selected::SpellSlots(new_idx));
                    }
                    KeyCode::Char('k') if app.player.class != Class::Warlock => {
                        app.selected = Some(Selected::SpellSlots(idx.saturating_sub(1)));
                    }
                    KeyCode::Char('r') => {
                        app.player.spell_slots_remaining = app.player.spell_slots.clone();
                    }
                    _ => {}
                }
            }
            Some(Selected::Funds(idx)) => match key_event.code {
                KeyCode::Char('a') => {
                    let fundage = app.player.funds.nth_mut(idx);
                    *fundage += 1;
                }
                KeyCode::Char('x') => {
                    let fundage = app.player.funds.nth_mut(idx);
                    *fundage = fundage.saturating_sub(1);
                }
                KeyCode::Char('A') => {
                    let fundage = app.player.funds.nth_mut(idx);
                    *fundage += 10;
                }
                KeyCode::Char('X') => {
                    let fundage = app.player.funds.nth_mut(idx);
                    *fundage = fundage.saturating_sub(10);
                }
                KeyCode::Char('j') if app.player.class != Class::Warlock => {
                    let new_idx = std::cmp::min(3, idx + 1);
                    app.selected = Some(Selected::Funds(new_idx));
                }
                KeyCode::Char('k') if app.player.class != Class::Warlock => {
                    app.selected = Some(Selected::Funds(idx.saturating_sub(1)));
                }
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
                    app.current_lookup = Some(app.get_completion(&app.lookup_buffer, lookup));
                    app.popup_scroll = 0;
                    app.selected = Some(Selected::FreeLookupSelect(0));
                }
                _ => {}
            },
            Some(Selected::FreeLookupSelect(idx)) => match key_event.code {
                KeyCode::Char('j') => app.update_popup_scroll(1)?,
                KeyCode::Char('k') => app.update_popup_scroll(-1)?,
                KeyCode::Char('J') => app.update_popup_scroll(10)?,
                KeyCode::Char('K') => app.update_popup_scroll(-10)?,
                KeyCode::Char('q') => {
                    app.selected = None;
                    app.current_lookup = None;
                }
                KeyCode::Enter => {
                    app.selected = Some(Selected::ClassLookup);
                    let options = match app.current_lookup {
                        Some(LookupResult::Completion(ref vec)) => vec,
                        _ => unreachable!(),
                    };
                    if !options.is_empty() {
                        app.current_lookup =
                            Some(LookupResult::Success(options[idx as usize].clone()))
                    } else {
                        app.selected = None;
                        app.current_lookup = None;
                    }
                }
                _ => {}
            },
            Some(Selected::Proficiency(idx)) => match key_event.code {
                KeyCode::Char('j') => {
                    let new_idx = std::cmp::min(17, idx + 1);
                    app.selected = Some(Selected::Proficiency(new_idx));
                    app.popup_scroll =
                        App::calculate_scroll(app.popup_scroll, new_idx, app.popup_height);
                }
                KeyCode::Char('k') => {
                    let new_idx = idx.saturating_sub(1);
                    app.selected = Some(Selected::Proficiency(new_idx));
                    app.popup_scroll =
                        App::calculate_scroll(app.popup_scroll, new_idx, app.popup_height);
                }
                KeyCode::Char('J') => {
                    let new_idx = std::cmp::min(17, idx + 10);
                    app.selected = Some(Selected::Proficiency(new_idx));
                    app.popup_scroll =
                        App::calculate_scroll(app.popup_scroll, new_idx, app.popup_height);
                }
                KeyCode::Char('K') => {
                    let new_idx = idx.saturating_sub(10);
                    app.selected = Some(Selected::Proficiency(new_idx));
                    app.popup_scroll =
                        App::calculate_scroll(app.popup_scroll, new_idx, app.popup_height);
                }
                KeyCode::Char('p') => app.player.skills[idx as usize] = ProficiencyLevel::Normal,
                KeyCode::Char('n') => app.player.skills[idx as usize] = ProficiencyLevel::None,
                KeyCode::Char('e') => app.player.skills[idx as usize] = ProficiencyLevel::Double,
                KeyCode::Char('h') => app.player.skills[idx as usize] = ProficiencyLevel::Half,
                KeyCode::Char('q') | KeyCode::Enter => {
                    app.selected = None;
                    app.current_lookup = None;
                }
                _ => {}
            },
            Some(Selected::Load(idx)) => match key_event.code {
                KeyCode::Char('j') => app.update_popup_scroll(1)?,
                KeyCode::Char('k') => app.update_popup_scroll(-1)?,
                KeyCode::Char('J') => app.update_popup_scroll(10)?,
                KeyCode::Char('K') => app.update_popup_scroll(-10)?,
                KeyCode::Char('q') => {
                    app.selected = None;
                    app.current_lookup = None;
                }
                KeyCode::Enter => {
                    app.selected = None;
                    app.popup_scroll = 0;
                    app.vscroll = 0;
                    app.update_tab(Tab::Notes)?;
                    let p = match &app.current_lookup {
                        Some(LookupResult::Files(ref f)) => &f[idx as usize],
                        _ => unreachable!(),
                    };
                    app.load_player(&std::path::PathBuf::from(p))?;
                    app.current_lookup = None;
                }
                _ => {}
            },
            None => match key_event.code {
                KeyCode::Char('u') => app.selected = Some(Selected::TopBarItem(0)),
                KeyCode::Char('s') => app.selected = Some(Selected::StatItem(0)),
                KeyCode::Char('i') => app.selected = Some(Selected::InfoItem(0)),
                KeyCode::Char('t') => app.selected = Some(Selected::TabItem(app.vscroll)),
                KeyCode::Char('E') => app.selected = Some(Selected::SpellSlots(0)),
                KeyCode::Char('F') => app.selected = Some(Selected::Funds(0)),
                KeyCode::Char('P') => app.selected = Some(Selected::Proficiency(0)),
                KeyCode::Char('C') => app.lookup_class(lookup),
                KeyCode::Char('R') => app.lookup_race(lookup),
                KeyCode::Char('L') => {
                    app.selected = Some(Selected::FreeLookup);
                    app.lookup_buffer.clear();
                }
                KeyCode::Char('[') => app.lookup_files()?,
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
