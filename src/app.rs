use std::path::Path;

use crate::{player::Player, Cycle};
use color_eyre::eyre::{eyre, Result, WrapErr};
use strum_macros::Display;

#[derive(Debug, Clone, Copy)]
pub enum Selected {
    TopBarItem(i8),
    StatItem(i8),
    InfoItem(i8),
    TabItem(i16),
    Quitting,
}

#[derive(Debug, Clone, Copy)]
pub enum ControlType {
    TextInput,
    NextPrev,
}

#[derive(Debug, Clone, Copy, Default, Display)]
pub enum Tab {
    #[default]
    Notes,
    Inventory,
    Spells,
}

#[derive(Debug, Default)]
pub struct App {
    pub player: Player,
    pub should_quit: bool,
    pub editing: bool,
    pub selected: Option<Selected>,
    pub control_type: Option<ControlType>,
    pub vscroll: u16,
    pub viewport_height: u16,
    pub current_tab: Tab,
    pub path: Option<String>,
}

impl App {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn quit(&mut self) {
        self.should_quit = true;
    }

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

    pub fn save_player(&self) -> Result<()> {
        let data = serde_json::to_string(&self.player)?;
        let path = format!("{}", self.path.as_ref().unwrap_or(&self.player.name));
        std::fs::write(path, data)?;
        Ok(())
    }

    pub fn update_viewport_height(&mut self, height: u16) -> Result<()> {
        // tab frame size
        self.viewport_height = height - 10;
        self.update_scroll()?;
        Ok(())
    }

    pub fn update_tab(&mut self, tab: Tab) {
        self.current_tab = tab;
        self.vscroll = std::cmp::min(
            self.vscroll,
            self.current_tab_len()
                .checked_sub(self.viewport_height as usize)
                .unwrap_or(0) as u16,
        );
        if let Some(Selected::TabItem(idx)) = self.selected {
            self.selected = Some(Selected::TabItem(std::cmp::min(
                idx,
                self.current_tab_len() as i16,
            )));
        }
    }

    pub fn can_edit_tab(&self) -> bool {
        self.current_tab_len() != 0
    }

    pub fn current_tab_len(&self) -> usize {
        use Tab::*;
        match self.current_tab {
            Notes => self.player.notes.len(),
            Inventory => self.player.inventory.len(),
            Spells => self.player.spells.len(),
        }
    }

    pub fn add_item_to_tab(&mut self) -> Result<()> {
        use Tab::*;
        match self.current_tab {
            Notes => {
                self.selected = Some(Selected::TabItem(self.player.notes.len() as i16));
                self.player.notes.push(String::from(" "));
            }
            Inventory => {
                self.selected = Some(Selected::TabItem(self.player.inventory.len() as i16));
                self.player.inventory.push(String::from(" "));
            }
            Spells => {
                self.selected = Some(Selected::TabItem(self.player.spells.len() as i16));
                self.player.spells.push(String::from(" "));
            }
        }

        self.update_scroll()?;
        Ok(())
    }

    pub fn insert_item_to_tab(&mut self) -> Result<()> {
        use Tab::*;
        let item = match self.selected {
            Some(Selected::TabItem(item)) => item,
            _ => return Err(eyre!("cannot insert while not a tab is not selected")),
        } as usize;

        match self.current_tab {
            Notes => {
                self.player.notes.insert(item, String::from(" "));
            }
            Inventory => {
                self.player.inventory.insert(item, String::from(" "));
            }
            Spells => {
                self.player.spells.insert(item, String::from(" "));
            }
        }

        self.update_scroll()?;
        Ok(())
    }

