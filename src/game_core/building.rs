use std::fmt;
use std::error::Error;

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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BuildingType {
    CommandCenter,
    FusionReactor,
    GasExtractor,
    MineralMine,
    GasTank,
    MineralStorage,
    OrbitalShipyard,
    ResearchLab,
}

pub struct Building {
    building_type: BuildingType,
    level: u8,
    max_level: u8,
}

pub trait BuildingBehavior {
    fn get_type(&self) -> BuildingType;
    fn get_level(&self) -> u8;
    fn upgrade(&mut self) -> Result<(), BuildingError>;
}

pub trait ResourceGenerator {
    fn generate_resources(&self) -> u32;
}

pub trait StorageProvider {
    fn get_storage_capacity(&self) -> u32;
}

impl Building {
    pub fn new(building_type: BuildingType, level: u8, max_level: u8) -> Self {
        Building {
            building_type,
            level,
            max_level,
        }
    }
}

impl BuildingBehavior for Building {
    fn get_type(&self) -> BuildingType {
        self.building_type
    }

    fn get_level(&self) -> u8 {
        self.level
    }

    fn upgrade(&mut self) -> Result<(), BuildingError> {
        if self.level < self.max_level {
            self.level += 1;
            Ok(())
        } else {
            Err(BuildingError::MaxLevelReached {
                current: self.level,
                max: self.max_level,
            })
        }
    }
}

impl ResourceGenerator for Building {
    fn generate_resources(&self) -> u32 {
        // TODO: Create config file for resource generation rates
        // For now, we will use a simple formula based on building type and level
        match self.building_type {
            BuildingType::MineralMine => self.level as u32 * 10,
            BuildingType::GasExtractor => self.level as u32 * 8,
            BuildingType::FusionReactor => self.level as u32 * 15,
            _ => 0,
        }
    }
}

impl StorageProvider for Building {
    fn get_storage_capacity(&self) -> u32 {
        // TODO: Create config file for storage capacities
        match self.building_type {
            BuildingType::MineralStorage => self.level as u32 * 1000,
            BuildingType::GasTank => self.level as u32 * 800,
            _ => 0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_building_creation() {
        let building = Building::new(BuildingType::CommandCenter, 1, 5);
        assert_eq!(building.get_type(), BuildingType::CommandCenter);
        assert_eq!(building.get_level(), 1);
    }

    #[test]
    fn test_building_upgrade() {
        let mut building = Building::new(BuildingType::CommandCenter, 1, 5);
        assert_eq!(building.upgrade(), Ok(()));
        assert_eq!(building.get_level(), 2);
    }

    #[test]
    fn test_building_upgrade_max_level() {
        let mut building = Building::new(BuildingType::CommandCenter, 5, 5);
        assert_eq!(
            building.upgrade(),
            Err(BuildingError::MaxLevelReached { current: 5, max: 5 })
        );
    }
}