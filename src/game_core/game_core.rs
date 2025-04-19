use super::CommandRegistry;
use super::Planet;
use super::Player;

pub struct Turn {
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

pub struct GameCore {
    command_registry: CommandRegistry,
    turn: Turn,
    current_player: String,
    players: Vec<Player>,
    planets: Vec<Planet>,
}

impl GameCore {
    pub fn new(command_registry_path: Option<&str>) -> Self {
        let command_registry = match command_registry_path {
            Some(path) => CommandRegistry::load(path).unwrap(),
            None => CommandRegistry::load("data/commands.toml").unwrap(),
        };
        GameCore {
            command_registry,
            turn: Turn::new(0),
            current_player: String::new(),
            players: Vec::new(),
            planets: Vec::new(),
        }
    }

    pub fn get_turn(&self) -> &Turn {
        &self.turn
    }
    
    pub fn add_player(&mut self, player: Player) {
        self.players.push(player);
    }

    pub fn remove_player(&mut self, player_name: &str) {
        self.players.retain(|player| player.get_name() != player_name);
    }
}