use std::{collections::HashMap, fmt};
use std::error::Error;

use super::building::building::Building;
use super::{
    BuildingError, BuildingType, BuildingTypeId, Resource
};

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

#[derive(Debug, Clone, Default)]
pub struct PlanetStatus {
    pub planet_name: String,
    pub buildings: Vec<(String, u8)>,
    pub production: HashMap<Resource, u32>,
    pub storage: HashMap<Resource, (u32, u32)>,
    pub planet_count: usize,
}

#[derive(Debug, Clone)]
pub struct Planet {
    name: String,
    buildings: HashMap<BuildingTypeId, BuildingType>,
}

impl Planet {
    pub fn new(name: &str, buildings: Option<HashMap<BuildingTypeId, BuildingType>>) -> Self {
        Self {
            name: name.to_string(),
            buildings: buildings.unwrap_or_else(HashMap::new),
        }
    }

    pub fn get_name(&self) -> &str {
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

    pub fn get_production_rates(&self) -> HashMap<Resource, u32> {
        let mut rates = HashMap::new();
        rates.insert(Resource::Energy, 0);
        rates.insert(Resource::Minerals, 0);
        rates.insert(Resource::Gas, 0);

        for building in self.buildings.values() {
            match building {
                BuildingType::FusionReactor(productor)
                | BuildingType::GasExtractor(productor)
                | BuildingType::MineralMine(productor) => {
                    let resource = productor.get_resource();
                    *rates.entry(*resource).or_insert(0) += productor.get_production_rate();
                }
                _ => {}
            }
        }
        rates
    }

    pub fn get_energy_amount(&self) -> u32 {
        self.get_resource_storage(Resource::Energy)
            .map(|building| match building {
                BuildingType::BatteryArray(storage) => storage.get_current_amount(),
                _ => 0,
            })
            .unwrap_or(0)
    }

    pub fn get_minerals_amount(&self) -> u32 {
        self.get_resource_storage(Resource::Minerals)
            .map(|building| match building {
                BuildingType::MineralStorage(storage) => storage.get_current_amount(),
                _ => 0,
            })
            .unwrap_or(0)
    }

    pub fn get_gas_amount(&self) -> u32 {
        self.get_resource_storage(Resource::Gas)
            .map(|building| match building {
                BuildingType::GasTank(storage) => storage.get_current_amount(),
                _ => 0,
            })
            .unwrap_or(0)
    }

    pub fn get_energy_capacity(&self) -> u32 {
        self.get_resource_storage(Resource::Energy)
            .map(|building| match building {
                BuildingType::BatteryArray(storage) => storage.get_capacity(),
                _ => 0,
            })
            .unwrap_or(0)
    }

    pub fn get_minerals_capacity(&self) -> u32 {
        self.get_resource_storage(Resource::Minerals)
            .map(|building| match building {
                BuildingType::MineralStorage(storage) => storage.get_capacity(),
                _ => 0,
            })
            .unwrap_or(0)
    }

    pub fn get_gas_capacity(&self) -> u32 {
        self.get_resource_storage(Resource::Gas)
            .map(|building| match building {
                BuildingType::GasTank(storage) => storage.get_capacity(),
                _ => 0,
            })
            .unwrap_or(0)
    }

    pub fn get_status(&self, total_planet_count: usize) -> PlanetStatus {
        let planet_name = self.get_name().to_string();

        let buildings_list: Vec<(String, u8)> = self
            .buildings
            .values()
            .map(|building| (building.get_name().to_string(), building.get_level())) 
            .collect();

        let production_rates = self.get_production_rates();

        let mut storage_map = HashMap::new();
        storage_map.insert(
            Resource::Energy,
            (self.get_energy_amount(), self.get_energy_capacity()),
        );
        storage_map.insert(
            Resource::Minerals,
            (self.get_minerals_amount(), self.get_minerals_capacity()),
        );
        storage_map.insert(
            Resource::Gas,
            (self.get_gas_amount(), self.get_gas_capacity()),
        );


        PlanetStatus {
            planet_name,
            buildings: buildings_list,
            production: production_rates,
            storage: storage_map,
            planet_count: total_planet_count,
        }
    }

    fn get_building(
        &self,
        building_id: BuildingTypeId,
    ) -> Result<&BuildingType, PlanetError> {
        self.buildings
            .get(&building_id)
            .ok_or(PlanetError::BuildingNotBuilt)
    }

    fn get_resource_storage(
        &self,
        resource: Resource,
    ) -> Result<&BuildingType, PlanetError> {
        match resource {
            Resource::Energy => self.get_building(BuildingTypeId::BatteryArray),
            Resource::Minerals => self.get_building(BuildingTypeId::MineralStorage),
            Resource::Gas => self.get_building(BuildingTypeId::GasTank)
        }
    }
    
    fn has_enough_resources(
        &self,
        energy: u32,
        minerals: u32,
        gas: u32,
    ) -> Result<(), PlanetError> {
        let energy_amount = self.get_energy_amount();
        let minerals_amount = self.get_minerals_amount();
        let gas_amount = self.get_gas_amount();

        if energy_amount >= energy && minerals_amount >= minerals && gas_amount >= gas {
            Ok(())
        } else {
            Err(PlanetError::InsufficientResources)
        }
    }
}
