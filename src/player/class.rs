use num_derive::FromPrimitive;
use serde_derive::{Deserialize, Serialize};
use strum::{Display, EnumCount};

#[derive(
    Clone, Copy, Default, FromPrimitive, Serialize, Deserialize, Display, EnumCount, PartialEq,
)]
pub enum Class {
    Artificer,
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

impl Class {
    pub fn hit_dice(&self) -> u32 {
        use Class::*;
        match self {
            Sorcerer | Wizard => 6,
            Artificer | Bard | Cleric | Druid | Monk | Rogue | Warlock => 8,
            Fighter | Paladin | Ranger => 10,
            Barbarian => 12,
        }
    }

    pub fn cycle_next(&mut self) {
        let next_val = std::cmp::min(*self as u8 + 1, Class::COUNT as u8 - 1);
        *self = num_traits::FromPrimitive::from_u8(next_val).unwrap();
    }

    pub fn cycle_prev(&mut self) {
        let prev_val = (*self as u8).saturating_sub(1);
        *self = num_traits::FromPrimitive::from_u8(prev_val).unwrap();
    }
}
