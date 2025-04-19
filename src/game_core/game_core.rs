use std::error::Error;
use std::fmt::Display;
use std::path::Path;

use super::building::building_config::BuildingsConfigError;
use super::building::BuildingsConfig;
use super::command::CommandError;
use super::CommandLoadError;
use super::CommandRegistry;
use super::Planet;
use super::Player;

#[derive(Debug)]
pub enum GameCoreError {
    CommandLoadError(CommandLoadError),
    CommandError(CommandError),
    BuildingConfigError(String),
}

impl Display for GameCoreError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            GameCoreError::CommandLoadError(err) => write!(f, "Command Load Error: {}", err),
            GameCoreError::CommandError(err) => write!(f, "Command Error: {}", err),
            GameCoreError::BuildingConfigError(err) => write!(f, "Building Config Error: {}", err),
        }
    }
}

impl Error for GameCoreError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            GameCoreError::CommandLoadError(err) => Some(err),
            GameCoreError::CommandError(err) => Some(err),
            GameCoreError::BuildingConfigError(_) => None,
        }
    }
}

impl From<CommandLoadError> for GameCoreError {
    fn from(err: CommandLoadError) -> Self {
        GameCoreError::CommandLoadError(err)
    }
}

impl From<CommandError> for GameCoreError {
    fn from(err: CommandError) -> Self {
        GameCoreError::CommandError(err)
    }
}

impl From<BuildingsConfigError> for GameCoreError {
    fn from(err: BuildingsConfigError) -> Self {
        GameCoreError::BuildingConfigError(err.to_string())
    }
}

// =================================================================================================

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
    buildings_config: BuildingsConfig,
    turn: Turn,
    current_player: String,
    players: Vec<Player>,
    planets: Vec<Planet>,
}

impl GameCore {
    pub fn new(
        command_registry_path: Option<&Path>,
        buildings_config_path: Option<&Path>,
    ) -> Result<Self, GameCoreError>  {
        let command_registry = match command_registry_path {
            Some(path) => CommandRegistry::load(path)?,
            None => CommandRegistry::load(Path::new("data/commands.toml"))?,
        };

        let buildings_config = match buildings_config_path {
            Some(path) => BuildingsConfig::load(path)?,
            None => BuildingsConfig::load(Path::new("data/buildings.toml"))?,
        };

        Ok(
            GameCore {
                command_registry,
                buildings_config,
                turn: Turn::new(0),
                current_player: String::new(),
                players: Vec::new(),
                planets: Vec::new(),
            }
        )
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