pub mod building;
pub mod building_config;

pub use building::{
    BuildingTypeId, 
    BuildingType, 
    BuildingBase, 
    Productor, 
    Storage, 
    BuildingError
};
pub use building_config::{
    BuildingsConfig,
    BuildingConfig,
    BuildingsConfigError
};