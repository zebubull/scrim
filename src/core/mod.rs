mod scroll_provider;

use std::{path::PathBuf, rc::Rc};

use crate::{
    lookup::{Lookup, LookupEntry},
    player::Player,
};
use color_eyre::eyre::Result;
use strum_macros::Display;

use self::scroll_provider::ScrollProvider;

/// An enum that represents a control as well as an index into that control's values, if it has any.
#[derive(Clone, Copy)]
pub enum Selected {
    /// An item in the top bar.
    TopBarItem,
    /// An item in the stat block.
    StatItem,
    /// An item in the player info bar.
    InfoItem,
    /// A line in the tab panel.``
    TabItem,
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
    Completion(u32),
    /// The spell slots popup is showing.
    SpellSlots,
    /// The money popup is showing.
    Funds,
    /// The free lookup menu is showing.
    FreeLookup,
    /// The free lookup select menu is showing.
    FreeLookupSelect,
    /// The proficiency menu is showing
    Proficiency,
    /// The load menu is showing
    Load,
    /// The error popup is showing
    Error,
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
    /// A control type that cycles numericaly upwards or downwards. A mutable
    /// reference to the number is provided, as well as a minimum and maximum
    /// value, respectively. This also indicates that the change will require
    /// a recalculation of player values.
    CycleRecalc(&'a mut u32, u32, u32),
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
    /// The currently selected tab.
    pub current_tab: Tab,
    /// The player path specified at startup, if it exists.
    pub path: Option<PathBuf>,
    /// The most recent lookup result, if it exists.
    pub current_lookup: Option<LookupResult>,
    /// The current free lookup buffer.
    pub lookup_buffer: String,
    /// The current selected control.
    pub selected: Option<Selected>,
    /// The current selected index for certain controls.
    pub index: u32,
    /// The current error string, if it exists.
    pub error: Option<String>,
    tab_scroll_provider: ScrollProvider,
    popup_scroll_provider: ScrollProvider,
}

impl App {
    /// Create a new instance of the `App` struct. Currently aliases to `App::default()`.
    pub fn new() -> Self {
        let mut app = Self::default();
        app.popup_scroll_mut().set_max(1);
        app.tab_scroll_mut().set_max(1);
        app
    }

    /// Requests the application to exit by updating the `should_quit` value.
    pub fn quit(&mut self) {
        self.should_quit = true;
    }

    /// Attempts to load the player at the given file path.
    ///
    /// The app will remember the load path for future saving.
    pub fn load_player(&mut self, path: PathBuf) -> Result<()> {
        self.player = Player::load(path.as_path())?;
        self.path = Some(path);
        let len = self.current_tab().len() as u32;
        self.tab_scroll_mut().set_max(len);
        Ok(())
    }

    /// Saves the currently edited player.
    ///
    /// Will either save to the current player path or,
    /// if it doesn't exist, a new file with the same name as the player.
    pub fn save_player(&self) -> Result<()> {
        let data = serde_json::to_string(&self.player)?;
        let path = match self.path.as_ref() {
            Some(path) => path.to_owned(),
            None => PathBuf::from(&format!("{}.player", self.player.name)),
        };

        std::fs::write(path, data)?;
        Ok(())
    }

    /// Update the app's internal viewport height.
    ///
    /// This value is used in scroll calculations,
    /// so calling this function may scroll the active pane.
    pub fn update_viewport_height(&mut self, height: u16) {
        // TODO: popup frame provider needs to be updated as well.
        self.tab_scroll_provider
            .update_frame_height(crate::ui::tab_pane_height(height) as u32);
    }

    pub fn show_error(&mut self, error: String) {
        self.error = Some(error);
        self.selected = Some(Selected::Error);
        self.popup_scroll_provider.clear_max();
    }

    /// Returns a reference to the tab scroll provider
    pub fn tab_scroll(&self) -> &ScrollProvider {
        &self.tab_scroll_provider
    }

    /// Returns a mutable reference to the tab scroll provider
    pub fn tab_scroll_mut(&mut self) -> &mut ScrollProvider {
        &mut self.tab_scroll_provider
    }

    /// Returns a reference to the popup scroll provider
    pub fn popup_scroll(&self) -> &ScrollProvider {
        &self.popup_scroll_provider
    }

    /// Returns a mutable reference to the popup scroll provider
    pub fn popup_scroll_mut(&mut self) -> &mut ScrollProvider {
        &mut self.popup_scroll_provider
    }

