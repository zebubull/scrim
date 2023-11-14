use std::{path::Path, rc::Rc};

use crate::{
    lookup::{Lookup, LookupEntry},
    player::Player,
    Cycle,
};
use color_eyre::eyre::{eyre, Result, WrapErr};
use strum_macros::Display;

/// An enum that represents a control as well as an index into that control's values, if it has any.
#[derive(Clone, Copy)]
pub enum Selected {
    /// An item in the top bar.
    TopBarItem(u32),
    /// An item in the stat block.
    StatItem(u32),
    /// An item in the player info bar.
    InfoItem(u32),
    /// A line in the tab panel.
    TabItem(u32),
    /// The quit menu is showing.
    Quitting,
    /// The lookup menu is showing.
    ///
    /// This holds a reference to the tab item that the lookup originated from.
    ItemLookup(u32),
}

/// An enum that represents the way in which a field can be modified by the user.
pub enum ControlType<'a> {
    /// A control type that is a text input from the user. A mutable reference
    /// to the text to be modified is provided.
    TextInput(&'a mut String),
    /// A control type that cycles numericaly upwards or downwards. A mutable
    /// reference to the number is provided, as well as a minimum and maximum
    /// value, respectively.
    Cycle(&'a mut u32, u32, u32),
    /// A control type that cycles upwards or downwards through non-numerical values.
    /// `prev` and `next` functions are given, respectively.
    CycleFn(fn(&mut App), fn(&mut App)),
}

#[derive(Clone, Copy, Default, Display)]
pub enum Tab {
    #[default]
    Notes,
    Inventory,
    Spells,
}

pub enum LookupResult {
    Success(Rc<LookupEntry>),
    Invalid(String),
}

/// A struct representing the current app state.
#[derive(Default)]
pub struct App {
    /// The player currently being viewed.
    pub player: Player,
    /// Whether the app should terminate at the next update cycle.
    pub should_quit: bool,
    /// Whether the user is currently editing a control.
    pub editing: bool,
    /// The currently selected pane of the user interface.
    pub selected: Option<Selected>,
    /// The amount of lines to scroll the current tab pane by.
    pub vscroll: u32,
    /// The current height of the tab viewport, in lines.
    pub viewport_height: u32,
    /// The currently selected tab.
    pub current_tab: Tab,
    /// The player path specified at startup, if it exists.
    pub path: Option<String>,
    /// The app's reference lookup table.
    pub lookup: Lookup,
    /// The amount of lines to scroll the reference lookup pane by.
    pub lookup_scroll: u32,
    /// The most recent lookup result, if it exists.
    pub current_lookup: Option<LookupResult>,
}

impl App {
    /// Create a new instance of the `App` struct. Currently aliases to `App::default()`.
    pub fn new() -> Self {
        Self::default()
    }

    /// Requests the application to exit by updating the should_quit value.
    pub fn quit(&mut self) {
        self.should_quit = true;
    }

    /// Attempts to load the player at the given file path.
    pub fn load_player(&mut self, path: &Path) -> Result<()> {
        let data = std::fs::read(path).wrap_err_with(|| {
            format!(
                "failed to load player from file `{}`",
                path.to_string_lossy()
            )
        })?;
        self.player = serde_json::from_slice(data.as_slice()).wrap_err_with(|| {
            format!(
                "player file `{}` could not be loaded, it may be corrupt",
                path.to_string_lossy()
            )
        })?;
        Ok(())
    }

    /// Saves the currently edited player.
    ///
    /// The app will try to save to the file name specified in the
    /// environment args. If no file was specified, it will create a
    /// new file with the same name as the player.
    pub fn save_player(&self) -> Result<()> {
        let data = serde_json::to_string(&self.player)?;
        let path = format!("{}", self.path.as_ref().unwrap_or(&self.player.name));
        std::fs::write(path, data)?;
        Ok(())
    }

    /// Update the app's viewport height cache.
    ///
    /// This method will recalculate the current tab scroll.
    pub fn update_viewport_height(&mut self, height: u16) -> Result<()> {
        // tab frame size
        self.viewport_height = height as u32 - 9;

        let len = self.current_tab().len() as u32;

        if self.vscroll + self.viewport_height >= len {
            self.vscroll = len.saturating_sub(1).saturating_sub(self.viewport_height);
        }

        Ok(())
    }

    /// Returns a reference to the data of the currently selected tab.
    pub fn current_tab(&self) -> &Vec<String> {
        use Tab::*;
        match self.current_tab {
            Notes => &self.player.notes,
            Inventory => &self.player.inventory,
            Spells => &self.player.spells,
        }
    }

    /// Returns a mutable reference to the data of the currently selected tab.
    pub fn current_tab_mut(&mut self) -> &mut Vec<String> {
        use Tab::*;
        match self.current_tab {
            Notes => &mut self.player.notes,
            Inventory => &mut self.player.inventory,
            Spells => &mut self.player.spells,
        }
    }

    /// Switches the current tab and recalculates the current tab scroll.
    pub fn update_tab(&mut self, tab: Tab) -> Result<()> {
        self.current_tab = tab;

        // Using update with zero will just recheck scroll bounds
        if let Some(Selected::TabItem(_)) = self.selected {
            self.update_item_scroll(0)?;
        } else {
            self.update_overview_scroll(0);
        }

        Ok(())
    }

    /// Adds an empty entry to the current tab.
    ///
    /// The entry is located after the currently selected item
    /// or at the first position if the current tab is empty.
    /// This method will also recalculate the current tab scroll.
    pub fn append_item_to_tab(&mut self) -> Result<()> {
        let mut item = match self.selected {
            Some(Selected::TabItem(item)) => item,
            _ => return Err(eyre!("cannot append while a tab is not selected")),
        } as usize;

        if self.current_tab().len() > 0 {
            item += 1;
        }

        self.current_tab_mut().insert(item, String::from(" "));
        self.selected = Some(Selected::TabItem(item as u32));

        self.refresh_scroll(item as u32);
        Ok(())
    }

    /// Adds an empty entry to the current tab.
    ///
    /// The entry is located before the currently selected item
    /// or at the first position if the current tab is empty.
    /// This method will also recalculate the current tab scroll.
    pub fn insert_item_to_tab(&mut self) -> Result<()> {
        let item = match self.selected {
            Some(Selected::TabItem(item)) => item,
            _ => return Err(eyre!("cannot insert while a tab is not selected")),
        } as usize;
        // The new item will be at the same index as the previously selected item, so
        // no need to change the selection
        self.current_tab_mut().insert(item, String::from(" "));

        self.refresh_scroll(item as u32);
        Ok(())
    }

    /// Remove the currently selected entry from the tab.
    ///
    /// This method does not check to make sure there is an entry
    /// to delete and will panic if the current tab is empty. It will
    /// also recalulate the current tab scroll.
    pub fn delete_item_from_tab(&mut self) -> Result<()> {
        let item = match self.selected {
            Some(Selected::TabItem(item)) => item,
            _ => return Err(eyre!("cannot delete while a tab is not selected")),
        } as usize;

        let tab = self.current_tab_mut();
        tab.remove(item);

        let new_idx = item.saturating_sub(1) as u32;

        if item >= tab.len() {
            self.selected = Some(Selected::TabItem(new_idx));
        }

        self.refresh_scroll(new_idx);
        Ok(())
    }

    /// Moves the current overview scroll value by the given amount of lines.
    pub fn update_overview_scroll(&mut self, amount: i32) {
        let len = self.current_tab().len() as u32;
        if len == 0 {
            self.vscroll = 0;
            return;
        }

        let max = len.saturating_sub(self.viewport_height);

        self.vscroll = std::cmp::min(self.vscroll.saturating_add_signed(amount), max);
    }

    /// Moves the current line scroll value and selected item by the given amount of lines.
    ///
    /// This method will throw an error if no tab is currently selected.
    pub fn update_item_scroll(&mut self, amount: i32) -> Result<()> {
        let item = match self.selected {
            Some(Selected::TabItem(item)) => item,
            _ => return Err(eyre!("cannot scroll item while a tab is not selected")),
        };

        let len = self.current_tab().len() as u32;

        if len == 0 {
            self.vscroll = 0;
            return Ok(());
        }

        let selected = std::cmp::min(item.saturating_add_signed(amount), len - 1);
        self.selected = Some(Selected::TabItem(selected));

        self.refresh_scroll(selected);

        Ok(())
    }

    /// Updates the current tab scroll if necessary to display the item
    /// on the given line.
    ///
    /// This should not be used in overview mode. Use `App::update_overview_scroll()` instead.
    fn refresh_scroll(&mut self, selected: u32) {
        if selected < self.vscroll {
            // If the current line is above the viewport, scroll up to it
            self.vscroll = selected;
        } else if selected >= self.vscroll + self.viewport_height {
            // If the current line is below the viewport, scroll down to it
            self.vscroll = selected - self.viewport_height + 1;
        }
    }

    /// Uses the current selected tab item to lookup a reference entry.
    ///
    /// This method does not perform any kind of caching.
    pub fn lookup_current_selection(&mut self) -> Result<()> {
        use Tab::*;
        let item = match self.selected {
            Some(Selected::TabItem(item)) => item,
            _ => return Ok(()),
        };

        let tab = match self.current_tab {
            Notes => &self.player.notes,
            Inventory => &self.player.inventory,
            Spells => &self.player.spells,
        };

        let text = tab[item as usize].trim().to_ascii_lowercase();
        if !self.lookup.loaded {
            self.lookup.load()?;
        }
        let lookup = self.lookup.get_entry(&text);

        // Probably shouldn't clone but the lifetimes were too confusing :(
        self.current_lookup = match lookup {
            Some(entry) => Some(LookupResult::Success(entry.clone())),
            None => Some(LookupResult::Invalid(text.clone())),
        };

        self.selected = Some(Selected::ItemLookup(item));
        self.lookup_scroll = 0;
        Ok(())
    }

    /// Get the control type associated with the currently selected item.
    pub fn get_selected_type(&mut self) -> Option<ControlType> {
        match self.selected {
            None | Some(Selected::Quitting) | Some(Selected::ItemLookup(_)) => None,
            Some(Selected::TopBarItem(idx)) => match idx {
                0 => Some(ControlType::TextInput(&mut self.player.name)),
                1 => Some(ControlType::CycleFn(
                    // Currently, there are no calculations made with the race so just
                    // raw setting it is fine.
                    |app| app.player.race = app.player.race.prev(),
                    |app| app.player.race = app.player.race.next(),
                )),
                2 => Some(ControlType::Cycle(&mut self.player.level, 1, 20)),
                3 => Some(ControlType::CycleFn(
                    |app| app.player.update_class(app.player.class.prev()),
                    |app| app.player.update_class(app.player.class.next()),
                )),
                4 => Some(ControlType::CycleFn(
                    // Just like race, there are no calculations made with the alignment
                    // so just raw setting it is fine.
                    |app| app.player.alignment = app.player.alignment.prev(),
                    |app| app.player.alignment = app.player.alignment.next(),
                )),
                _ => unreachable!(),
            },
            Some(Selected::StatItem(idx)) => {
                Some(ControlType::Cycle(self.player.stats.nth(idx), 1, 20))
            }
            Some(Selected::InfoItem(idx)) => match idx {
                0 => Some(ControlType::Cycle(
                    &mut self.player.hp,
                    0,
                    self.player.max_hp,
                )),
                1 => Some(ControlType::Cycle(
                    &mut self.player.max_hp,
                    1,
                    std::u32::MAX,
                )),
                2 => Some(ControlType::Cycle(
                    &mut self.player.temp_hp,
                    0,
                    std::u32::MAX,
                )),
                3 => Some(ControlType::Cycle(&mut self.player.ac, 0, 50)),
                4 => Some(ControlType::Cycle(&mut self.player.prof_bonus, 2, 6)),
                5 => Some(ControlType::Cycle(
                    &mut self.player.hit_dice_remaining,
                    0,
                    self.player.hit_dice,
                )),
                6 => Some(ControlType::CycleFn(
                    |app| app.player.background = app.player.background.prev(),
                    |app| app.player.background = app.player.background.next(),
                )),
                _ => unreachable!(),
            },
            Some(Selected::TabItem(idx)) => Some(ControlType::TextInput(match self.current_tab {
                Tab::Notes => &mut self.player.notes[idx as usize],
                Tab::Inventory => &mut self.player.inventory[idx as usize],
                Tab::Spells => &mut self.player.spells[idx as usize],
            })),
        }
    }
}
