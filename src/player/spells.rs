use serde_derive::{Deserialize, Serialize};

#[derive(Default, Serialize, Deserialize, Clone)]
#[serde(default)]
pub struct SpellSlots {
    first: u32,
    second: u32,
    third: u32,
    fourth: u32,
    fifth: u32,
    sixth: u32,
    seventh: u32,
    eigth: u32,
    ninth: u32,
    pub warlock: u32,
}

macro_rules! get_slots {
    ($i:ident, $n:literal) => {
        if $i >= $n { 1 } else { 0 }
    };

    ($i:ident, $n:literal, $($ns:literal),+) => {
        if $i >= $n { 1 } else { 0 } + get_slots!($i, $($ns),+)
    }
}

impl SpellSlots {
    pub fn from(level: u32, class: &super::Class) -> Self {
        use super::Class::*;
        match class {
            Bard | Cleric | Wizard | Sorcerer | Druid => SpellSlots::from_full(level),
            Artificer | Ranger | Paladin => SpellSlots::from_half(level),
            Warlock => SpellSlots::from_warlock(level),
            _ => Self::default(),
        }
    }

    fn from_full(level: u32) -> Self {
        Self {
            first: get_slots!(level, 1, 1, 2, 3),
            second: get_slots!(level, 3, 3, 4),
            third: get_slots!(level, 5, 5, 6),
            fourth: get_slots!(level, 7, 8, 9),
            fifth: get_slots!(level, 9, 10, 18),
            sixth: get_slots!(level, 11, 19),
            seventh: get_slots!(level, 13, 20),
            eigth: get_slots!(level, 15),
            ninth: get_slots!(level, 17),
            ..Self::default()
        }
    }

    fn from_half(level: u32) -> Self {
        Self {
            first: get_slots!(level, 2, 2, 3, 5),
            second: get_slots!(level, 5, 5, 7),
            third: get_slots!(level, 9, 9, 11),
            fourth: get_slots!(level, 13, 15, 17),
            fifth: get_slots!(level, 17, 19),
            ..Self::default()
        }
    }

    fn from_warlock(level: u32) -> Self {
        Self {
            warlock: get_slots!(level, 1, 2, 11, 17),
            ..Self::default()
        }
    }

    pub fn warlock_slot_level(level: u32) -> u32 {
        get_slots!(level, 1, 3, 5, 7, 9)
    }
}

impl std::ops::Index<usize> for SpellSlots {
    type Output = u32;

    fn index(&self, index: usize) -> &Self::Output {
        match index {
            0 => &self.first,
            1 => &self.second,
            2 => &self.third,
            3 => &self.fourth,
            4 => &self.fifth,
            5 => &self.sixth,
            6 => &self.seventh,
            7 => &self.eigth,
            8 => &self.ninth,
            _ => panic!("invalid spell slot index"),
        }
    }
}

impl std::ops::IndexMut<usize> for SpellSlots {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        match index {
            0 => &mut self.first,
            1 => &mut self.second,
            2 => &mut self.third,
            3 => &mut self.fourth,
            4 => &mut self.fifth,
            5 => &mut self.sixth,
            6 => &mut self.seventh,
            7 => &mut self.eigth,
            8 => &mut self.ninth,
            _ => panic!("invalid spell slot index"),
        }
    }
}