    pub fn delete_item_from_tab(&mut self) -> Result<()> {
        use Tab::*;
        let item = match self.selected {
            Some(Selected::TabItem(item)) => item,
            _ => return Err(eyre!("cannot insert while not a tab is not selected")),
        } as usize;

        if self.current_tab_len() == 0 {
            return Ok(());
        }

        match self.current_tab {
            Notes => {
                self.player.notes.remove(item);
            }
            Inventory => {
                self.player.inventory.remove(item);
            }
            Spells => {
                self.player.spells.remove(item);
            }
        };

        if item >= self.current_tab_len() {
            self.selected = Some(Selected::TabItem(item as i16 - 1));
        }

        self.update_scroll()?;
        Ok(())
    }

    pub fn update_scroll(&mut self) -> Result<()> {
        use Tab::*;
        let len = match self.current_tab {
            Notes => self.player.notes.len(),
            Inventory => self.player.inventory.len(),
            Spells => self.player.spells.len(),
        } as u16;

        let selected = match self.selected {
            Some(Selected::TabItem(item)) => item,
            _ => 0,
        } as u16;

        if len < self.viewport_height || selected < self.viewport_height {
            self.vscroll = 0;
        } else {
            self.vscroll = selected - self.viewport_height + 1;
        }

        Ok(())
    }

    pub fn update_selected_type(&mut self) {
        self.control_type = match self.selected {
            None | Some(Selected::Quitting) => None,
            Some(Selected::TopBarItem(idx)) => match idx {
                0 => Some(ControlType::TextInput),
                1 | 2 | 3 | 4 => Some(ControlType::NextPrev),
                _ => unreachable!(),
            },
            Some(Selected::StatItem(_)) => Some(ControlType::NextPrev),
            Some(Selected::InfoItem(idx)) => match idx {
                0 | 1 | 2 | 3 | 4 | 5 | 6 => Some(ControlType::NextPrev),
                _ => unreachable!(),
            },
            Some(Selected::TabItem(_)) => Some(ControlType::TextInput),
        }
    }

    pub fn get_current_string(&mut self) -> Result<&mut String> {
        match self.selected {
            None | Some(Selected::Quitting) => Err(eyre!("no control is selected")),
            Some(Selected::StatItem(_)) => Err(eyre!("selected control has no underlying string")),
            Some(Selected::InfoItem(_)) => Err(eyre!("selected control has no underlying string")),
            Some(Selected::TopBarItem(item)) => match item {
                0 => Ok(&mut self.player.name),
                _ => Err(eyre!("selected control has no underlying string")),
            },
            Some(Selected::TabItem(item)) => {
                use Tab::*;
                match self.current_tab {
                    Notes => Ok(&mut self.player.notes[item as usize]),
                    Inventory => Ok(&mut self.player.inventory[item as usize]),
                    Spells => Ok(&mut self.player.spells[item as usize]),
                }
            }
        }
    }

    pub fn cycle_current_next(&mut self) -> Result<()> {
        match self.selected {
            None | Some(Selected::TabItem(_)) | Some(Selected::Quitting) => {
                return Err(eyre!("no control is selected"))
            }
            Some(Selected::StatItem(item)) => {
                match item {
                    0 => {
                        self.player.stats.strength =
                            std::cmp::min(20, self.player.stats.strength + 1)
                    }
                    1 => {
                        self.player.stats.dexterity =
                            std::cmp::min(20, self.player.stats.dexterity + 1)
                    }
                    2 => {
                        self.player.stats.constitution =
                            std::cmp::min(20, self.player.stats.constitution + 1)
                    }
                    3 => {
                        self.player.stats.intelligence =
                            std::cmp::min(20, self.player.stats.intelligence + 1)
                    }
                    4 => self.player.stats.wisdom = std::cmp::min(20, self.player.stats.wisdom + 1),
                    5 => {
                        self.player.stats.charisma =
                            std::cmp::min(20, self.player.stats.charisma + 1)
                    }
                    _ => return Err(eyre!("selected control has no underlying cyclable")),
                }

                self.player.recalculate_stats();
            }
            Some(Selected::InfoItem(item)) => {
                match item {
                    0 => self.player.hp = std::cmp::min(self.player.max_hp, self.player.hp + 1),
                    1 => self.player.max_hp += 1,
                    2 => self.player.temp_hp += 1,
                    3 => self.player.ac = std::cmp::min(self.player.ac + 1, 20),
                    4 => self.player.prof_bonus += 1,
                    5 => {
                        self.player.hit_dice_remaining =
                            std::cmp::min(self.player.level, self.player.hit_dice_remaining + 1)
                    }
                    // Reverse for more natural scrolling
                    6 => self.player.background = self.player.background.prev(),
                    _ => return Err(eyre!("selected control has no underlying cyclable")),
                }
            }
            Some(Selected::TopBarItem(item)) => {
                match item {
                    2 => {
                        self.player.level = std::cmp::min(20, self.player.level + 1);
                        self.player.recalculate_level();
                    }
                    // Reverse these for more natural scrolling
                    1 => self.player.race = self.player.race.prev(),
                    3 => {
                        self.player.class = self.player.class.prev();
                        self.player.recalculate_class();
                    }
                    4 => self.player.alignment = self.player.alignment.prev(),
                    _ => return Err(eyre!("selected control has no underlying cyclable")),
                }
            }
        };

        Ok(())
    }

