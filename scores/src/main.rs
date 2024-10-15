use std::{error, ops::{Add, AddAssign, Sub}, sync::Mutex};
use scores::bowling::{error::Error, frame::Frame, game::BowlingGame};
use rand::{thread_rng, Rng};

const MAX_FRAMES_TURNS: u32 = 10;

fn bowling<'a>(
    bowling_game: &'a mut BowlingGame, frame: Frame, counter: &Mutex<u32>, bonus_frame: u32
) -> Result<&'a BowlingGame, Error> {
    bowling_game.frame(frame.roll1, frame.roll2)?;
    bowling_game.frames.push(frame);
    bowling_game.current_frame.add_assign(1);
    bowling_game.score = *counter.lock().unwrap() + bonus_frame;

    Ok(bowling_game)
}

fn print_turn(bowling_game: &BowlingGame, frame: &Frame, counter: &Mutex<u32>, bonus_frame: u32) {
    println!("The current frame is: {}|{}", frame.roll1, frame.roll2);
    println!("The current match is on round: {} with score: {}/10. The total score is: {}\n", 
        bowling_game.current_frame, 
        frame.roll1 + frame.roll2, 
        *counter.lock().unwrap() + bonus_frame
    )
}

fn end_turn(bowling_game: &BowlingGame) {
    println!("The bowling ball match has ended. Your end score is: {}", bowling_game.end_score())
}

fn random_frame_number(pins: u32) -> u32 {
    let mut threadrng = thread_rng();
    Rng::gen_range(&mut threadrng, 0..=pins)
}

fn strike_iter(bowling_game: &BowlingGame) -> Option<&Frame> {
    bowling_game.frames.iter().last().filter(|x| x.roll1.eq(&MAX_FRAMES_TURNS))
}

fn spare_iter(bowling_game: &BowlingGame) -> Option<&Frame> {
    bowling_game.frames.iter().last().filter(|x| x.roll1.add(x.roll2).eq(&MAX_FRAMES_TURNS))
}

fn turn(mut bowling_game: BowlingGame) -> Result<BowlingGame, Error> {
    let counter = Mutex::new(0);
    let mut random_roll1 = random_frame_number(MAX_FRAMES_TURNS);
    let mut pins = MAX_FRAMES_TURNS.sub(random_roll1);
    let mut random_roll2 = random_frame_number(pins);
    let mut bonus_frame = Frame::new(random_roll1, random_roll2);
    
    bowling_game.current_frame = 1;
    println!("The bowling ball match has started. Your start score is: {}\n", counter.lock().unwrap());

    for _ in bowling_game.current_frame..=MAX_FRAMES_TURNS as usize {
        random_roll1 = random_frame_number(MAX_FRAMES_TURNS);
        pins = MAX_FRAMES_TURNS.sub(random_roll1);
        random_roll2 = random_frame_number(pins);
        let frame = Frame::new(random_roll1, random_roll2);
        
        counter.try_lock().unwrap().add_assign(frame.roll1.add(frame.roll2));

        print_turn(&bowling_game, &frame, &counter, Default::default());
        bowling(&mut bowling_game, frame, &counter, Default::default())?;
    }

    let strike = strike_iter(&bowling_game);
    let spare = spare_iter(&bowling_game);

    if strike.is_some() {
        println!("Strike! 2 bonus rolls left.");
        print_turn(&bowling_game, &bonus_frame, &counter, bonus_frame.roll1.add(bonus_frame.roll2));
        bowling(&mut bowling_game, bonus_frame, &counter, bonus_frame.roll1.add(bonus_frame.roll2))?;
        end_turn(&bowling_game);

        return Ok(bowling_game);
    }

    else if spare.is_some() {
        bonus_frame.roll2 = 0;

        println!("Spare! 1 bonus roll left.");
        print_turn(&bowling_game, &bonus_frame, &counter, bonus_frame.roll1);
        bowling(&mut bowling_game, bonus_frame, &counter, bonus_frame.roll1)?;
        end_turn(&bowling_game);

        return Ok(bowling_game);
    }

    end_turn(&bowling_game);
    Ok(bowling_game)
}

fn main() -> Result<(), Box<dyn error::Error>> {
    // code here;
    let bowling_game = BowlingGame::default();
    turn(bowling_game).unwrap_or_default();

    Ok(())
}
