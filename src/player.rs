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
    pub strength: u32,
    #[serde(default)]
    pub dexterity: u32,
    #[serde(default)]
    pub constitution: u32,
    #[serde(default)]
    pub intelligence: u32,
    #[serde(default)]
    pub wisdom: u32,
    #[serde(default)]
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

// I do not know of a better way to do this
#[derive(Debug, Serialize, Deserialize)]
pub struct Player {
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub class: Class,
    #[serde(default)]
    pub level: u32,
    #[serde(default)]
    pub background: Background,
    #[serde(default)]
    pub alignment: Alignment,
    #[serde(default)]
    pub stats: Stats,
    #[serde(default)]
    pub hit_dice: u32,
    #[serde(default)]
    pub hit_dice_remaining: u32,
    #[serde(default)]
    pub race: Race,
    #[serde(default)]
    pub inventory: Vec<String>,
    #[serde(default)]
    pub notes: Vec<String>,
    #[serde(default)]
    pub spells: Vec<String>,
    #[serde(default)]
    pub hp: u32,
    #[serde(default)]
    pub ac: u32,
    #[serde(default)]
    pub temp_hp: u32,
    #[serde(default)]
    pub max_hp: u32,
    #[serde(default)]
    pub prof_bonus: u32,
}

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

    pub fn update_level_dependants(&mut self) {
        self.hit_dice_remaining = std::cmp::min(self.level, self.hit_dice_remaining + 1);
        self.prof_bonus = (self.level as f32 / 4.0).ceil() as u32 + 1;

        self.update_hp();
    }

    pub fn update_stat_dependants(&mut self) {
        self.update_hp();
    }

    fn update_hp(&mut self) {
        self.max_hp = self.hit_dice as u32
            + (avg_roll(self.hit_dice) as u32 * (self.level as u32 - 1))
            + ((self.stats.constitution as f32 - 10.0) / 2.0).floor() as u32 * self.level as u32;
        self.hp = std::cmp::min(self.hp, self.max_hp);
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
