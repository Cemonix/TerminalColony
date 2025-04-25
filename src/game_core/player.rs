use std::collections::HashMap;

use super::{
    building::BuildingsConfig, planet::PlanetError, BuildingBase, BuildingConfig, BuildingType, BuildingTypeId, Planet
};

pub struct Player {
    name: String,
    planets: HashMap<String, Planet>,
}

impl Player {
    pub fn new(name: &str, planet_name: &str, buildings_config: &BuildingsConfig) -> Self {
        let planet = Planet::new(planet_name, buildings_config).unwrap(); // TODO: Handle error

        Player {
            name: name.to_string(),
            planets: HashMap::from([
                (
                    planet_name.to_string(),
                    planet,
                ),
            ]),
        }
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn get_planets_count(&self) -> usize {
        self.planets.len()
    }

    pub fn get_planet_names(&self) -> Vec<String> {
        self.planets.keys().cloned().collect()
    }

    pub fn process_turn_end(&mut self) -> Result<(), PlanetError> {
        for planet in self.planets.values_mut() {
            planet.generate_resources()?;
        }
        Ok(())
    }
    
    pub fn get_planet(&self, planet_name: &str) -> Option<&Planet> {
        self.planets.get(planet_name)
    }

    pub fn get_mut_planet(&mut self, planet_name: &str) -> Option<&mut Planet> {
        self.planets.get_mut(planet_name)
    }
}