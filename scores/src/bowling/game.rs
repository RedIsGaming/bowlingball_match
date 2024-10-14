use crate::bowling::{frame::*, error::*};

#[derive(Debug, Default)]
pub struct BowlingGame {
    pub frames: Vec<Frame>,
    pub current_frame: usize,
    pub score: u32,
    pub ended: bool,
}

impl BowlingGame {
    pub fn frame(&mut self, roll1: u32, roll2: u32) -> Result<u32, Error> {
        if self.ended {
            return Err(Error::GameAlreadyEnded);
        }
        if self.current_frame <= 10 && roll1 + roll2 > 10 {
            return Err(Error::InvalidFrame);
        }
        Ok(0)
    }

    pub fn end_score(&self) -> u32 {
        if self.ended {
            0
        } else {
            self.score
        }
    }

    pub fn calculate_score(&self) -> usize {
        self.current_frame + self.score as usize
    }
}
