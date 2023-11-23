use serde_derive::{Deserialize, Serialize};

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
