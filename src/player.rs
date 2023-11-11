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
    Dwarf,
    Elf,
    Gnome,
    HalfElf,
    HalfOrc,
    Halfling,
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
    pub strength: u8,
    pub dexterity: u8,
    pub constitution: u8,
    pub intelligence: u8,
    pub wisdom: u8,
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

#[derive(Debug, Serialize, Deserialize)]
pub struct Player {
    pub name: String,
    pub class: Class,
    pub level: u8,
    pub background: Background,
    pub alignment: Alignment,
    pub stats: Stats,
    pub hit_dice: u8,
    pub hit_dice_remaining: u8,
    pub race: Race,
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
        }
    }
}