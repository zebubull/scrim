use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(default)]
pub struct Stats {
    pub(super) strength: u32,
    pub(super) dexterity: u32,
    pub(super) constitution: u32,
    pub(super) intelligence: u32,
    pub(super) wisdom: u32,
    pub(super) charisma: u32,
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

impl std::ops::Index<usize> for Stats {
    type Output = u32;

    fn index(&self, index: usize) -> &Self::Output {
        match index {
            0 => &self.strength,
            1 => &self.dexterity,
            2 => &self.constitution,
            3 => &self.intelligence,
            4 => &self.wisdom,
            5 => &self.charisma,
            _ => panic!("stat index out of range"),
        }
    }
}

impl std::ops::IndexMut<usize> for Stats {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        match index {
            0 => &mut self.strength,
            1 => &mut self.dexterity,
            2 => &mut self.constitution,
            3 => &mut self.intelligence,
            4 => &mut self.wisdom,
            5 => &mut self.charisma,
            _ => panic!("stat index out of range"),
        }
    }
}

impl<'a> IntoIterator for &'a Stats {
    type Item = u32;
    type IntoIter = StatsIter<'a>;

    fn into_iter(self) -> Self::IntoIter {
        StatsIter {
            stats: self,
            idx: 0,
        }
    }
}

pub struct StatsIter<'a> {
    stats: &'a Stats,
    idx: usize,
}

impl<'a> Iterator for StatsIter<'a> {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        let val = if self.idx <= 5 {
            Some(self.stats[self.idx])
        } else {
            None
        };
        self.idx += 1;
        val
    }
}

impl std::ops::Add<Stats> for &Stats {
    type Output = Stats;

    fn add(self, rhs: Stats) -> Self::Output {
        Stats {
            strength: self.strength + rhs.strength,
            dexterity: self.dexterity + rhs.dexterity,
            constitution: self.constitution + rhs.constitution,
            intelligence: self.intelligence + rhs.intelligence,
            wisdom: self.wisdom + rhs.wisdom,
            charisma: self.charisma + rhs.charisma,
        }
    }
}

impl std::ops::Sub<Stats> for &Stats {
    type Output = Stats;

    fn sub(self, rhs: Stats) -> Self::Output {
        Stats {
            strength: self.strength - rhs.strength,
            dexterity: self.dexterity - rhs.dexterity,
            constitution: self.constitution - rhs.constitution,
            intelligence: self.intelligence - rhs.intelligence,
            wisdom: self.wisdom - rhs.wisdom,
            charisma: self.charisma - rhs.charisma,
        }
    }
}

impl std::ops::AddAssign<Stats> for Stats {
    fn add_assign(&mut self, rhs: Stats) {
        *self = &*self + rhs;
    }
}

impl std::ops::SubAssign<Stats> for Stats {
    fn sub_assign(&mut self, rhs: Stats) {
        *self = &*self - rhs;
    }
}
