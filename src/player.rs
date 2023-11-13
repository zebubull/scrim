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
pub struct Stats {
    #[serde(default)]
    pub strength: u8,
    #[serde(default)]
    pub dexterity: u8,
    #[serde(default)]
    pub constitution: u8,
    #[serde(default)]
    pub intelligence: u8,
    #[serde(default)]
    pub wisdom: u8,
    #[serde(default)]
    pub charisma: u8,
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
    type Item = u8;
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
    type Item = u8;

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

// I do not know of a better way to do this
#[derive(Debug, Serialize, Deserialize)]
pub struct Player {
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub class: Class,
    #[serde(default)]
    pub level: u8,
    #[serde(default)]
    pub background: Background,
    #[serde(default)]
    pub alignment: Alignment,
    #[serde(default)]
    pub stats: Stats,
    #[serde(default)]
    pub hit_dice: u8,
    #[serde(default)]
    pub hit_dice_remaining: u8,
    #[serde(default)]
    pub race: Race,
    #[serde(default)]
    pub inventory: Vec<String>,
    #[serde(default)]
    pub notes: Vec<String>,
    #[serde(default)]
    pub spells: Vec<String>,
    #[serde(default)]
    pub hp: u16,
    #[serde(default)]
    pub ac: u8,
    #[serde(default)]
    pub temp_hp: u16,
    #[serde(default)]
    pub max_hp: u16,
    #[serde(default)]
    pub prof_bonus: u8,
}

fn avg_roll(dice: u8) -> u8 {
    match dice {
        6 => 4,
        8 => 5,
        10 => 6,
        12 => 7,
        _ => unreachable!(),
    }
}

impl Player {
    /// Recalculate all class dependant values
    pub fn recalculate_class(&mut self) {
        use Class::*;
        self.hit_dice = match self.class {
            Sorcerer | Wizard => 6,
            Artificer | Bard | Cleric | Druid | Monk | Rogue | Warlock => 8,
            Fighter | Paladin | Ranger => 10,
            Barbarian => 12,
        };
        self.max_hp = self.hit_dice as u16
            + (avg_roll(self.hit_dice) as u16 * (self.level as u16 - 1))
            + ((self.stats.constitution as f32 - 10.0) / 2.0).floor() as u16 * self.level as u16;
    }

    /// Recalculate all level dependant values
    pub fn recalculate_level(&mut self) {
        self.hit_dice_remaining = std::cmp::min(self.level, self.hit_dice_remaining + 1);
        self.prof_bonus = (self.level as f32 / 4.0).ceil() as u8 + 1;
        self.max_hp = self.hit_dice as u16
            + (avg_roll(self.hit_dice) as u16 * (self.level as u16 - 1))
            + ((self.stats.constitution as f32 - 10.0) / 2.0).floor() as u16 * self.level as u16;
    }

    /// Recalculate all level dependant values
    pub fn recalculate_stats(&mut self) {
        self.max_hp = self.hit_dice as u16
            + (avg_roll(self.hit_dice) as u16 * (self.level as u16 - 1))
            + ((self.stats.constitution as f32 - 10.0) / 2.0).floor() as u16 * self.level as u16;
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
