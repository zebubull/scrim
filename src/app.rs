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
    /// A line in the tab panel.``
    TabItem(u32),
    /// The quit menu is showing.
    Quitting,
    /// The lookup menu is showing.
    ///
    /// This holds a reference to the tab item that the lookup originated from.
    ItemLookup(u32),
    /// The lookup menu is showing the player's current class or the player's race.
    ClassLookup,
    /// The completion menu is showing.
    ///
    /// This holds a reference to the currently selected item and the tab item that
    /// the completion frame originated from
    Completion(u32, u32),
    /// The spell slots popup is showing.
    SpellSlots(u32),
    /// The money popup is showing.
    Funds(u32),
    /// The free lookup menu is showing.
    FreeLookup,
    /// The free lookup select menu is showing.
    FreeLookupSelect(u32),
    /// The proficiency menu is showing
    Proficiency(u32),
    /// The load menu is showing
    Load(u32),
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
    Completion(Vec<Rc<LookupEntry>>),
    Files(Vec<String>),
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
    /// The amount of lines to scroll the popup pane by.
    pub popup_scroll: u32,
    /// The height of the popup panel
    pub popup_height: u32,
    /// The most recent lookup result, if it exists.
    pub current_lookup: Option<LookupResult>,
    /// The current free lookup buffer
    pub lookup_buffer: String,
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
        self.path = Some(path.to_string_lossy().to_string());
        Ok(())
    }

    /// Saves the currently edited player.
    ///
    /// The app will try to save to the file name specified in the
    /// environment args. If no file was specified, it will create a
    /// new file with the same name as the player.
    pub fn save_player(&self) -> Result<()> {
        let data = serde_json::to_string(&self.player)?;
        let path = format!(
            "{}",
            self.path
                .as_ref()
                .unwrap_or(&format!("{}.player", self.player.name))
        );
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

        self.vscroll = App::calculate_scroll(self.vscroll, item as u32, self.viewport_height);
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

        self.vscroll = App::calculate_scroll(self.vscroll, item as u32, self.viewport_height);
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

        self.vscroll = App::calculate_scroll(self.vscroll, new_idx, self.viewport_height);
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

        self.vscroll = App::calculate_scroll(self.vscroll, selected, self.viewport_height);

        Ok(())
    }

    /// Move the current popup scroll and selected item by the given amount.
    pub fn update_popup_scroll(&mut self, amount: i32) -> Result<()> {
        let mut selected = match self.selected {
            Some(Selected::Completion(item, _)) => item,
            Some(Selected::FreeLookupSelect(item)) => item,
            Some(Selected::Load(item)) => item,
            _ => return Err(eyre!("current selection does not allow popup scroll")),
        };

        let num_entries = match &self.current_lookup {
            Some(LookupResult::Completion(v)) => v.len(),
            Some(LookupResult::Files(v)) => v.len(),
            Some(LookupResult::Invalid(_)) => return Ok(()),
            _ => {
                return Err(eyre!(
                    "current lookup does not support line-by-line scrolling"
                ))
            }
        } as u32;

        selected = selected.saturating_add_signed(amount).min(num_entries - 1);

        self.selected = match self.selected {
            Some(Selected::Completion(_, tab_item)) => {
                Some(Selected::Completion(selected, tab_item))
            }
            Some(Selected::FreeLookupSelect(_)) => Some(Selected::FreeLookupSelect(selected)),
            Some(Selected::Load(_)) => Some(Selected::Load(selected)),
            _ => unreachable!(),
        };

        self.popup_scroll = App::calculate_scroll(self.popup_scroll, selected, self.popup_height);

        Ok(())
    }

    /// Move the current popup overview scroll by the given amount.
    pub fn update_popup_overview_scroll(&mut self, amount: i32) {
        self.popup_scroll = self.popup_scroll.saturating_add_signed(amount);
    }

    /// Calculate the correct scroll value given the current scroll, selection, and frame height.
    pub fn calculate_scroll(scroll: u32, selected: u32, height: u32) -> u32 {
        if selected < scroll {
            // If the current line is above the viewport, scroll up to it
            return selected;
        } else if selected >= scroll + height {
            // If the current line is below the viewport, scroll down to it
            return selected - height + 1;
        } else {
            return scroll;
        }
    }

    /// Uses the current selected tab item to lookup a reference entry.
    ///
    /// This method does not perform any kind of caching.
    pub fn lookup_current_selection(&mut self, lookup: &Lookup) {
        use Tab::*;
        let item = match self.selected {
            Some(Selected::TabItem(item)) => item,
            _ => return,
        };

        let tab = match self.current_tab {
            Notes => &self.player.notes,
            Inventory => &self.player.inventory,
            Spells => &self.player.spells,
        };

        let text = tab[item as usize].trim().to_ascii_lowercase();
        let lookup = lookup.get_entry(&text);

        // Probably shouldn't clone but the lifetimes were too confusing :(
        self.current_lookup = match lookup {
            Some(entry) => Some(LookupResult::Success(entry.clone())),
            None => Some(LookupResult::Invalid(text.clone())),
        };

        self.selected = Some(Selected::ItemLookup(item));
        self.popup_scroll = 0;
    }

    pub fn lookup_class(&mut self, lookup: &Lookup) {
        let text = self.player.class.to_string().to_lowercase();
        let lookup = lookup.get_entry(&text);

        // Probably shouldn't clone but the lifetimes were too confusing :(
        self.current_lookup = match lookup {
            Some(entry) => Some(LookupResult::Success(entry.clone())),
            None => Some(LookupResult::Invalid(text.clone())),
        };

        self.selected = Some(Selected::ClassLookup);
        self.popup_scroll = 0;
    }

    /// Lookup the player's current race.
    pub fn lookup_race(&mut self, lookup: &Lookup) {
        let text = self.player.race.to_lookup_string();
        let lookup = lookup.get_entry(&text);

        // Probably shouldn't clone but the lifetimes were too confusing :(
        self.current_lookup = match lookup {
            Some(entry) => Some(LookupResult::Success(entry.clone())),
            None => Some(LookupResult::Invalid(String::from(text))),
        };

        self.selected = Some(Selected::ClassLookup);
        self.popup_scroll = 0;
    }

    pub fn lookup_files(&mut self) -> Result<()> {
        let dir = std::env::current_dir()?;
        let files = std::fs::read_dir(dir)?;

        let names: Vec<String> = files
            .map(|f| f.unwrap().file_name().to_str().unwrap().to_owned())
            .filter(|f| f != ".")
            .collect();

        self.selected = Some(Selected::Load(0));
        self.current_lookup = Some(LookupResult::Files(names));

        Ok(())
    }

    /// Get all completions for the currently selected tab item
    pub fn complete_current_selection(&mut self, lookup: &Lookup) -> Result<()> {
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

        self.current_lookup = Some(self.get_completion(&tab[item as usize], lookup));

        self.selected = Some(Selected::ItemLookup(item));
        self.popup_scroll = 0;
        Ok(())
    }

    pub fn get_completion(&self, text: &str, lookup: &Lookup) -> LookupResult {
        let text = text.trim().to_ascii_lowercase();
        let lookup = lookup.get_completions(&text);

        // Probably shouldn't clone but the lifetimes were too confusing :(
        if lookup.len() > 0 {
            LookupResult::Completion(lookup)
        } else {
            LookupResult::Invalid(format!("{}:{}", text.clone(), lookup.len()))
        }
    }

    pub fn finish_completion(&mut self) {
        let (comp_item, tab_item) = match self.selected {
            Some(Selected::Completion(c, i)) => (c, i),
            _ => unreachable!(),
        };

        let completion = match self.current_lookup {
            Some(LookupResult::Completion(ref vec)) => vec,
            Some(LookupResult::Invalid(_)) => return,
            _ => unreachable!(),
        }[comp_item as usize]
            .name
            .clone(); // If there's a way to do this without a clone I don't know it

        let current = &mut self.current_tab_mut()[tab_item as usize];
        current.push_str(&completion[current.trim().len()..]);
    }

    /// Get the control type associated with the currently selected item.
    pub fn get_selected_type(&mut self) -> Option<ControlType> {
        match self.selected {
            None
            | Some(Selected::Quitting)
            | Some(Selected::ItemLookup(_))
            | Some(Selected::Completion(_, _))
            | Some(Selected::ClassLookup)
            | Some(Selected::SpellSlots(_))
            | Some(Selected::Funds(_))
            | Some(Selected::FreeLookupSelect(_))
            | Some(Selected::Proficiency(_))
            | Some(Selected::Load(_)) => None,
            Some(Selected::TopBarItem(idx)) => match idx {
                0 => Some(ControlType::TextInput(&mut self.player.name)),
                1 => Some(ControlType::CycleFn(
                    // Currently, there are no calculations made with the race so just
                    // raw setting it is fine.
                    |app| app.player.update_race(app.player.race.prev()),
                    |app| app.player.update_race(app.player.race.next()),
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
            Some(Selected::FreeLookup) => Some(ControlType::TextInput(&mut self.lookup_buffer)),
        }
    }
}
