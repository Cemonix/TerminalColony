use std::collections::HashMap;

use super::building::{Building, BuildingType};

pub struct Planet {
    name: String,
    buildings: Vec<Building>,
    building_slots: HashMap<BuildingType, u8>,
    size: u32,
    category: String,
}

impl Planet {
    // pub fn new(name: String, buildings: Vec<Building>, size: u32, category: String) -> Self {
    //     let mut building_slots = HashMap::new();
    //     for building in &buildings {
    //         *building_slots.entry(building.building_type).or_insert(0) += 1;
    //     }
    //     Planet {
    //         name,
    //         buildings,
    //         size,
    //         category,
    //     }
    // }
}


// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn test_planet_creation() {
//         let planet = Planet::new(
//             "Earth".to_string(), 
//             vec![
//                 Building::new(BuildingType::CommandCenter, 1, 5),
//                 Building::new(BuildingType::FusionReactor, 1, 5),
//             ], 
//             100, 
//             "Terrestrial".to_string()
//         );

//         assert_eq!(planet.name, "Earth");
//         assert_eq!(planet.buildings.len(), 2);
//         assert_eq!(planet.size, 100);
//         assert_eq!(planet.category, "Terrestrial");
//     }
// }