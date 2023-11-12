use num_derive::FromPrimitive;
use serde_derive::{Serialize, Deserialize};
use strum_macros::{Display, EnumCount};

#[derive(Debug, Clone, Copy, Default, FromPrimitive, Serialize, Deserialize, Display, EnumCount)]
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

#[derive(Debug, Clone, Copy, Default, FromPrimitive, Serialize, Deserialize, Display, EnumCount)]
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

#[derive(Debug, Clone, Copy, Default, FromPrimitive, Serialize, Deserialize, Display, EnumCount)]
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

#[derive(Debug, Clone, Copy, Default, FromPrimitive, Serialize, Deserialize, Display, EnumCount)]
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

#[derive(Debug, Serialize, Deserialize)]
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
    pub charisma: u8
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
    }

    /// Recalculate all level dependant values
    pub fn recalculate_level(&mut self) {
        self.hit_dice_remaining = std::cmp::min(self.level, self.hit_dice_remaining + 1);
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
            hp: 0,
            ac: 0,
            temp_hp: 0,
            max_hp: 0,
            prof_bonus: 0,
        }
    }
}