    /// Returns a reference to the data of the currently selected tab.
    pub fn current_tab(&self) -> &Vec<String> {
        use Tab::{Inventory, Notes, Spells};
        match self.current_tab {
            Notes => &self.player.notes,
            Inventory => &self.player.inventory,
            Spells => &self.player.spells,
        }
    }

    /// Returns a mutable reference to the data of the currently selected tab.
    pub fn current_tab_mut(&mut self) -> &mut Vec<String> {
        use Tab::{Inventory, Notes, Spells};
        match self.current_tab {
            Notes => &mut self.player.notes,
            Inventory => &mut self.player.inventory,
            Spells => &mut self.player.spells,
        }
    }

    /// Switches the current tab and recalculates the current tab scroll.
    pub fn update_tab(&mut self, tab: Tab) -> Result<()> {
        self.current_tab = tab;
        self.tab_scroll_provider
            .set_max(self.current_tab().len() as u32);
        Ok(())
    }

    /// Adds an empty entry to the current tab.
    ///
    /// The entry is located after the currently selected item
    /// or at the first position if the current tab is empty.
    /// This method will also recalculate the current tab scroll.
    pub fn append_item_to_tab(&mut self) -> Result<()> {
        let mut item = self.tab_scroll_provider.get_line() as usize;

        let tab = self.current_tab_mut();

        if !tab.is_empty() {
            item += 1;
        }

        tab.insert(item, String::from(" "));

        let len = tab.len() as u32;
        self.tab_scroll_provider.set_max(len);
        self.tab_scroll_provider.scroll_down(1);
        Ok(())
    }

    /// Adds an empty entry to the current tab.
    ///
    /// The entry is located before the currently selected item
    /// or at the first position if the current tab is empty.
    /// This method will also recalculate the current tab scroll.
    pub fn insert_item_to_tab(&mut self) -> Result<()> {
        let item = self.tab_scroll_provider.get_line() as usize;
        let tab = self.current_tab_mut();
        tab.insert(item, String::from(" "));

        let len = tab.len() as u32;
        self.tab_scroll_provider.set_max(len);
        Ok(())
    }

    /// Remove the currently selected entry from the tab.
    ///
    /// This method does not check to make sure there is an entry
    /// to delete and will panic if the current tab is empty. It will
    /// also recalulate the current tab scroll.
    pub fn delete_item_from_tab(&mut self) -> Result<()> {
        let item = self.tab_scroll_provider.get_line() as usize;

        let tab = self.current_tab_mut();
        tab.remove(item);

        let len = tab.len() as u32;
        self.tab_scroll_provider.set_max(len);

        Ok(())
    }

    /// Uses the current selected tab item to lookup a reference entry.
    ///
    /// This method does not perform any kind of caching.
    pub fn lookup_current_selection(&mut self, lookup: &mut Lookup) -> Result<()> {
        let item = self.tab_scroll_provider.get_line();

        let text = &self.current_tab()[item as usize].clone();
        self.lookup_text(lookup, text)?;
        self.selected = Some(Selected::ItemLookup(item));

        Ok(())
    }

    /// Lookup the player's current class
    pub fn lookup_class(&mut self, lookup: &mut Lookup) -> Result<()> {
        let text = self.player.class.to_string();
        self.lookup_text(lookup, &text)?;
        self.selected = Some(Selected::ClassLookup);

        Ok(())
    }

    /// Lookup the player's current race.
    pub fn lookup_race(&mut self, lookup: &mut Lookup) -> Result<()> {
        let text = self.player.race.to_lookup_string();
        self.lookup_text(lookup, text)?;
        self.selected = Some(Selected::ClassLookup);

        Ok(())
    }

    /// Try to lookup the given text
    fn lookup_text(&mut self, lookup: &mut Lookup, text: &str) -> Result<()> {
        if !lookup.loaded {
            lookup.load()?;
        }

        let lookup = lookup.get_entry(text);

        // Probably shouldn't clone but the lifetimes were too confusing :(
        self.current_lookup = match lookup {
            Some(entry) => {
                self.popup_scroll_provider.clear_max();
                Some(LookupResult::Success(entry.clone()))
            }
            None => {
                self.popup_scroll_provider.set_max(1);
                Some(LookupResult::Invalid(text.to_owned()))
            }
        };

        self.popup_scroll_provider.reset();
        Ok(())
    }

