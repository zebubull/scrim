use num_derive::FromPrimitive;
use serde_derive::{Deserialize, Serialize};
use strum_macros::{Display, EnumCount};

#[derive(
    Debug, Clone, Copy, Default, FromPrimitive, Serialize, Deserialize, Display, EnumCount,
)]
pub enum Class {
    #[default]
    Artificer,
    Bard,
    Barbarian,
    Cleric,
    Druid,
    Fighter,
    Monk,
    Paladin,
    Ranger,
    Rogue,
    Sorcerer,
    Warlock,
    Wizard,
}

crate::impl_cycle!(Class);

#[derive(
    Debug, Clone, Copy, Default, FromPrimitive, Serialize, Deserialize, Display, EnumCount,
)]
pub enum Alignment {
    LG,
    LN,
    LE,
    NG,
    #[default]
    TN,
    NE,
    CG,
    CN,
    CE,
}

crate::impl_cycle!(Alignment);

#[derive(
    Debug, Clone, Copy, Default, FromPrimitive, Serialize, Deserialize, Display, EnumCount,
)]
pub enum Race {
    Dragonborn,
    HillDwarf,
    MountainDwarf,
    HighElf,
    WoodElf,
    DarkElf,
    ForestGnome,
    RockGnome,
    HalfElf,
    HalfOrc,
    LightfootHalfling,
    StoutHalfling,
    #[default]
    Human,
    Tiefling,
}

crate::impl_cycle!(Race);

#[derive(
    Debug, Clone, Copy, Default, FromPrimitive, Serialize, Deserialize, Display, EnumCount,
)]
pub enum Background {
    #[default]
    Acolyte,
    Charlatan,
    Criminal,
    Entertainer,
    FolkHero,
    GuildArtisan,
    Hermit,
    Knight,
    Noble,
    Outlander,
    Pirate,
    Sage,
    Sailor,
    Soldier,
    Urchin,
}

crate::impl_cycle!(Background);

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(default)]
pub struct Stats {
    pub strength: u32,
    pub dexterity: u32,
    pub constitution: u32,
    pub intelligence: u32,
    pub wisdom: u32,
    pub charisma: u32,
}

impl Stats {
    pub fn nth(&mut self, idx: u32) -> &mut u32 {
        match idx {
            0 => &mut self.strength,
            1 => &mut self.dexterity,
            2 => &mut self.constitution,
            3 => &mut self.intelligence,
            4 => &mut self.wisdom,
            5 => &mut self.charisma,
            _ => unreachable!(),
        }
    }
}

impl Default for Stats {
    fn default() -> Self {
        Self {
            strength: 10,
            dexterity: 10,
            constitution: 10,
            intelligence: 10,
            wisdom: 10,
            charisma: 10,
        }
    }
}

impl IntoIterator for Stats {
    type Item = u32;
    type IntoIter = StatsIter;

    fn into_iter(self) -> Self::IntoIter {
        StatsIter {
            stats: self,
            idx: 0,
        }
    }
}

pub struct StatsIter {
    stats: Stats,
    idx: usize,
}

impl Iterator for StatsIter {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        let ret = match self.idx {
            0 => self.stats.strength,
            1 => self.stats.dexterity,
            2 => self.stats.constitution,
            3 => self.stats.intelligence,
            4 => self.stats.wisdom,
            5 => self.stats.charisma,
            _ => return None,
        };
        self.idx += 1;
        Some(ret)
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(default)]
pub struct Player {
    /// The player's name.
    pub name: String,
    /// The player's class.
    ///
    /// This should not be manually updated, as class dependant
    /// values will not be recalculated; call `Player::update_class()` instead.
    pub class: Class,
    /// The player's level.
    ///
    /// Whenever this is changed, `Player::update_level_dependants()`
    /// should be called to ensure that all calculated values are up to date.
    pub level: u32,
    /// The player's background.
    pub background: Background,
    /// The player's alignment.
    pub alignment: Alignment,
    /// The player's stats.
    ///
    /// Whenever a stat is changed, `Player::update_stat_dependants()`
    /// should be called to ensure that all calculated values are up to date.
    pub stats: Stats,
    /// The player's hit dice value, as calculated using the current class.
    pub hit_dice: u32,
    /// The amount of hit dice the player has remaining.
    pub hit_dice_remaining: u32,
    /// The player's current race.
    ///
    /// Currently, no dependants are calculated using the player's race, so this
    /// is free to be updated as-is.
    pub race: Race,
    /// A vector containing all entries into the inventory tab.
    pub inventory: Vec<String>,
    /// A vector containing all entries into the notes tab.
    pub notes: Vec<String>,
    /// A vector containing all entries into the spells tab.
    pub spells: Vec<String>,
    /// The player's current health.
    pub hp: u32,
    /// The player's armor class.
    pub ac: u32,
    /// The amount of temporary hit points the player has.
    pub temp_hp: u32,
    /// The player's maximum health.
    ///
    /// This value is automatically calculated using the player's class and
    /// level, assuming that avg rolls are used (as they should be).
    pub max_hp: u32,
    /// The player's proficiency bonus.
    ///
    /// This value is automatically calculated using the player's level.
    pub prof_bonus: u32,
}

/// Get the avg roll value for a given dice.
fn avg_roll(dice: u32) -> u32 {
    match dice {
        6 => 4,
        8 => 5,
        10 => 6,
        12 => 7,
        _ => unreachable!(),
    }
}

impl Player {
    /// Sets the player's class and updates any class dependant values.
    pub fn update_class(&mut self, class: Class) {
        use Class::*;
        self.class = class;
        self.hit_dice = match self.class {
            Sorcerer | Wizard => 6,
            Artificer | Bard | Cleric | Druid | Monk | Rogue | Warlock => 8,
            Fighter | Paladin | Ranger => 10,
            Barbarian => 12,
        };

        self.update_hp();
    }

    /// Updates any level dependant values.
    pub fn update_level_dependants(&mut self) {
        self.hit_dice_remaining = std::cmp::min(self.level, self.hit_dice_remaining + 1);
        self.prof_bonus = (self.level as f32 / 4.0).ceil() as u32 + 1;

        self.update_hp();
    }

    /// Updates any stat dependant values.
    pub fn update_stat_dependants(&mut self) {
        self.update_hp();
    }

    /// Recalculates the player's max health and adjusts current
    /// health accordingly.
    fn update_hp(&mut self) {
        let old_max = self.max_hp as i32;
        self.max_hp = self.hit_dice as u32
            + (avg_roll(self.hit_dice) as u32 * (self.level as u32 - 1))
            + ((self.stats.constitution as f32 - 10.0) / 2.0).floor() as u32 * self.level as u32;

        self.hp = self.hp.saturating_add_signed(self.max_hp as i32 - old_max);
    }
}

impl Default for Player {
    fn default() -> Self {
        Self {
            level: 1,
            hit_dice: 8,
            hit_dice_remaining: 1,
            name: String::default(),
            class: Class::default(),
            background: Background::default(),
            alignment: Alignment::default(),
            stats: Stats::default(),
            race: Race::default(),
            inventory: vec![],
            notes: vec![],
            spells: vec![],
            hp: 8,
            ac: 0,
            temp_hp: 0,
            max_hp: 8,
            prof_bonus: 2,
        }
    }
}
