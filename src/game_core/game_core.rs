use std::{collections::HashMap, error::Error};
use std::fmt::Display;
use std::path::Path;

use serde::de::Error as SerdeError;
use toml::de::Error as TomlError;

use super::building::BuildingTypeId;
use super::command::EndTurnCommand;
use super::{
    command::CommandExecution, planet::PlanetStatus, BuildingsConfig, BuildingsConfigError, CommandError, CommandLoadError, CommandRegistry, Planet, PlanetError, Player, Turn
};

#[derive(Debug)]
pub enum GameCoreError {
    CommandLoadError(CommandLoadError),
    CommandError(CommandError),
    BuildingConfigError(BuildingsConfigError),
    PlanetError(PlanetError),
}

impl Display for GameCoreError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            GameCoreError::CommandLoadError(err) => write!(f, "Command Load Error: {}", err),
            GameCoreError::BuildingConfigError(err) => write!(f, "Building Config Error: {}", err),
            GameCoreError::CommandError(err) => write!(f, "Command Error: {}", err),
            GameCoreError::PlanetError(err) => write!(f, "Planet Error: {}", err),
        }
    }
}

impl Error for GameCoreError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            GameCoreError::CommandLoadError(err) => Some(err),
            GameCoreError::CommandError(err) => Some(err),
            GameCoreError::BuildingConfigError(_) => None,
            GameCoreError::PlanetError(err) => Some(err),
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
        GameCoreError::BuildingConfigError(err)
    }
}

impl From<PlanetError> for GameCoreError {
    fn from(err: PlanetError) -> Self {
        GameCoreError::PlanetError(err)
    }
}

// =================================================================================================

pub struct GameCore {
    command_registry: CommandRegistry,
    buildings_config: BuildingsConfig,
    turn: Turn,
    current_player: String,
    players: HashMap<String, Player>,
    is_running: bool,
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

        // TODO: Number of players created should be set by the user via ui
        let player1 = Player::new(
            "Player 1", 
            "Planet1", 
            &buildings_config
        );

        Ok(
            GameCore {
                command_registry,
                buildings_config,
                turn: Turn::new(1),
                current_player: "Player 1".to_string(),
                players: HashMap::from([
                    (player1.get_name().to_string(), player1),
                ]),
                is_running: true,
            }
        )
    }

    pub fn is_running(&self) -> bool {
        self.is_running
    }

    pub fn get_current_turn(&self) -> u32 {
        self.turn.get_turn_number()
    }

    pub fn get_current_player_name(&self) -> String {
        self.current_player.clone()
    }

    pub fn get_current_player_planet_names(&self) -> Vec<String> {
        self.players.get(self.current_player.as_str()).map_or(vec![], |player| {
            player.get_planet_names()
        })
    }

    pub fn get_current_player_planet_status(&self, planet_name: &str) -> Option<PlanetStatus> {
        self.players.get(self.current_player.as_str()).and_then(|player| {
            player.get_planet(planet_name).map(|planet| {
                planet.get_status(player.get_planets_count())
            })
        })
    }

    pub fn get_planet_count(&self) -> usize {
        self.players.get(self.current_player.as_str()).map_or(0, |player| {
            player.get_planets_count()
        })
    }
    
    pub fn remove_player(&mut self, player_name: &str) {
        self.players.remove(player_name);
    }

    pub fn execute_command(
        &mut self,
        command: &str,
    ) -> Result<Option<String>, GameCoreError> {
        let command = CommandExecution::parse(&self.command_registry, command)?;
        
        match command {
            CommandExecution::Build(build_command) => {
                let player = self.players.get_mut(&self.current_player).ok_or_else(|| {
                    GameCoreError::CommandError(CommandError::new("Current player not found."))
                })?;

                let planet = player.get_mut_planet(build_command.get_planet()).ok_or_else(|| {
                    GameCoreError::CommandError(
                        CommandError::new(&format!("Planet '{}' not found.", build_command.get_planet()))
                    )
                })?;

                // Find the BuildingTypeId corresponding to the name
                let building_name_to_build = build_command.get_building();
                let target_building_id = BuildingTypeId::all()
                   .iter()
                   .find(|&&id| id.get_name().eq_ignore_ascii_case(building_name_to_build))
                   .cloned()
                   .ok_or_else(
                    || GameCoreError::CommandError(
                            CommandError::new(&format!("Building '{}' not recognized.", building_name_to_build))
                        )
                    )?;


                let building_config = self.buildings_config.buildings.get(
                    target_building_id.get_name()
                ).ok_or_else(|| {
                    // This should ideally not happen if BuildingTypeId::all() is consistent with config keys
                    GameCoreError::BuildingConfigError(
                        BuildingsConfigError::Toml(
                            TomlError::custom(
                                format!("Building '{}' not found in config.", target_building_id.get_name())
                            )
                        )
                    )
                })?;

                planet.build(target_building_id, building_config)?;

                // TODO: Deduct resources from the planet AFTER successful build/upgrade call
                // This part is complex as it needs access to upgrade costs based on the *next* level
                // and mutable access to storage buildings. Needs further implementation.

                Ok(Some(format!("Build command successful for {} on {}.",
                    build_command.get_building(),
                    build_command.get_planet()
                )))
            }
            CommandExecution::EndTurn(_end_turn_command) => { //
                let player = self.players.get_mut(&self.current_player).ok_or_else(|| {
                    GameCoreError::CommandError(CommandError::new("Current player not found."))
                })?;

                player.process_turn_end()?;

                let turn_number = self.turn.get_turn_number();
                self.turn.next_turn();

                // TODO: Handle switching to the next player if multiple players exist

                Ok(Some(format!("Turn {} ended.", turn_number)))
            }
            CommandExecution::Quit(_) => {
                self.is_running = false;
                Ok(Some("Quit command recognized.".to_string()))
            }
            CommandExecution::Help(_) => {
                // TODO: Implement help command
                Ok(Some("Help command recognized.".to_string()))
            }
            CommandExecution::UnknownInternal(_) => {
                Err(GameCoreError::CommandError(CommandError::new("Parsed command is unknown internally.")))
            }
        }
    }
}
