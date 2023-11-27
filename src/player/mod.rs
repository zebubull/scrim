/// Module containing all possible player alignments
pub mod alignment;
/// Module containing all PHB backgrounds
pub mod background;
/// Module containing all classes
pub mod class;
/// Module for keeping track of player funds
pub mod funds;
/// Module containing all PHB races
pub mod race;
/// Module for keeping track of player skill proficiencies
pub mod skills;
/// Module for keeping track of and generating player spell slots
pub mod spells;
/// Module for keeping track of stat values and iterating them
pub mod stats;
/// Module containing various player-related utility functions
pub mod util;

use std::path::Path;

use self::alignment::Alignment;
use self::background::Background;
use self::class::Class;
use self::funds::Funds;
use self::race::Race;
use self::skills::ProficiencyLevel;
use self::spells::SpellSlots;
use self::stats::Stats;
use self::util::{calculate_hp, get_modifier};

use color_eyre::{
    eyre::{eyre, WrapErr},
    Result,
};
use serde_derive::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
#[serde(default)]
pub struct Player {
    /// The player's name.
    pub name: String,
    /// The player's class.
    ///
    /// Whenever this is changed, `Player::recalculate()`
    /// should be called to ensure that all calculated values are up to date.
    pub class: Class,
    /// The player's level.
    ///
    /// Whenever this is changed, `Player::recalculate()`
    /// should be called to ensure that all calculated values are up to date.
    pub level: u32,
    /// The player's background.
    pub background: Background,
    /// The player's alignment.
    pub alignment: Alignment,
    /// The player's stats.
    ///
    /// Whenever a stat is changed, `Player::recalculate()`
    /// should be called to ensure that all calculated values are up to date.
    pub stats: Stats,
    /// The player's hit dice value, as calculated using the current class.
    pub hit_dice: u32,
    /// The amount of hit dice the player has remaining.
    pub hit_dice_remaining: u32,
    /// The player's current race.
    ///
    /// This should be modified via `Player::update_race()` to ensure
    /// that caluclated stat values are properly updated.
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
    /// level, assuming average rolls are used (as they should be).
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

impl Player {
    pub fn load(path: &Path) -> Result<Self> {
        let bytes = std::fs::read(path).wrap_err_with(|| {
            format!(
                "failed to read file '{}'",
                path.to_str().unwrap_or(&path.to_string_lossy())
            )
        })?;

        let ext = path.extension().unwrap_or_default();
        let player = match ext.to_str().unwrap_or(&ext.to_string_lossy()) {
            // Legacy .player files always use JSON format
            "json" | "player" => serde_json::from_slice(bytes.as_slice()).wrap_err_with(|| {
                format!(
                    "failed to parse file '{}'",
                    path.to_str().unwrap_or(&path.to_string_lossy())
                )
            }),
            "yaml" => serde_yaml::from_slice(bytes.as_slice()).wrap_err_with(|| {
                format!(
                    "failed to parse file '{}'",
                    path.to_str().unwrap_or(&path.to_string_lossy())
                )
            }),
            _ => Err(eyre!(format!(
                "file '{}' has unknown or unsupported file type",
                path.to_str().unwrap_or(&path.to_string_lossy())
            ))),
        }?;

        Ok(player)
    }

    /// Recalculates auto-generated values
    pub fn recalculate(&mut self) {
        self.hit_dice = self.class.hit_dice();

        self.hit_dice_remaining = std::cmp::min(self.level, self.hit_dice_remaining + 1);
        self.prof_bonus = (self.level as f32 / 4.0).ceil() as u32 + 1;

        self.spell_slots = SpellSlots::from(self.level, &self.class);
        self.spell_slots_remaining = self.spell_slots.clone();

        self.update_hp();
    }

    pub fn update_race(&mut self, race: Race) {
        let old_race = self.race;
        self.race = race;

        self.stats -= old_race.stats();
        self.stats += race.stats();

        self.recalculate();
    }

    /// Recalculates the player's max health and adjusts current
    /// health accordingly.
    fn update_hp(&mut self) {
        let old_max = self.max_hp;
        self.max_hp = calculate_hp(
            self.level,
            self.hit_dice,
            self.stats.constitution,
            self.race.health_bonus(),
        );

        self.hp = self
            .hp
            .saturating_add_signed(self.max_hp as i32 - old_max as i32);
    }

    /// Get the stat modifier corresponding to the given skill
    fn get_skill_modifier(&self, skill: u32) -> i32 {
        let val = match skill {
            0 => self.stats.strength,
            1 | 2 | 3 => self.stats.dexterity,
            4 | 5 | 6 | 7 | 8 => self.stats.intelligence,
            9 | 10 | 11 | 12 | 13 => self.stats.wisdom,
            14 | 15 | 16 | 17 => self.stats.charisma,
            _ => panic!("skill index '{}' out of range", skill),
        };

        get_modifier(val) + self.skills[skill as usize].get_mod(self.prof_bonus) as i32
    }

    /// Get all of the player's skill modifier values
    pub fn get_skills(&self) -> [i32; 18] {
        let mut skills = [0; 18];
        for (i, skill) in skills.iter_mut().enumerate() {
            *skill = self.get_skill_modifier(i as u32);
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
            hp: 10,
            ac: 10,
            max_hp: 10,
            prof_bonus: 2,
            spell_slots: SpellSlots::from(1, &Class::Fighter),
            spell_slots_remaining: SpellSlots::from(1, &Class::Fighter),
            name: String::default(),
            class: Class::default(),
            background: Background::default(),
            alignment: Alignment::default(),
            stats: Stats::default(),
            race: Race::default(),
            inventory: vec![],
            notes: vec![],
            spells: vec![],
            temp_hp: 0,
            funds: Funds::default(),
            skills: [ProficiencyLevel::default(); 18],
        }
    }
}
