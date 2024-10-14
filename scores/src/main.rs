use std::{error, ops::AddAssign, sync::Mutex};

use scores::bowling::{game::BowlingGame, frame::Frame, error::Error};
use rand::{thread_rng, Rng};

fn turn(mut bowling_game: BowlingGame) -> Result<(), Error> {
    let mut threadrng = thread_rng();
    let counter = Mutex::new(0);

    for i in 1..=10 {
        let random_roll1 = Rng::gen_range(&mut threadrng, 0..=10);
        let pins = 10 - random_roll1;
        let random_roll2 = Rng::gen_range(&mut threadrng, 0..=pins);
        let frame = Frame::new(random_roll1, random_roll2);
        let score = random_roll1 + random_roll2;
        
        counter.lock().unwrap().add_assign(score);

        bowling_game.score = score;
        bowling_game.frame(frame.roll1, frame.roll2)?;

        println!("Game: {} with score: {:?}", i, bowling_game.calculate_score());
        println!("Frame: {} {:?}\n", i, frame);
        
        bowling_game.frames.push(frame);
    }

    println!("End score: {:?}", counter);
    Ok(())
}

fn main() -> Result<(), Box<dyn error::Error>> {
    // code here;
    let bowling_game = BowlingGame::default();
    
    turn(bowling_game).unwrap();
    Ok(())
}
