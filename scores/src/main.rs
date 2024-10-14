use std::{error, ops::{Add, AddAssign}, sync::Mutex};

use scores::bowling::{game::BowlingGame, frame::Frame, error::Error, state::State};
use rand::{thread_rng, Rng};

fn bowling<'a>(
    bowling_game: &'a mut BowlingGame, 
    frame: Frame, 
    counter: &Mutex<u32>, 
    state: Option<u32>
) -> Result<&'a BowlingGame, Error> {
    bowling_game.frame(frame.roll1, frame.roll2)?;
    bowling_game.frames.push(frame);
    bowling_game.current_frame.add_assign(1);
    bowling_game.score = *counter.lock().unwrap() + state.unwrap_or_default();

    Ok(bowling_game)
}

fn print_turn(bowling_game: &BowlingGame, frame: &Frame, counter: &Mutex<u32>, state: Option<u32>) -> Result<(), Error> {
    println!("The current frame is: {}|{}", frame.roll1, frame.roll2);
    println!("The current match is on round: {} with score: {}/10. The total score is: {}\n", 
        bowling_game.current_frame, 
        frame.roll1 + frame.roll2, 
        *counter.lock().unwrap() + state.unwrap_or_default()
    );

    Ok(())
}

fn turn(mut bowling_game: BowlingGame) -> Result<BowlingGame, Error> {
    let mut threadrng = thread_rng();
    let counter = Mutex::new(0);
    
    bowling_game.current_frame = 1;
    println!("The bowling ball match has started. Your start score is: {}\n", counter.lock().unwrap());

    for round in bowling_game.current_frame..=10 {
        let random_roll1 = Rng::gen_range(&mut threadrng, 0..=10);
        let pins = 10 - &random_roll1;
        let random_roll2 = Rng::gen_range(&mut threadrng, 0..=pins);
        let frame = Frame::new(random_roll1, random_roll2);
        
        counter
            .try_lock()
            .unwrap()
            .add_assign(frame.roll1.add(frame.roll2));
        
        let score1 = frame.roll1;
        let score2 = frame.roll2;
        #[allow(unused_assignments)]
        let mut state = Some(0);

        if round.eq(&10) && score1.eq(&10) { 
            state = State::pin(State::Strike); 
        }
        
        else if round.eq(&10) && score1.add(score2).eq(&10) { state = State::pin(State::Spare); }
        else { state = State::pin(State::Open); }

        print_turn(&bowling_game, &frame, &counter, state)?;
        bowling(&mut bowling_game, frame, &counter, state)?;
    }
    
    let total_score = bowling_game.frames
        .iter()
        .map(|x| x.roll1.add(x.roll2))
        .collect::<Vec<u32>>();

    let end_score = total_score
        .iter()
        .sum::<u32>();

    bowling_game.end_score();
    println!("The bowling ball match has ended. Your end score is: {}", end_score);

    Ok(bowling_game)
}

fn main() -> Result<(), Box<dyn error::Error>> {
    // code here;
    let bowling_game = BowlingGame::default();
    turn(bowling_game).unwrap_or_default();

    Ok(())
}
