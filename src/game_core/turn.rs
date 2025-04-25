pub struct Turn {
    turn_number: u32,
}

impl Turn {
    pub fn new(turn_number: u32) -> Self {
        Turn { turn_number }
    }
    
    pub fn get_turn_number(&self) -> u32 {
        self.turn_number
    }

    pub fn next_turn(&mut self) {
        self.turn_number += 1;
    }

    pub fn reset_turn(&mut self) {
        self.turn_number = 1;
    }
}