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
    pub fn new(name: String, building_config: BuildingConfig) -> Self {
        let command_center = BuildingType::CommandCenter(
            BuildingBase::new(
                building_config.name,
                1,
                building_config.max_level,
            )
        );
    
        let mut buildings = HashMap::new();
        buildings.insert(BuildingTypeId::CommandCenter, command_center);
    
        Player {
            name,
            planets: vec![Planet::new("Earth", Some(buildings))],
        }
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn set_name(&mut self, name: String) {
        self.name = name;
    }
}