use std::{collections::HashMap, fmt};
use std::error::Error;

use super::building::building::Building;
use super::building::{BuildingError, BuildingType, BuildingTypeId};
use super::resource::Resource;

#[derive(Debug, PartialEq, Eq)]
pub enum PlanetError {
    BuildingNotBuilt,
    InsufficientResources,
    BuildingError(BuildingError),
}

impl fmt::Display for PlanetError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PlanetError::BuildingNotBuilt => write!(f, "Building not built"),
            PlanetError::InsufficientResources => write!(f, "Insufficient resources"),
            PlanetError::BuildingError(err) => write!(f, "Building error: {}", err),
        }
    }
}

impl Error for PlanetError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            PlanetError::BuildingNotBuilt => None,
            PlanetError::InsufficientResources => None,
            PlanetError::BuildingError(err) => Some(err),
        }
    }
}

impl From<BuildingError> for PlanetError {
    fn from(err: BuildingError) -> Self {
        PlanetError::BuildingError(err)
    }
}

// =================================================================================================

pub struct Planet {
    name: String,
    buildings: HashMap<BuildingTypeId, BuildingType>,
}

impl Planet {
    pub fn new(name: String, buildings: Option<HashMap<BuildingTypeId, BuildingType>>) -> Self {
        Self {
            name,
            buildings: buildings.unwrap_or_else(HashMap::new),
        }
    }

    pub fn get_name(&self) -> &String {
        &self.name
    }

    pub fn build(
        &mut self,
        building: BuildingType,
        energy: u32,
        minerals: u32,
        gas: u32,
    ) -> Result<(), PlanetError> {
        self.has_enough_resources(energy, minerals, gas)?;

        if let Some(existing_building) = self.buildings.get_mut(&building.get_id()) {
            existing_building.upgrade()?;
        } else {
            self.buildings.insert(building.get_id(), building);
        }

        Ok(())
    }

    fn get_resource_storage(
        &self,
        resource: Resource,
    ) -> Result<BuildingType, PlanetError> {
        match resource {
            Resource::Energy => {
                if let Some(building) = self.buildings.get(&BuildingTypeId::BatteryArray) {
                    return Ok(building.clone());
                }
                Err(PlanetError::BuildingNotBuilt)
            }
            Resource::Minerals => {
                if let Some(building) = self.buildings.get(&BuildingTypeId::MineralStorage) {
                    return Ok(building.clone());
                }
                Err(PlanetError::BuildingNotBuilt)
            }
            Resource::Gas => {
                if let Some(building) = self.buildings.get(&BuildingTypeId::GasTank) {
                    return Ok(building.clone());
                }
                Err(PlanetError::BuildingNotBuilt)
            }
        }
    }
    
    fn has_enough_resources(
        &self,
        energy: u32,
        minerals: u32,
        gas: u32,
    ) -> Result<(), PlanetError> {
        match self.get_resource_storage(Resource::Energy)? {
            BuildingType::BatteryArray(ref storage) if energy <= storage.get_current_amount() => {}
            BuildingType::BatteryArray(_) => return Err(PlanetError::InsufficientResources),
            _ => return Err(PlanetError::BuildingNotBuilt),
        }

        match self.get_resource_storage(Resource::Minerals)? {
            BuildingType::MineralStorage(ref storage) if minerals <= storage.get_current_amount() => {}
            BuildingType::MineralStorage(_) => return Err(PlanetError::InsufficientResources),
            _ => return Err(PlanetError::BuildingNotBuilt),
        }

        match self.get_resource_storage(Resource::Gas)? {
            BuildingType::GasTank(ref storage) if gas <= storage.get_current_amount() => {}
            BuildingType::GasTank(_) => return Err(PlanetError::InsufficientResources),
            _ => return Err(PlanetError::BuildingNotBuilt),
        }

        Ok(())
    }
}
