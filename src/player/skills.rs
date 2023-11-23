use serde_derive::{Deserialize, Serialize};

#[derive(Clone, Copy, Serialize, Deserialize, Default)]
pub enum ProficiencyLevel {
    #[default]
    None,
    Half,
    Normal,
    Double,
}

impl ProficiencyLevel {
    pub fn get_mod(&self, prof_bonus: u32) -> u32 {
        use ProficiencyLevel::{Double, Half, None, Normal};
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
