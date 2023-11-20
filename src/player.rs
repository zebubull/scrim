use num_derive::FromPrimitive;
use serde_derive::{Deserialize, Serialize};
use strum_macros::{Display, EnumCount};

#[derive(
    Debug,
    Clone,
    Copy,
    Default,
    FromPrimitive,
    Serialize,
    Deserialize,
    Display,
    EnumCount,
    PartialEq,
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
    Debug, Clone, Copy, Default, FromPrimitive, Serialize, Deserialize, Display, EnumCount, PartialEq,
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

crate::impl_cycle!(Background);

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(default)]
pub struct Stats {
    strength: u32,
    dexterity: u32,
    constitution: u32,
    intelligence: u32,
    wisdom: u32,
    charisma: u32,
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
            strength: 11,
            dexterity: 11,
            constitution: 11,
            intelligence: 11,
            wisdom: 11,
            charisma: 11,
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

#[derive(Default, Serialize, Deserialize, Debug, Clone)]
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
    pub fn from(level: u32, class: &Class) -> Self {
        use Class::*;
        match class {
            Bard | Cleric | Wizard | Sorcerer | Druid => SpellSlots::from_full(level),
            Artificer | Ranger | Paladin => SpellSlots::from_half(level),
            Warlock => SpellSlots::warlock(level),
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

    fn warlock(level: u32) -> Self {
        Self {
            warlock: get_slots!(level, 1, 2, 11, 17),
            ..Self::default()
        }
    }

    pub fn warlock_slot_level(level: u32) -> u32 {
        get_slots!(level, 1, 3, 7, 9)
    }

    pub fn nth(&self, idx: u32, class: &Class) -> u32 {
        if let Class::Warlock = class {
            self.warlock
        } else {
            match idx {
                0 => self.first,
                1 => self.second,
                2 => self.third,
                3 => self.fourth,
                4 => self.fifth,
                5 => self.sixth,
                6 => self.seventh,
                7 => self.eigth,
                8 => self.ninth,
                _ => unreachable!(),
            }
        }
    }

    pub fn nth_mut(&mut self, idx: u32, class: &Class) -> &mut u32 {
        if let Class::Warlock = class {
            &mut self.warlock
        } else {
            match idx {
                0 => &mut self.first,
                1 => &mut self.second,
                2 => &mut self.third,
                3 => &mut self.fourth,
                4 => &mut self.fifth,
                5 => &mut self.sixth,
                6 => &mut self.seventh,
                7 => &mut self.eigth,
                8 => &mut self.ninth,
                _ => unreachable!(),
            }
        }
    }
}

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct Funds {
    pp: u32,
    gp: u32,
    sp: u32,
    cp: u32,
}

impl Funds {
    pub fn nth(&self, idx: u32) -> u32 {
        match idx {
            0 => self.pp,
            1 => self.gp,
            2 => self.sp,
            3 => self.cp,
            _ => unreachable!(),
        }
    }

    pub fn nth_mut(&mut self, idx: u32) -> &mut u32 {
        match idx {
            0 => &mut self.pp,
            1 => &mut self.gp,
            2 => &mut self.sp,
            3 => &mut self.cp,
            _ => unreachable!(),
        }
    }
}

#[derive(Clone, Copy, Serialize, Deserialize, Debug, Default)]
pub enum ProficiencyLevel {
    #[default]
    None,
    Half,
    Normal,
    Double,
}

impl ProficiencyLevel {
    pub fn get_mod(&self, prof_bonus: u32) -> u32 {
        use ProficiencyLevel::*;
        match self {
            None => 0,
            Half => prof_bonus / 2,
            Normal => prof_bonus,
            Double => prof_bonus * 2,
        }
    }
}

pub static SKILL_NAMES: [&str; 18] = [
    "Athletics",
    "Acrobatics",
    "Sleight of Hand",
    "Stealth",
    "Arcana",
    "History",
    "Investigation",
    "Nature",
    "Religion",
    "Animal Handling",
    "Insight",
    "Medicine",
    "Perception",
    "Survival",
    "Deception",
    "Intimidation",
    "Performance",
    "Persuasion",
];

#[derive(Debug, Serialize, Deserialize)]
#[serde(default)]
pub struct Player {
    /// The player's name.
    pub name: String,
    /// The player's class.
    ///
    /// This should not be manually updated, as class dependant
    /// values will not be recalculated; call `Player::update_class()` instead.
    pub class: Class,
    /// The player's level.
    ///
    /// Whenever this is changed, `Player::update_level_dependants()`
    /// should be called to ensure that all calculated values are up to date.
    pub level: u32,
    /// The player's background.
    pub background: Background,
    /// The player's alignment.
    pub alignment: Alignment,
    /// The player's stats.
    ///
    /// Whenever a stat is changed, `Player::update_stat_dependants()`
    /// should be called to ensure that all calculated values are up to date.
    pub stats: Stats,
    /// The player's hit dice value, as calculated using the current class.
    pub hit_dice: u32,
    /// The amount of hit dice the player has remaining.
    pub hit_dice_remaining: u32,
    /// The player's current race.
    ///
    /// Currently, no dependants are calculated using the player's race, so this
    /// is free to be updated as-is.
    pub race: Race,
    /// A vector containing all entries into the inventory tab.
    pub inventory: Vec<String>,
    /// A vector containing all entries into the notes tab.
    pub notes: Vec<String>,
    /// A vector containing all entries into the spells tab.
    pub spells: Vec<String>,
    /// The player's current health.
    pub hp: u32,
    /// The player's armor class.
    pub ac: u32,
    /// The amount of temporary hit points the player has.
    pub temp_hp: u32,
    /// The player's maximum health.
    ///
    /// This value is automatically calculated using the player's class and
    /// level, assuming that avg rolls are used (as they should be).
    pub max_hp: u32,
    /// The player's proficiency bonus.
    ///
    /// This value is automatically calculated using the player's level.
    pub prof_bonus: u32,
    /// The maximum spells slots
    pub spell_slots: SpellSlots,
    /// The current remaining spell slots
    pub spell_slots_remaining: SpellSlots,
    /// The player's current funds,
    pub funds: Funds,
    /// The players skill modifiers
    pub skills: [ProficiencyLevel; 18],
}

/// Get the avg roll value for a given dice.
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
    /// Sets the player's class and updates any class dependant values.
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
        self.spell_slots = SpellSlots::from(self.level, &self.class);
        self.spell_slots_remaining = self.spell_slots.clone();
    }

