use std::{collections::HashMap, fmt};
use std::error::Error;

use super::building::building::Building;
use super::building::{building_config, BuildingConfig, BuildingsConfig, BuildingsConfigError, Storage};
use super::{
    resource, BuildingError, BuildingType, BuildingTypeId, Resource
};

#[derive(Debug)]
pub enum PlanetError {
    BuildingNotBuilt,
    InsufficientResources,
    IncorrectBuildingType,
    BuildingError(BuildingError),
    BuildingsConfigError(BuildingsConfigError),
}

impl fmt::Display for PlanetError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PlanetError::BuildingNotBuilt => write!(f, "Building not built"),
            PlanetError::InsufficientResources => write!(f, "Insufficient resources"),
            PlanetError::IncorrectBuildingType => write!(f, "Incorrect building type"),
            PlanetError::BuildingError(err) => write!(f, "Building error: {}", err),
            PlanetError::BuildingsConfigError(err) => write!(f, "Building config error: {}", err),
        }
    }
}

impl Error for PlanetError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            PlanetError::BuildingNotBuilt => None,
            PlanetError::InsufficientResources => None,
            PlanetError::IncorrectBuildingType => None,
            PlanetError::BuildingError(err) => Some(err),
            PlanetError::BuildingsConfigError(err) => Some(err),
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
    pub fn new(name: &str, buildings_config: &BuildingsConfig) -> Result<Self, PlanetError> {
        let buildings = Self::init_all_buildings_zero(buildings_config)?;

        Ok(
            Self {
                name: name.to_string(),
                buildings
            }
        )
    }

    pub fn get_name(&self) -> &str {
        &self.name 
    }

    fn get_mut_building(&mut self, building_id: BuildingTypeId) -> Result<&mut BuildingType, PlanetError> {
        self.buildings.get_mut(&building_id).ok_or(PlanetError::BuildingNotBuilt)
    }

    fn get_mut_resource_storage(&mut self, resource: Resource) -> Result<&mut Storage, PlanetError> {
        let building_id = match resource {
           Resource::Energy => BuildingTypeId::BatteryArray,
           Resource::Minerals => BuildingTypeId::MineralSilo,
           Resource::Gas => BuildingTypeId::GasTank,
        };
        let building = self.get_mut_building(building_id)?;
        match building {
            BuildingType::BatteryArray(storage)
            | BuildingType::MineralSilo(storage)
            | BuildingType::GasTank(storage) => Ok(storage),
            _ => Err(PlanetError::IncorrectBuildingType), // Should not happen if IDs match types
        }
   }

   pub fn generate_resources(&mut self) -> Result<(), PlanetError> {
        let production = self.get_production_rates();

        for (resource, rate) in production.iter() {
            if *rate > 0 {
                let storage_building = self.get_mut_resource_storage(*resource)?;
                storage_building.add_resource(*rate);
            }
        }
        Ok(())
    }

    fn get_resource_storage_ref(&self, resource: Resource) -> Result<&Storage, PlanetError> {
        let building_id = match resource {
            Resource::Energy => BuildingTypeId::BatteryArray,
            Resource::Minerals => BuildingTypeId::MineralSilo,
            Resource::Gas => BuildingTypeId::GasTank,
        };
        let building = self.get_building(building_id)?;
        match building {
            BuildingType::BatteryArray(storage)
            | BuildingType::MineralSilo(storage)
            | BuildingType::GasTank(storage) => Ok(storage),
            _ => Err(PlanetError::IncorrectBuildingType),
        }
    }

    pub fn build(
        &mut self,
        building_id: BuildingTypeId,
        building_config: &BuildingConfig,
    ) -> Result<(), PlanetError> {
        if let Some(building) = self.buildings.get(&building_id) {
            self.has_enough_resources(Some(building), building_config)?;

            if let Some(existing_building) = self.buildings.get_mut(&building_id) {
                existing_building.upgrade()?;
                return Ok(());
            }
        }
        Err(PlanetError::BuildingNotBuilt)
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

    pub fn get_resource_amount(&self, resource: Resource) -> u32 {
        self.get_resource_storage_ref(resource)
            .map(|storage| storage.get_current_amount()) //
            .unwrap_or(0)
    }

    pub fn get_resource_capacity(&self, resource: Resource) -> u32 {
        self.get_resource_storage_ref(resource)
           .map(|storage| storage.get_capacity()) //
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
            (self.get_resource_amount(Resource::Energy), self.get_resource_capacity(Resource::Energy)),
        );
        storage_map.insert(
            Resource::Minerals,
            (self.get_resource_amount(Resource::Minerals), self.get_resource_capacity(Resource::Minerals)),
        );
        storage_map.insert(
            Resource::Gas,
            (self.get_resource_amount(Resource::Gas), self.get_resource_capacity(Resource::Gas)),
        );


        PlanetStatus {
            planet_name,
            buildings: buildings_list,
            production: production_rates,
            storage: storage_map,
            planet_count: total_planet_count,
        }
    }

    fn init_all_buildings_zero(buildings_config: &BuildingsConfig) -> Result<HashMap<BuildingTypeId, BuildingType>, PlanetError> {
        let mut map = HashMap::new();
        for &building_id in BuildingTypeId::all() {
            if let Some(building_config) = buildings_config.buildings.get(building_id.get_name()) {
                map.insert(building_id, BuildingType::new_zero(building_id, building_config.clone()));
            } else {
                Err(PlanetError::BuildingsConfigError(
                    BuildingsConfigError::BuildingNotFound(format!("Building config for {} not found", building_id.get_name()))
                ))?;
            }
        }
        Ok(map)
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
            Resource::Minerals => self.get_building(BuildingTypeId::MineralSilo),
            Resource::Gas => self.get_building(BuildingTypeId::GasTank)
        }
    }
    
    fn has_enough_resources(
        &self,
        building: Option<&BuildingType>,
        building_config: &BuildingConfig,
    ) -> Result<(), PlanetError> {
        let building_level = building.map_or(1, |b| b.get_level());
        let upgrade_cost = building_config.get_upgrade_cost();

        let energy_cost = upgrade_cost.energy.get(building_level as usize).ok_or(
            PlanetError::BuildingsConfigError(
                BuildingsConfigError::EnergyCostMismatch(
                    format!("Energy cost for level {} not found", building_level)
                )
            )
        )?;
        let minerals_cost = upgrade_cost.minerals.get(building_level as usize).ok_or(
            PlanetError::BuildingsConfigError(
                BuildingsConfigError::MineralsCostMismatch(
                    format!("Minerals cost for level {} not found", building_level)
                )
            )
        )?;
        let gas_cost = upgrade_cost.gas.get(building_level as usize).ok_or(
            PlanetError::BuildingsConfigError(
                BuildingsConfigError::GasCostMismatch(
                    format!("Gas cost for level {} not found", building_level)
                )
            )
        )?;

        if
            self.get_resource_amount(Resource::Energy) >= *energy_cost &&
            self.get_resource_amount(Resource::Minerals) >= *minerals_cost &&
            self.get_resource_amount(Resource::Gas) >= *gas_cost 
        {
            Ok(())
        } else {
            Err(PlanetError::InsufficientResources)
        }
    }
}