    /// Lookup all player files in the cwd
    pub fn lookup_files(&mut self) -> Result<()> {
        let dir = std::env::current_dir()?;
        let files = std::fs::read_dir(dir)?;

        let names: Vec<String> = files
            .map(|f| f.unwrap().path())
            .filter(|f| f.is_file() && f.extension().unwrap_or_default() == "player")
            .map(|f| f.to_str().unwrap_or(&f.to_string_lossy()).to_owned())
            .collect();

        self.selected = Some(Selected::Load);
        self.current_lookup = Some(LookupResult::Files(names));
        self.popup_scroll_provider.reset();

        Ok(())
    }

    /// Get all completions for the currently selected tab item
    pub fn complete_current_selection(&mut self, lookup: &Lookup) -> Result<()> {
        let item = self.tab_scroll_provider.get_line();
        let tab = self.current_tab();

        let result = self.get_completion(&tab[item as usize].clone(), lookup);

        self.selected = Some(Selected::ItemLookup(item));
        self.current_lookup = Some(result);
        self.popup_scroll_provider.reset();
        Ok(())
    }

    /// Get the completion result for the provided text
    pub fn get_completion(&mut self, text: &str, lookup: &Lookup) -> LookupResult {
        let text = text.trim().to_ascii_lowercase();
        let lookup = lookup.get_completions(&text);

        // Probably shouldn't clone but the lifetimes were too confusing :(
        if !lookup.is_empty() {
            self.popup_scroll_provider.set_max(lookup.len() as u32);
            LookupResult::Completion(lookup)
        } else {
            self.popup_scroll_provider.set_max(1);
            LookupResult::Invalid(text.clone())
        }
    }

    /// Complete the current text using the current selection
    pub fn finish_completion(&mut self) {
        let comp_item = self.popup_scroll_provider.get_line();

        let completion = match self.current_lookup {
            Some(LookupResult::Completion(ref vec)) => vec,
            Some(LookupResult::Invalid(_)) => return,
            _ => unreachable!(),
        }[comp_item as usize]
            .name
            .clone(); // If there's a way to do this without a clone I don't know it

        let tab_item = match self.selected {
            Some(Selected::Completion(item)) => item,
            _ => panic!("attempt to finish completion while not in completion mode"),
        };

        let current = &mut self.current_tab_mut()[tab_item as usize];
        current.push_str(&completion[current.trim().len()..]);
    }

    /// Get the control type associated with the currently selected item.
    pub fn get_selected_type(&mut self) -> Option<ControlType> {
        match self.selected {
            None
            | Some(
                Selected::Quitting
                | Selected::ItemLookup(_)
                | Selected::Completion(_)
                | Selected::ClassLookup
                | Selected::SpellSlots
                | Selected::Funds
                | Selected::FreeLookupSelect
                | Selected::Proficiency
                | Selected::Error
                | Selected::Load,
            ) => None,
            Some(Selected::TopBarItem) => match self.index {
                0 => Some(ControlType::TextInput(&mut self.player.name)),
                1 => Some(ControlType::CycleFn(
                    |app| app.player.update_race(app.player.race.get_prev()),
                    |app| app.player.update_race(app.player.race.get_next()),
                )),
                2 => Some(ControlType::CycleRecalc(&mut self.player.level, 1, 20)),
                3 => Some(ControlType::CycleFn(
                    |app| {
                        app.player.class.cycle_prev();
                        app.player.recalculate();
                    },
                    |app| {
                        app.player.class.cycle_next();
                        app.player.recalculate();
                    },
                )),
                4 => Some(ControlType::CycleFn(
                    // Just like race, there are no calculations made with the alignment
                    // so just raw setting it is fine.
                    |app| {
                        app.player.alignment.cycle_prev();
                        app.player.recalculate()
                    },
                    |app| {
                        app.player.alignment.cycle_next();
                        app.player.recalculate()
                    },
                )),
                _ => unreachable!(),
            },
            Some(Selected::StatItem) => Some(ControlType::CycleRecalc(
                &mut self.player.stats[self.index as usize],
                1,
                30,
            )),
            Some(Selected::InfoItem) => match self.index {
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
                    |app| app.player.background.cycle_prev(),
                    |app| app.player.background.cycle_next(),
                )),
                _ => unreachable!(),
            },
            Some(Selected::TabItem) => Some(ControlType::TextInput(match self.current_tab {
                Tab::Notes => &mut self.player.notes[self.tab_scroll_provider.get_line() as usize],
                Tab::Inventory => {
                    &mut self.player.inventory[self.tab_scroll_provider.get_line() as usize]
                }
                Tab::Spells => {
                    &mut self.player.spells[self.tab_scroll_provider.get_line() as usize]
                }
            })),
            Some(Selected::FreeLookup) => Some(ControlType::TextInput(&mut self.lookup_buffer)),
        }
    }
}
