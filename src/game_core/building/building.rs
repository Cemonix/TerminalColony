use std::fmt;
use std::error::Error;

use toml::value::Array;

use crate::game_core::resource::Resource;

#[derive(Debug, PartialEq, Eq)]
pub enum BuildingError {
    MaxLevelReached { current: u8, max: u8 },
    InsufficientResources { required: u32, available: u32 },
}

impl fmt::Display for BuildingError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            BuildingError::MaxLevelReached { current, max } => 
                write!(f, "Cannot upgrade: level {current} is at max {max}"),
            BuildingError::InsufficientResources { required, available } => 
                write!(f, "Insufficient resources: need {required}, have {available}"),
        }
    }
}

impl Error for BuildingError {}

// =================================================================================================

#[derive(Debug, Clone)]
pub enum BuildingType {
    CommandCenter(Building),
    FusionReactor(Productor),
    BatteryArray(Storage),
    GasExtractor(Productor),
    GasTank(Storage),
    MineralMine(Productor),
    MineralStorage(Storage),
    OrbitalShipyard(Building),
    ResearchLab(Building),
}

#[derive(Debug, Clone)]
pub struct Building {
    name: String,
    level: u8,
    max_level: u8
}

impl Building {
    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn get_level(&self) -> u8 {
        self.level
    }
}

#[derive(Debug, Clone)]
struct Productor {
    building: Building,
    resource: Resource,
    production_rate: u32,
}

#[derive(Debug, Clone)]
struct Storage {
    building: Building,
    resource: Resource,
    capacity: u32,
    current_amount: u32,
}
