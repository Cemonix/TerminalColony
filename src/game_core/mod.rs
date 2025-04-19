pub mod game_core;

mod command;
mod resources;
mod building;
mod planet;
mod player;

// =================================================================================================

pub use game_core::GameCore;
pub use command::CommandLoadError;

use command::{CommandRegistry, CommandError};
use resources::ResourceType;
use building::{Building, BuildingBase, BuildingDefinitionId, BuildingError, BuildingBehavior, ResourceGenerator, StorageProvider};
use planet::Planet;
use player::Player;