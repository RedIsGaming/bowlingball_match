use rand::{thread_rng, Rng};

#[derive(Debug, Default)]
pub enum State {
    Strike,
    Spare,
    #[default] Open,
}

impl State {
    pub fn pin(state: State) -> Option<u32> {
        let mut threadrng = thread_rng();
        let random_roll1 = Rng::gen_range(&mut threadrng, 0..=10);
        let pins = 10 - &random_roll1;
        let random_roll2 = Rng::gen_range(&mut threadrng, 0..=pins);

        match state {
            State::Strike => {
                println!("The last turn was strike!");
                Some(random_roll1 + random_roll2)
            },
            State::Spare => {
                println!("The last turn was spare!"); 
                Some(random_roll1)
            },
            State::Open => None,
        }
    }
}
