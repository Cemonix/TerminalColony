pub mod game_core;

mod command;
mod resource;
mod turn;
mod building;
mod planet;
mod player;

// =================================================================================================

pub use game_core::{GameCore, GameCoreError};
pub use command::CommandLoadError;
pub use planet::PlanetStatus;
pub use resource::Resource;

use command::{CommandRegistry, CommandError};
use turn::Turn;
use building::{
    BuildingsConfig,
    BuildingConfig,
    BuildingsConfigError,
    BuildingTypeId,
    BuildingType,
    BuildingBase,
    Productor,
    Storage,
    BuildingError
};
use planet::{Planet, PlanetError};
use player::Player;