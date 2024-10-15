use std::ops::Add;
use crate::bowling::game::BowlingGame;

#[derive(Debug, Default, Clone, Copy)]
pub struct Frame {
    pub roll1: u32,
    pub roll2: u32,
}

const MAX_FRAMES_TURNS: u32 = 10;

impl Frame {
    pub fn new(roll1: u32, roll2: u32) -> Self {
        Self { 
            roll1, 
            roll2, 
        }
    }

    pub fn strike_iter(bowling_game: &BowlingGame) -> Option<&Frame> {
        bowling_game.frames
            .iter()
            .last()
            .filter(|x| x.roll1.eq(&MAX_FRAMES_TURNS))
    }
    
    pub fn spare_iter(bowling_game: &BowlingGame) -> Option<&Frame> {
        bowling_game.frames
            .iter()
            .last()
            .filter(|x| x.roll1.add(x.roll2).eq(&MAX_FRAMES_TURNS))
    }
}
