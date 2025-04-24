use std::{collections::HashMap, fs, path::Path};

use serde::Deserialize;

use crate::game_core::Resource;

#[derive(Debug)]
pub enum BuildingsConfigError {
    Io(std::io::Error),
    Toml(toml::de::Error),
    EnergyCostMismatch(String),
    MineralsCostMismatch(String),
    GasCostMismatch(String),
    ProductionRateMismatch(String),
    StorageCapacityMismatch(String),
    BuildingTimeMismatch(String),
}

impl std::fmt::Display for BuildingsConfigError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BuildingsConfigError::Io(err) => write!(
                f, "Failed to read buildings configuration file: {}", err
            ),
            BuildingsConfigError::Toml(err) => write!(
                f, "Failed to parse buildings configuration file (TOML): {}", err
            ),
            BuildingsConfigError::EnergyCostMismatch(err) => write!(
                f, "Energy cost mismatch: {} doesn't match max_level", err
            ),
            BuildingsConfigError::MineralsCostMismatch(err) => write!(
                f, "Minerals cost mismatch: {} doesn't match max_level", err
            ),
            BuildingsConfigError::GasCostMismatch(err) => write!(
                f, "Gas cost mismatch: {} doesn't match max_level", err
            ),
            BuildingsConfigError::ProductionRateMismatch(err) => write!(
                f, "Production rate mismatch: {} doesn't match max_level", err
            ),
            BuildingsConfigError::StorageCapacityMismatch(err) => write!(
                f, "Storage capacity mismatch: {} doesn't match max_level", err
            ),
            BuildingsConfigError::BuildingTimeMismatch(err) => write!(
                f, "Building time mismatch: {} doesn't match max_level", err
            ),
        }
    }
}

impl std::error::Error for BuildingsConfigError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            BuildingsConfigError::Io(err) => Some(err),
            BuildingsConfigError::Toml(err) => Some(err),
            BuildingsConfigError::EnergyCostMismatch(_) => None,
            BuildingsConfigError::MineralsCostMismatch(_) => None,
            BuildingsConfigError::GasCostMismatch(_) => None,
            BuildingsConfigError::ProductionRateMismatch(_) => None,
            BuildingsConfigError::StorageCapacityMismatch(_) => None,
            BuildingsConfigError::BuildingTimeMismatch(_) => None,
        }
    }
}

impl From<std::io::Error> for BuildingsConfigError {
    fn from(err: std::io::Error) -> Self {
        BuildingsConfigError::Io(err)
    }
}

impl From<toml::de::Error> for BuildingsConfigError {
    fn from(err: toml::de::Error) -> Self {
        BuildingsConfigError::Toml(err)
    }
}

// =================================================================================================

#[derive(Deserialize, Debug)]
pub struct BuildingsConfig {
    #[serde(flatten)]
    pub buildings: HashMap<String, BuildingConfig>,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct BuildingConfig {
    name: String,
    max_level: u8,
    upgrade_cost: UpgradeCost,
    building_time: BuildingTime,
    #[serde(default)]
    production: Option<ProductionInfo>,
    #[serde(default)]
    storage: Option<StorageInfo>,
}

impl BuildingConfig {
    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn get_max_level(&self) -> u8 {
        self.max_level
    }

    pub fn get_upgrade_cost(&self) -> &UpgradeCost {
        &self.upgrade_cost
    }

    pub fn get_production(&self) -> Option<&ProductionInfo> {
        self.production.as_ref()
    }

    pub fn get_storage(&self) -> Option<&StorageInfo> {
        self.storage.as_ref()
    }
}

#[derive(Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct UpgradeCost {
    #[serde(default)]
    pub energy: Vec<u32>,
    #[serde(default)]
    pub minerals: Vec<u32>,
    #[serde(default)]
    pub gas: Vec<u32>,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct BuildingTime {
    pub time: Vec<u32>,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct ProductionInfo {
    pub resource: Resource,
    pub rate_per_level: Vec<u32>,
}

impl ProductionInfo {
    pub fn get_resource(&self) -> &Resource {
        &self.resource
    }

    pub fn get_rate_for_level(&self, level: usize) -> Option<u32> {
        self.rate_per_level.get(level).cloned()
    }
}

#[derive(Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct StorageInfo {
    pub resource: Resource,
    pub capacity_per_level: Vec<u32>,
}

impl StorageInfo {
    pub fn get_resource(&self) -> &Resource {
        &self.resource
    }

    pub fn get_capacity_for_level(&self, level: usize) -> Option<u32> {
        self.capacity_per_level.get(level).cloned()
    }
}

impl BuildingsConfig {
    pub fn load(path: &Path) -> Result<BuildingsConfig, BuildingsConfigError> {
        let config_content = fs::read_to_string(path)?;
        let buildings_config: BuildingsConfig = toml::from_str(&config_content)?;
    
        for (_, config) in &buildings_config.buildings {
            let max_lvl = config.max_level as usize;
    
            // Validate upgrade costs
            if config.upgrade_cost.energy.len() != max_lvl {
                return Err(
                    BuildingsConfigError::EnergyCostMismatch(
                        config.upgrade_cost.energy.len().to_string()
                    )
                );
            }
            if config.upgrade_cost.minerals.len() != max_lvl {
                return Err(
                    BuildingsConfigError::MineralsCostMismatch(
                        config.upgrade_cost.minerals.len().to_string()
                    )
                );
            }
            if config.upgrade_cost.gas.len() != 0 && config.upgrade_cost.gas.len() != max_lvl {
                return Err(
                    BuildingsConfigError::GasCostMismatch(
                        config.upgrade_cost.gas.len().to_string()
                    )
                );
            }
    
            if let Some(prod) = &config.production {
                // Validate production info
                if prod.rate_per_level.len() != max_lvl {
                    return Err(
                        BuildingsConfigError::ProductionRateMismatch(
                            prod.rate_per_level.len().to_string()
                        )
                    );
                }
            }
    
            // Validate storage info
            if let Some(stor) = &config.storage {
                if stor.capacity_per_level.len() != max_lvl {
                    return Err(
                        BuildingsConfigError::StorageCapacityMismatch(
                            stor.capacity_per_level.len().to_string()
                        )
                    );
                }
            }

            // Validate building time
            if config.building_time.time.len() != max_lvl {
                return Err(
                    BuildingsConfigError::BuildingTimeMismatch(
                        config.building_time.time.len().to_string()
                    )
                );
            }
        }
    
        Ok(buildings_config)
    }
}
