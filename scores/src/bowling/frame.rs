#[derive(Debug, Default, Clone, Copy)]
pub struct Frame {
    pub roll1: u32,
    pub roll2: u32,
}

impl Frame {
    pub fn new(roll1: u32, roll2: u32) -> Self {
        Self { 
            roll1, 
            roll2, 
        }
    }
}
