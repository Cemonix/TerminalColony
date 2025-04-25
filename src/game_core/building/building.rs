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

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum BuildingTypeId {
    CommandCenter,
    OrbitalShipyard,
    ResearchLab,
    FusionReactor,
    GasExtractor,
    MineralMine,
    BatteryArray,
    GasTank,
    MineralSilo,
}

impl BuildingTypeId {
    pub fn get_name(&self) -> &str {
        match self {
            Self::CommandCenter => "CommandCenter",
            Self::OrbitalShipyard => "OrbitalShipyard",
            Self::ResearchLab => "ResearchLab",
            Self::FusionReactor => "FusionReactor",
            Self::GasExtractor => "GasExtractor",
            Self::MineralMine => "MineralMine",
            Self::BatteryArray => "BatteryArray",
            Self::GasTank => "GasTank",
            Self::MineralSilo => "MineralSilo",
        }
    }

    pub fn all() -> &'static [BuildingTypeId] {
        use BuildingTypeId::*;
        &[
            CommandCenter,
            OrbitalShipyard,
            ResearchLab,
            FusionReactor,
            GasExtractor,
            MineralMine,
            BatteryArray,
            GasTank,
            MineralSilo,
        ]
    }
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
            Self::MineralSilo => write!(f, "Mineral Silo"),
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
    MineralSilo(Storage),
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
            Self::MineralSilo(_) => BuildingTypeId::MineralSilo,
        }
    }

    pub fn new_zero(id: BuildingTypeId, building_config: BuildingConfig) -> Self {
        match id {
            BuildingTypeId::FusionReactor => 
                Self::FusionReactor(Productor::new("Fusion Reactor", 0, Resource::Energy, building_config)),
            BuildingTypeId::GasExtractor =>
                Self::GasExtractor(Productor::new("Gas Extractor", 0, Resource::Gas, building_config)),
            BuildingTypeId::MineralMine =>
                Self::MineralMine(Productor::new("Mineral Mine", 0, Resource::Minerals, building_config)),
            BuildingTypeId::BatteryArray =>
                Self::BatteryArray(Storage::new("Battery Array", 0, Resource::Energy, building_config)),
            BuildingTypeId::GasTank =>
                Self::GasTank(Storage::new("Gas Tank", 0, Resource::Gas, building_config)),
            BuildingTypeId::MineralSilo =>
                Self::MineralSilo(Storage::new("Mineral Storage", 0, Resource::Minerals, building_config)),
            BuildingTypeId::CommandCenter =>
                Self::CommandCenter(BuildingBase::new("Command Center", 0, building_config)),
            BuildingTypeId::OrbitalShipyard =>
                Self::OrbitalShipyard(BuildingBase::new("Orbital Shipyard", 0, building_config)),
            BuildingTypeId::ResearchLab =>
                Self::ResearchLab(BuildingBase::new("Research Lab", 0, building_config)),
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
            | Self::MineralSilo(storage) => &storage.get_name(),
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
            | Self::MineralSilo(storage) => storage.get_level(),
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
            | Self::MineralSilo(storage) => storage.upgrade(),
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
    pub fn new(name: &str, level: u8, resource: Resource, building_config: BuildingConfig) -> Self {
        let production_rate = match &building_config.get_production() {
            Some(production) => {
                match production.get_rate_for_level(level as usize) {
                    Some(rate) => rate as u32,
                    None => 0,
                }
            }
            None => 0,
        };

        Productor {
            building: BuildingBase::new(name, level, building_config),
            resource: resource,
            production_rate: production_rate
        }
    }

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
        self.building.upgrade()?;

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
    pub fn new(name: &str, level: u8, resource: Resource, building_config: BuildingConfig) -> Self {
        let capacity = match &building_config.get_storage() {
            Some(storage) => {
                match storage.get_capacity_for_level(level as usize) {
                    Some(capacity) => capacity as u32,
                    None => 0,
                }
            }
            None => 0,
        };

        Storage {
            building: BuildingBase::new(name, level, building_config),
            resource: resource,
            capacity: capacity,
            current_amount: 0,
        }
    }

    pub fn get_capacity(&self) -> u32 {
        self.capacity
    }

    pub fn get_current_amount(&self) -> u32 {
        self.current_amount
    }

    pub fn add_resource(&mut self, amount_to_add: u32) -> u32 {
        let available_space = self.capacity.saturating_sub(self.current_amount);
        let actual_added = std::cmp::min(amount_to_add, available_space);
        self.current_amount += actual_added;
        actual_added
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
        self.building.upgrade()?;

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
