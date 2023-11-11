use std::fmt::Display;

use num_derive::FromPrimitive;
use num_traits::FromPrimitive;
pub trait Scrollable {
    fn next(self) -> Self;
    fn prev(self) -> Self;
}

#[derive(Debug, Clone, Copy, Default, FromPrimitive)]
pub enum Class {
    Bard,
    Barbarian,
    Cleric,
    Druid,
    #[default]
    Fighter,
    Monk,
    Paladin,
    Ranger,
    Rogue,
    Sorcerer,
    Warlock,
    Wizard,
}

impl Display for Class {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let text = match self {
            Self::Bard => "Bard",
            Self::Barbarian => "Barbarian",
            Self::Cleric => "Cleric",
            Self::Druid => "Druid",
            Self::Fighter => "Fighter",
            Self::Monk => "Monk",
            Self::Paladin => "Paladin",
            Self::Ranger => "Ranger",
            Self::Rogue => "Rogue",
            Self::Sorcerer => "Sorcerer",
            Self::Warlock => "Warlock",
            Self::Wizard => "Wizard",
        };

        write!(f, "{}", text)
    }
}

impl Scrollable for Class {
    fn next(self) -> Class {
        FromPrimitive::from_u8(std::cmp::min(self as u8 + 1, 11)).unwrap()
    }

    fn prev(self) -> Class {
        FromPrimitive::from_u8(std::cmp::max(self as i8 - 1, 0) as u8).unwrap()
    }
}

#[derive(Debug, Clone, Copy, Default, FromPrimitive)]
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

impl Display for Alignment {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let text = match self {
            Self::LG => "LG",
            Self::LN => "LN",
            Self::LE => "LE",
            Self::NG => "NG",
            Self::TN => "TN",
            Self::NE => "NE",
            Self::CG => "CG",
            Self::CN => "CN",
            Self::CE => "CE",
        };

        write!(f, "{}", text)
    }
}

impl Scrollable for Alignment {
    fn next(self) -> Alignment {
        FromPrimitive::from_u8(std::cmp::min(self as u8 + 1, 8)).unwrap()
    }

    fn prev(self) -> Alignment {
        FromPrimitive::from_u8(std::cmp::max(self as i8 - 1, 0) as u8).unwrap()
    }
}

#[derive(Debug)]
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

#[derive(Debug, Default)]
pub struct Player {
    pub name: String,
    pub class: Class,
    pub level: u8,
    pub background: String,
    pub alignment: Alignment,
    pub stats: Stats,
}