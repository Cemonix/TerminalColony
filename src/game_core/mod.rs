pub mod game_core;

mod command;
mod resource;
mod turn;
mod building;
mod planet;
mod player;

// =================================================================================================

pub use game_core::GameCore;
pub use game_core::GameCoreError;
pub use command::CommandLoadError;

use command::{CommandRegistry, CommandError};
use resource::Resource;
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