    /// Updates any level dependant values.
    pub fn update_level_dependants(&mut self) {
        self.hit_dice_remaining = std::cmp::min(self.level, self.hit_dice_remaining + 1);
        self.prof_bonus = (self.level as f32 / 4.0).ceil() as u32 + 1;

        self.update_hp();
        self.spell_slots = SpellSlots::from(self.level, &self.class);
        self.spell_slots_remaining = self.spell_slots.clone();
    }

    /// Updates any stat dependant values.
    pub fn update_stat_dependants(&mut self) {
        self.update_hp();
    }

    /// Recalculates the player's max health and adjusts current
    /// health accordingly.
    fn update_hp(&mut self) {
        let old_max = self.max_hp as i32;
        self.max_hp = self.hit_dice as u32
            + (avg_roll(self.hit_dice) as u32 * (self.level as u32 - 1))
            + ((self.stats.constitution as f32 - 10.0) / 2.0).floor() as u32 * self.level as u32
            + if self.race == Race::HillDwarf { self.level } else { 0 };

        self.hp = self.hp.saturating_add_signed(self.max_hp as i32 - old_max);
    }

    /// Get the stat modifier corresponding to the given skill
    fn get_stat_modifier(&self, skill: u32) -> i32 {
        let val = match skill {
            0 => self.stats.strength,
            1 | 2 | 3 => self.stats.dexterity,
            4 | 5 | 6 | 7 | 8 => self.stats.intelligence,
            9 | 10 | 11 | 12 | 13 => self.stats.wisdom,
            14 | 15 | 16 | 17 => self.stats.charisma,
            _ => unreachable!(),
        };

        ((val as f32 - 10.0) / 2.0).floor() as i32
            + self.skills[skill as usize].get_mod(self.prof_bonus) as i32
    }

    /// Get all of the player's skill modifier values
    pub fn get_skills(&self) -> [i32; 18] {
        let mut skills = [0; 18];
        for (i, skill) in skills.iter_mut().enumerate() {
            *skill = self.get_stat_modifier(i as u32);
        }

        skills
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
            ac: 10,
            temp_hp: 0,
            max_hp: 8,
            prof_bonus: 2,
            spell_slots: SpellSlots::from(1, &Class::Fighter),
            spell_slots_remaining: SpellSlots::from(1, &Class::Fighter),
            funds: Funds::default(),
            skills: [ProficiencyLevel::default(); 18],
        }
    }
}
