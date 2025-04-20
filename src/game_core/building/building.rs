use std::fmt;
use std::error::Error;

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

pub trait Building {
    fn get_name(&self) -> &String;
    fn get_level(&self) -> u8;
    fn get_max_level(&self) -> u8;
    fn upgrade(&mut self) -> Result<(), BuildingError>;
}

#[derive(Debug, PartialEq, Eq, Hash)]
pub enum BuildingTypeId {
    CommandCenter,
    OrbitalShipyard,
    ResearchLab,
    FusionReactor,
    GasExtractor,
    MineralMine,
    BatteryArray,
    GasTank,
    MineralStorage,
}

#[derive(Debug, Clone)]
pub enum BuildingType {
    CommandCenter(BuildingBase),
    OrbitalShipyard(BuildingBase),
    ResearchLab(BuildingBase),
    FusionReactor(Productor),
    GasExtractor(Productor),
    MineralMine(Productor),
    BatteryArray(Storage),
    GasTank(Storage),
    MineralStorage(Storage),
}

impl BuildingType {
    pub fn get_id(&self) -> BuildingTypeId {
        match self {
            Self::CommandCenter(_) => BuildingTypeId::CommandCenter,
            Self::OrbitalShipyard(_) => BuildingTypeId::OrbitalShipyard,
            Self::ResearchLab(_) => BuildingTypeId::ResearchLab,
            Self::FusionReactor(_) => BuildingTypeId::FusionReactor,
            Self::GasExtractor(_) => BuildingTypeId::GasExtractor,
            Self::MineralMine(_) => BuildingTypeId::MineralMine,
            Self::BatteryArray(_) => BuildingTypeId::BatteryArray,
            Self::GasTank(_) => BuildingTypeId::GasTank,
            Self::MineralStorage(_) => BuildingTypeId::MineralStorage,
        }
    }
}

impl Building for BuildingType {
    fn get_name(&self) -> &String {
        match self {
            Self::CommandCenter(building)
            | Self::OrbitalShipyard(building)
            | Self::ResearchLab(building) => &building.name,
            Self::FusionReactor(productor)
            | Self::GasExtractor(productor)
            | Self::MineralMine(productor) => &productor.building.name,
            Self::BatteryArray(storage)
            | Self::GasTank(storage)
            | Self::MineralStorage(storage) => &storage.building.name,
        }
    }

    fn get_level(&self) -> u8 {
        match self {
            Self::CommandCenter(building)
            | Self::OrbitalShipyard(building)
            | Self::ResearchLab(building) => building.level,
            Self::FusionReactor(productor)
            | Self::GasExtractor(productor)
            | Self::MineralMine(productor) => productor.building.level,
            Self::BatteryArray(storage)
            | Self::GasTank(storage)
            | Self::MineralStorage(storage) => storage.building.level,
        }
    }

    fn get_max_level(&self) -> u8 {
        match self {
            Self::CommandCenter(building)
            | Self::OrbitalShipyard(building)
            | Self::ResearchLab(building) => building.max_level,
            Self::FusionReactor(productor)
            | Self::GasExtractor(productor)
            | Self::MineralMine(productor) => productor.building.max_level,
            Self::BatteryArray(storage)
            | Self::GasTank(storage)
            | Self::MineralStorage(storage) => storage.building.max_level,
        }
    }

    fn upgrade(&mut self) -> Result<(), BuildingError> {
        match self {
            Self::CommandCenter(building)
            | Self::OrbitalShipyard(building)
            | Self::ResearchLab(building) => building.upgrade(),
            Self::FusionReactor(productor)
            | Self::GasExtractor(productor)
            | Self::MineralMine(productor) => productor.building.upgrade(),
            Self::BatteryArray(storage)
            | Self::GasTank(storage)
            | Self::MineralStorage(storage) => storage.building.upgrade(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct BuildingBase {
    name: String,
    level: u8,
    max_level: u8,
}

impl BuildingBase {
    fn upgrade(&mut self) -> Result<(), BuildingError> {
        if self.level >= self.max_level {
            return Err(BuildingError::MaxLevelReached {
                current: self.level,
                max: self.max_level,
            });
        }
        
        self.level += 1;
        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct Productor {
    building: BuildingBase,
    resource: Resource,
    production_rate: u32,
}

impl Productor {
    pub fn get_production_rate(&self) -> u32 {
        self.production_rate
    }
}

#[derive(Debug, Clone)]
pub struct Storage {
    building: BuildingBase,
    resource: Resource,
    capacity: u32,
    current_amount: u32,
}

impl Storage {
    pub fn get_capacity(&self) -> u32 {
        self.capacity
    }

    pub fn get_current_amount(&self) -> u32 {
        self.current_amount
    }
}
