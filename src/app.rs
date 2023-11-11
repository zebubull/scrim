use crate::{player::Player, Cycle};
use color_eyre::eyre::{eyre, Result, WrapErr};

#[derive(Debug, Clone, Copy)]
pub enum Selected {
    TopBarItem(i8),
    StatItem(i8),
    InfoItem(i8),
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

    pub fn load_player(&mut self, path: &str) -> Result<()> {
        let data = std::fs::read(path)
            .wrap_err_with(|| format!("failed to load player from file `{}`", path))?;
        match serde_json::from_slice(data.as_slice()) {
            Ok(player) => self.player = player,
            _ => {},
        };
        Ok(())
    }

    pub fn update_selected_type(&mut self) {
        self.control_type = match self.selected {
            None => None,
            Some(Selected::TopBarItem(idx)) => match idx {
                0 => Some(ControlType::TextInput),
                1 | 2 | 3 | 4 => Some(ControlType::NextPrev),
                _ => unreachable!(),
            },
            Some(Selected::StatItem(_)) => Some(ControlType::NextPrev),
            Some(Selected::InfoItem(idx)) => match idx {
                0 | 1 => Some(ControlType::NextPrev),
                _ => unreachable!(),
            },
        }
    }

    pub fn get_current_string(&mut self) -> Result<&mut String> {
        match self.selected {
            None => Err(eyre!("no control is selected")),
            Some(Selected::StatItem(_)) => Err(eyre!("selected control has no underlying string")),
            Some(Selected::InfoItem(_)) => Err(eyre!("selected control has no underlying string")),
            Some(Selected::TopBarItem(item)) => {
                match item {
                    0 => Ok(&mut self.player.name),
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
            Some(Selected::InfoItem(item)) => {
                match item {
                    0 => self.player.hit_dice_remaining = std::cmp::min(self.player.level, self.player.hit_dice_remaining + 1),
                    // Reverse for more natural scrolling
                    1 => self.player.background = self.player.background.prev(),
                    _ => return Err(eyre!("selected control has no underlying cyclable")),
                }
            },
            Some(Selected::TopBarItem(item)) => {
                match item {
                    2 => { self.player.level = std::cmp::min(20, self.player.level + 1); self.player.recalculate_level(); },
                    // Reverse these for more natural scrolling
                    1 => self.player.race = self.player.race.prev(),
                    3 => { self.player.class = self.player.class.prev(); self.player.recalculate_class(); },
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
            Some(Selected::InfoItem(item)) => {
                match item {
                    0 => self.player.hit_dice_remaining = self.player.hit_dice_remaining.checked_sub(1).unwrap_or(0),
                    // Reverse for more natural scrolling
                    1 => self.player.background = self.player.background.next(),
                    _ => return Err(eyre!("selected control has no underlying cyclable")),
                }
            },
            Some(Selected::TopBarItem(item)) => {
                match item {
                    1 => self.player.race = self.player.race.prev(),
                    2 => { self.player.level = self.player.level.checked_sub(1).unwrap_or(0); self.player.recalculate_level(); },
                    // Reverse these two for more natural scrolling
                    3 => {self.player.class = self.player.class.next(); self.player.recalculate_class(); },
                    4 => self.player.alignment = self.player.alignment.next(),
                    _ => return Err(eyre!("selected control has no underlying cyclable")),
                }
            }
        };

        Ok(())
    }
}