    pub fn cycle_current_prev(&mut self) -> Result<()> {
        match self.selected {
            None | Some(Selected::TabItem(_)) | Some(Selected::Quitting) => {
                return Err(eyre!("no control is selected"))
            }
            Some(Selected::StatItem(item)) => {
                match item {
                    0 => {
                        self.player.stats.strength =
                            self.player.stats.strength.checked_sub(1).unwrap_or(0)
                    }
                    1 => {
                        self.player.stats.dexterity =
                            self.player.stats.dexterity.checked_sub(1).unwrap_or(0)
                    }
                    2 => {
                        self.player.stats.constitution =
                            self.player.stats.constitution.checked_sub(1).unwrap_or(0)
                    }
                    3 => {
                        self.player.stats.intelligence =
                            self.player.stats.intelligence.checked_sub(1).unwrap_or(0)
                    }
                    4 => {
                        self.player.stats.wisdom =
                            self.player.stats.wisdom.checked_sub(1).unwrap_or(0)
                    }
                    5 => {
                        self.player.stats.charisma =
                            self.player.stats.charisma.checked_sub(1).unwrap_or(0)
                    }
                    _ => return Err(eyre!("selected control has no underlying cyclable")),
                }

                self.player.recalculate_stats();
            }
            Some(Selected::InfoItem(item)) => {
                match item {
                    0 => self.player.hp = self.player.hp.checked_sub(1).unwrap_or(0),
                    1 => self.player.max_hp = self.player.max_hp.checked_sub(1).unwrap_or(0),
                    2 => self.player.temp_hp = self.player.temp_hp.checked_sub(1).unwrap_or(0),
                    3 => self.player.ac = self.player.ac.checked_sub(1).unwrap_or(0),
                    4 => {
                        self.player.prof_bonus = self.player.prof_bonus.checked_sub(1).unwrap_or(0)
                    }
                    5 => {
                        self.player.hit_dice_remaining =
                            self.player.hit_dice_remaining.checked_sub(1).unwrap_or(0)
                    }
                    // Reverse for more natural scrolling
                    6 => self.player.background = self.player.background.next(),
                    _ => return Err(eyre!("selected control has no underlying cyclable")),
                }
            }
            Some(Selected::TopBarItem(item)) => {
                match item {
                    1 => self.player.race = self.player.race.next(),
                    2 => {
                        self.player.level = std::cmp::max(1, self.player.level - 1);
                        self.player.recalculate_level();
                    }
                    // Reverse these two for more natural scrolling
                    3 => {
                        self.player.class = self.player.class.next();
                        self.player.recalculate_class();
                    }
                    4 => self.player.alignment = self.player.alignment.next(),
                    _ => return Err(eyre!("selected control has no underlying cyclable")),
                }
            }
        };

        Ok(())
    }
}
