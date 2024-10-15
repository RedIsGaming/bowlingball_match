use std::{ops::AddAssign, sync::Mutex};
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
            return 0;
        }

        self.score
    }

    pub fn bowling<'a>(
        bowling_game: &'a mut BowlingGame, frame: Frame, counter: &Mutex<u32>, bonus_frame: u32
    ) -> Result<&'a BowlingGame, Error> {
        bowling_game.frame(frame.roll1, frame.roll2)?;
        bowling_game.frames.push(frame);
        bowling_game.current_frame.add_assign(1);
        bowling_game.score = *counter.lock().unwrap() + bonus_frame;
    
        Ok(bowling_game)
    }
    
    pub fn print_turn(bowling_game: &BowlingGame, frame: &Frame, counter: &Mutex<u32>, bonus_frame: u32) {
        println!("The current frame is: {}|{}", frame.roll1, frame.roll2);
        println!("The current match is on round: {} with score: {}/10. The total score is: {}\n", 
            bowling_game.current_frame, 
            frame.roll1 + frame.roll2, 
            *counter.lock().unwrap() + bonus_frame
        )
    }

    pub fn end_turn(bowling_game: &BowlingGame) {
        println!("The bowling ball match has ended. Your end score is: {}", bowling_game.end_score())
    }
}
