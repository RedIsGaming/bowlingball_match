use std::{error, ops::{Add, AddAssign, Sub}, sync::Mutex};
use scores::bowling::{error::Error, frame::Frame, game::BowlingGame};
use rand::{thread_rng, Rng};

const MAX_FRAMES_TURNS: u32 = 10;

fn random_frame_number(pins: u32) -> u32 {
    let mut threadrng = thread_rng();
    Rng::gen_range(&mut threadrng, 0..=pins)
}

fn call_strike<'a>(
    bowling_game: &'a mut BowlingGame, bonus_frame: Frame, counter: &Mutex<u32>
) -> Result<&'a BowlingGame, Error> {
    BowlingGame::print_turn(bowling_game, &bonus_frame, counter, bonus_frame.roll1.add(bonus_frame.roll2));
    BowlingGame::bowling(bowling_game, bonus_frame, counter, bonus_frame.roll1.add(bonus_frame.roll2))?;
    BowlingGame::end_turn(bowling_game);

    Ok(bowling_game)
}

fn call_spare<'a>(
    bowling_game: &'a mut BowlingGame, mut bonus_frame: Frame, counter: &Mutex<u32>
) -> Result<&'a mut BowlingGame, Error>{
    bonus_frame.roll2 = 0;

    BowlingGame::print_turn(bowling_game, &bonus_frame, counter, bonus_frame.roll1);
    BowlingGame::bowling(bowling_game, bonus_frame, counter, bonus_frame.roll1)?;
    BowlingGame::end_turn(bowling_game);

    Ok(bowling_game)
}

fn game_turn(bowling_game: &mut BowlingGame) {
    let counter = Mutex::new(0);

    for _ in bowling_game.current_frame..=MAX_FRAMES_TURNS as usize {
        let random_roll1 = random_frame_number(MAX_FRAMES_TURNS);
        let pins = MAX_FRAMES_TURNS.sub(random_roll1);
        let random_roll2 = random_frame_number(pins);
        let frame = Frame::new(random_roll1, random_roll2);
        
        counter.try_lock().unwrap().add_assign(frame.roll1.add(frame.roll2));

        BowlingGame::print_turn(bowling_game, &frame, &counter, Default::default());
        BowlingGame::bowling(bowling_game, frame, &counter, Default::default()).unwrap();
    }
}

fn turn(mut bowling_game: BowlingGame) -> Result<BowlingGame, Error> {
    let counter = Mutex::new(0);
    let random_roll1 = random_frame_number(MAX_FRAMES_TURNS);
    let pins = MAX_FRAMES_TURNS.sub(random_roll1);
    let random_roll2 = random_frame_number(pins);
    let bonus_frame = Frame::new(random_roll1, random_roll2);
    
    bowling_game.current_frame = 1;
    
    println!("The bowling ball match has started. Your start score is: {}\n", counter.lock().unwrap());
    game_turn(&mut bowling_game);

    let strike = Frame::strike_iter(&bowling_game);
    let spare = Frame::spare_iter(&bowling_game);

    if strike.is_some() {
        println!("Strike! 2 bonus rolls left.");
        call_strike(&mut bowling_game, bonus_frame, &counter)?;
    }

    else if spare.is_some() {
        println!("Spare! 1 bonus roll left.");
        call_spare(&mut bowling_game, bonus_frame, &counter)?;
    }

    BowlingGame::end_turn(&bowling_game);
    Ok(bowling_game)
}

fn main() -> Result<(), Box<dyn error::Error>> {
    // code here;
    let bowling_game = BowlingGame::default();
    turn(bowling_game).unwrap_or_default();

    Ok(())
}
