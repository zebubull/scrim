use num_derive::FromPrimitive;
use serde_derive::{Deserialize, Serialize};
use strum::{Display, EnumCount};

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
        use Race::{
            DarkElf, Dragonborn, ForestGnome, HalfElf, HalfOrc, HighElf, HillDwarf, Human,
            LightfootHalfling, MountainDwarf, RockGnome, StoutHalfling, Tiefling, WoodElf,
        };
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
}
