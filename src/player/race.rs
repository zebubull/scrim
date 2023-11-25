use num_derive::FromPrimitive;
use serde_derive::{Deserialize, Serialize};
use strum::{Display, EnumCount};

use super::stats::Stats;

#[derive(
    Clone, Copy, Default, FromPrimitive, Serialize, Deserialize, Display, EnumCount, PartialEq,
)]
pub enum Race {
    Dragonborn,
    #[strum(to_string = "Hill Dwarf")]
    HillDwarf,
    #[strum(to_string = "Mountain Dwarf")]
    MountainDwarf,
    #[strum(to_string = "High Elf")]
    HighElf,
    #[strum(to_string = "Wood Elf")]
    WoodElf,
    #[strum(to_string = "Dark Elf")]
    DarkElf,
    #[strum(to_string = "Forest Gnome")]
    ForestGnome,
    #[strum(to_string = "Rock Gnome")]
    RockGnome,
    #[strum(to_string = "Half-Elf")]
    HalfElf,
    #[strum(to_string = "Half-Orc")]
    HalfOrc,
    #[strum(to_string = "Lightfoot Halfling")]
    LightfootHalfling,
    #[strum(to_string = "Stout Halfling")]
    StoutHalfling,
    #[default]
    Human,
    Tiefling,
}

impl Race {
    pub fn to_lookup_string(&self) -> &'static str {
        use Race::*;
        match self {
            Dragonborn => "dragonborn",
            HillDwarf | MountainDwarf => "dwarf",
            WoodElf | HighElf | DarkElf => "elf",
            HalfElf => "half-elf",
            HalfOrc => "half-orc",
            ForestGnome | RockGnome => "gnome",
            LightfootHalfling | StoutHalfling => "halfling",
            Human => "human",
            Tiefling => "tiefling",
        }
    }

    pub fn get_next(&self) -> Self {
        let next_val = std::cmp::min(*self as u8 + 1, Race::COUNT as u8 - 1);
        num_traits::FromPrimitive::from_u8(next_val).unwrap()
    }

    pub fn get_prev(&self) -> Self {
        let prev_val = (*self as u8).saturating_sub(1);
        num_traits::FromPrimitive::from_u8(prev_val).unwrap()
    }

    pub fn cycle_next(&mut self) {
        *self = self.get_next()
    }

    pub fn cycle_prev(&mut self) {
        *self = self.get_prev()
    }

    pub fn stats(&self) -> Stats {
        use Race::*;
        let strength = match self {
            MountainDwarf | Dragonborn | HalfOrc => 2,
            Human => 1,
            _ => 0,
        };
        let dexterity = match self {
            HighElf | WoodElf | DarkElf | LightfootHalfling | StoutHalfling => 2,
            Human | ForestGnome => 1,
            _ => 0,
        };

        let constitution = match self {
            MountainDwarf | HillDwarf => 2,
            StoutHalfling | Human | RockGnome => 1,
            _ => 0,
        };

        let intelligence = match self {
            ForestGnome | RockGnome => 2,
            HighElf | Human | Tiefling => 1,
            _ => 0,
        };

        let wisdom = match self {
            HillDwarf | WoodElf | Human => 1,
            _ => 0,
        };

        let charisma = match self {
            HalfElf | Tiefling => 2,
            DarkElf | LightfootHalfling | Human | Dragonborn => 1,
            _ => 0,
        };

        Stats {
            strength,
            dexterity,
            constitution,
            intelligence,
            wisdom,
            charisma,
        }
    }

    pub fn health_bonus(&self) -> u32 {
        use Race::*;
        match self {
            HillDwarf => 1,
            _ => 0,
        }
    }
}
