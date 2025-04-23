use std::fmt;
use std::error::Error;

use crate::game_core::Resource;

use super::BuildingConfig;

#[derive(Debug, PartialEq, Eq)]
pub enum BuildingError {
    WrongBuildingConfiguration,
    MaxLevelReached { current: u8, max: u8 },
    InsufficientResources { required: u32, available: u32 },
}

impl fmt::Display for BuildingError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            BuildingError::WrongBuildingConfiguration => 
                write!(f, "Wrong building configuration"),
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
    fn get_name(&self) -> &str;
    fn get_level(&self) -> u8;
    fn upgrade(&mut self) -> Result<(), BuildingError>;
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
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

impl fmt::Display for BuildingTypeId {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::CommandCenter => write!(f, "Command Center"),
            Self::OrbitalShipyard => write!(f, "Orbital Shipyard"),
            Self::ResearchLab => write!(f, "Research Lab"),
            Self::FusionReactor => write!(f, "Fusion Reactor"),
            Self::GasExtractor => write!(f, "Gas Extractor"),
            Self::MineralMine => write!(f, "Mineral Mine"),
            Self::BatteryArray => write!(f, "Battery Array"),
            Self::GasTank => write!(f, "Gas Tank"),
            Self::MineralStorage => write!(f, "Mineral Storage"),
        }
    }
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
    fn get_name(&self) -> &str {
        match self {
            Self::CommandCenter(building)
            | Self::OrbitalShipyard(building)
            | Self::ResearchLab(building) => &building.get_name(),
            Self::FusionReactor(productor)
            | Self::GasExtractor(productor)
            | Self::MineralMine(productor) => &productor.get_name(),
            Self::BatteryArray(storage)
            | Self::GasTank(storage)
            | Self::MineralStorage(storage) => &storage.get_name(),
        }
    }

    fn get_level(&self) -> u8 {
        match self {
            Self::CommandCenter(building)
            | Self::OrbitalShipyard(building)
            | Self::ResearchLab(building) => building.get_level(),
            Self::FusionReactor(productor)
            | Self::GasExtractor(productor)
            | Self::MineralMine(productor) => productor.get_level(),
            Self::BatteryArray(storage)
            | Self::GasTank(storage)
            | Self::MineralStorage(storage) => storage.get_level(),
        }
    }

    fn upgrade(&mut self) -> Result<(), BuildingError> {
        match self {
            Self::CommandCenter(building)
            | Self::OrbitalShipyard(building)
            | Self::ResearchLab(building) => building.upgrade(),
            Self::FusionReactor(productor)
            | Self::GasExtractor(productor)
            | Self::MineralMine(productor) => productor.upgrade(),
            Self::BatteryArray(storage)
            | Self::GasTank(storage)
            | Self::MineralStorage(storage) => storage.upgrade(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct BuildingBase {
    name: String,
    level: u8,
    building_config: BuildingConfig,
}

impl BuildingBase {
    pub fn new(name: &str, level: u8, building_config: BuildingConfig) -> Self {
        BuildingBase { name: name.to_string(), level, building_config }
    }
}

impl Building for BuildingBase {
    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_level(&self) -> u8 {
        self.level
    }

    fn upgrade(&mut self) -> Result<(), BuildingError> {
        let max_level = self.building_config.get_max_level();

        if self.level >= max_level {
            return Err(BuildingError::MaxLevelReached {
                current: self.level,
                max: max_level,
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
    pub fn get_resource(&self) -> &Resource {
        &self.resource
    }

    pub fn get_production_rate(&self) -> u32 {
        self.production_rate
    }
}

impl Building for Productor {
    fn get_name(&self) -> &str {
        &self.building.name
    }

    fn get_level(&self) -> u8 {
        self.building.level
    }

    fn upgrade(&mut self) -> Result<(), BuildingError> {
        self.building.upgrade();

        match &self.building.building_config.get_production() {
            Some(production) => {
                match production.get_rate_for_level(self.building.level as usize) {
                    Some(rate) => {
                        self.production_rate = rate as u32;
                    }
                    None => {
                        return Err(BuildingError::WrongBuildingConfiguration);
                    }
                }
                Ok(())
            }
            None => {
                return Err(BuildingError::WrongBuildingConfiguration);
            }
        }
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

impl Building for Storage {
    fn get_name(&self) -> &str {
        &self.building.name
    }

    fn get_level(&self) -> u8 {
        self.building.level
    }

    fn upgrade(&mut self) -> Result<(), BuildingError> {
        self.building.upgrade();

        match &self.building.building_config.get_storage() {
            Some(storage) => {
                match storage.get_capacity_for_level(self.building.level as usize) {
                    Some(capacity) => {
                        self.capacity = capacity as u32;
                    }
                    None => {
                        return Err(BuildingError::WrongBuildingConfiguration);
                    }
                }
                Ok(())
            }
            None => {
                return Err(BuildingError::WrongBuildingConfiguration);
            }
        }
    }
}
