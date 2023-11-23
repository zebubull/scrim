use num_derive::FromPrimitive;
use serde_derive::{Deserialize, Serialize};
use strum::{Display, EnumCount};

#[derive(Clone, Copy, Default, FromPrimitive, Serialize, Deserialize, Display, EnumCount)]
pub enum Background {
    #[default]
    Acolyte,
    Charlatan,
    Criminal,
    Entertainer,
    #[strum(to_string = "Folk Hero")]
    FolkHero,
    #[strum(to_string = "Guild Artisan")]
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

impl Background {
    pub fn cycle_next(&mut self) {
        let next_val = std::cmp::min(*self as u8 + 1, Background::COUNT as u8 - 1);
        *self = num_traits::FromPrimitive::from_u8(next_val).unwrap();
    }

    pub fn cycle_prev(&mut self) {
        let prev_val = (*self as u8).saturating_sub(1);
        *self = num_traits::FromPrimitive::from_u8(prev_val).unwrap();
    }
}
