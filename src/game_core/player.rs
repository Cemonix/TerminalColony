use std::collections::HashMap;

use super::{
    BuildingTypeId,
    BuildingType,
    BuildingBase,
    BuildingConfig,
    Planet,
};

pub struct Player {
    name: String,
    planets: Vec<Planet>,
}

impl Player {
    pub fn new(name: &str, planet_name: &str, building_config: BuildingConfig) -> Self {
        let command_center = BuildingType::CommandCenter(
            BuildingBase::new(
                building_config.clone().get_name(),
                1,
                building_config,
            )
        );
    
        let mut buildings = HashMap::new();
        buildings.insert(BuildingTypeId::CommandCenter, command_center);
    
        Player {
            name: name.to_string(),
            planets: vec![Planet::new(planet_name, Some(buildings))],
        }
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn get_planets(&self) -> &[Planet] {
        &self.planets
    }
}