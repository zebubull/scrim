use crate::player::{Player, Scrollable};
use color_eyre::eyre::{eyre, Result};

#[derive(Debug, Clone, Copy)]
pub enum Selected {
    TopBarItem(i8),
    StatItem(i8)
}

#[derive(Debug, Clone, Copy)]
pub enum ControlType {
    TextInput,
    NextPrev,
}

#[derive(Debug, Default)]
pub struct App {
    pub player: Player,
    pub should_quit: bool,
    pub editing: bool,
    pub selected: Option<Selected>,
    pub control_type: Option<ControlType>,
}

impl App {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn quit(&mut self) {
        self.should_quit = true;
    }

    pub fn update_selected_type(&mut self) {
        self.control_type = match self.selected {
            None => None,
            Some(Selected::TopBarItem(idx)) => match idx {
                0 | 1 => Some(ControlType::TextInput),
                2 | 3 | 4 => Some(ControlType::NextPrev),
                _ => unreachable!(),
            },
            Some(Selected::StatItem(_)) => Some(ControlType::NextPrev),
        }
    }

    pub fn get_current_string(&mut self) -> Result<&mut String> {
        match self.selected {
            None => Err(eyre!("no control is selected")),
            Some(Selected::StatItem(_)) => Err(eyre!("selected control has no underlying string")),
            Some(Selected::TopBarItem(item)) => {
                match item {
                    0 => Ok(&mut self.player.name),
                    1 => Ok(&mut self.player.background),
                    _ => Err(eyre!("selected control has no underlying string")),
                }
            }
        }
    }

    pub fn cycle_current_next(&mut self) -> Result<()> {
        match self.selected {
            None => return Err(eyre!("no control is selected")),
            Some(Selected::StatItem(item)) => {
                match item {
                    0 => self.player.stats.strength = std::cmp::min(20, self.player.stats.strength + 1),
                    1 => self.player.stats.dexterity = std::cmp::min(20, self.player.stats.dexterity + 1),
                    2 => self.player.stats.constitution = std::cmp::min(20, self.player.stats.constitution + 1),
                    3 => self.player.stats.intelligence = std::cmp::min(20, self.player.stats.intelligence + 1),
                    4 => self.player.stats.wisdom = std::cmp::min(20, self.player.stats.wisdom + 1),
                    5 => self.player.stats.charisma = std::cmp::min(20, self.player.stats.charisma + 1),
                    _ => return Err(eyre!("selected control has no underlying cyclable")),
                }
            },
            Some(Selected::TopBarItem(item)) => {
                match item {
                    2 => self.player.level = std::cmp::min(20, self.player.level + 1),
                    // Reverse these two for more natural scrolling
                    3 => self.player.class = self.player.class.prev(),
                    4 => self.player.alignment = self.player.alignment.prev(),
                    _ => return Err(eyre!("selected control has no underlying cyclable")),
                }
            }
        };

        Ok(())
    }

    pub fn cycle_current_prev(&mut self) -> Result<()> {
        match self.selected {
            None => return Err(eyre!("no control is selected")),
            Some(Selected::StatItem(item)) => {
                match item {
                    0 => self.player.stats.strength = self.player.stats.strength.checked_sub(1).unwrap_or(0),
                    1 => self.player.stats.dexterity = self.player.stats.dexterity.checked_sub(1).unwrap_or(0),
                    2 => self.player.stats.constitution = self.player.stats.constitution.checked_sub(1).unwrap_or(0),
                    3 => self.player.stats.intelligence = self.player.stats.intelligence.checked_sub(1).unwrap_or(0),
                    4 => self.player.stats.wisdom = self.player.stats.wisdom.checked_sub(1).unwrap_or(0),
                    5 => self.player.stats.charisma = self.player.stats.charisma.checked_sub(1).unwrap_or(0),
                    _ => return Err(eyre!("selected control has no underlying cyclable")),
                }
            },
            Some(Selected::TopBarItem(item)) => {
                match item {
                    2 => self.player.level = self.player.level.checked_sub(1).unwrap_or(0),
                    // Reverse these two for more natural scrolling
                    3 => self.player.class = self.player.class.next(),
                    4 => self.player.alignment = self.player.alignment.next(),
                    _ => return Err(eyre!("selected control has no underlying cyclable")),
                }
            }
        };

        Ok(())
    }
}
