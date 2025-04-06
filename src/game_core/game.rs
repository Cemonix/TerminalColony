use super::planet::Planet;
use super::player::Player;

struct Turn {
    turn_number: u32,
}

impl Turn {
    fn new(turn_number: u32) -> Self {
        Turn { turn_number }
    }
    
    fn get_turn_number(&self) -> u32 {
        self.turn_number
    }

    fn next_turn(&mut self) {
        self.turn_number += 1;
    }

    fn reset_turn(&mut self) {
        self.turn_number = 0;
    }
}

pub struct Game {
    turn: Turn,
    current_player: String,
    players: Vec<Player>,
    planets: Vec<Planet>,
}

impl Game {
    pub fn new() -> Self {
        Game {
            turn: Turn::new(0),
            current_player: String::new(),
            players: Vec::new(),
            planets: Vec::new(),
        }
    }

    pub fn run(&mut self) {
        loop {
            // Game logic goes here
            // For example, you can call next_turn() to advance the game
            self.turn.next_turn();
            println!("Turn: {}", self.turn.get_turn_number());
            
            // Break condition for the loop (for demonstration purposes)
            if self.turn.get_turn_number() >= 10 {
                break;
            }
        }
    }
    
    fn add_player(&mut self, player: Player) {
        self.players.push(player);
    }

    fn remove_player(&mut self, player_name: &str) {
        self.players.retain(|player| player.get_name() != player_name);
    }

}