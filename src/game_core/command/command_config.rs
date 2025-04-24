use std::path::Path;
use std::{fmt, fs, io};
use std::error::Error;
use std::collections::HashMap;
use serde::Deserialize;

#[derive(Debug)]
pub enum CommandLoadError {
    Io(io::Error),
    Toml(toml::de::Error),
}

impl fmt::Display for CommandLoadError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CommandLoadError::Io(e) => write!(f, "Failed to read command configuration file: {}", e),
            CommandLoadError::Toml(e) => write!(f, "Failed to parse command configuration file (TOML): {}", e),
        }
    }
}

impl Error for CommandLoadError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            CommandLoadError::Io(e) => Some(e),
            CommandLoadError::Toml(e) => Some(e),
        }
    }
}

impl From<io::Error> for CommandLoadError {
    fn from(err: io::Error) -> Self {
        CommandLoadError::Io(err)
    }
}

impl From<toml::de::Error> for CommandLoadError {
    fn from(err: toml::de::Error) -> Self {
        CommandLoadError::Toml(err)
    }
}

// =================================================================================================

#[derive(Deserialize, Debug, Clone)]
pub struct CommandDefinition {
    pub name: String,
    #[serde(default)]
    pub aliases: Vec<String>,
    pub description: String,
    pub expected_args: usize,
    #[serde(default)]
    pub arg_hints: Vec<String>,
}

#[derive(Deserialize, Debug)]
struct CommandsConfig {
    commands: Vec<CommandDefinition>,
}

#[derive(Debug)]
pub struct CommandRegistry {
    definitions: HashMap<String, Vec<CommandDefinition>>,
}

impl CommandRegistry {
    pub fn load(config_path: &Path) -> Result<Self, CommandLoadError> {
        let toml_content = fs::read_to_string(config_path)?;
        let config: CommandsConfig = toml::from_str(&toml_content)?;

        let mut definitions: HashMap<String, Vec<CommandDefinition>> = HashMap::new();
        for cmd_def in config.commands {
            // --- Handle command name ---
            definitions
                .entry(cmd_def.name.clone())
                .or_insert_with(Vec::new)
                .push(cmd_def.clone());
    
            // --- Handle aliases ---
            for alias in cmd_def.aliases.iter() {
                definitions
                    .entry(alias.clone())
                    .or_insert_with(Vec::new)
                    .push(cmd_def.clone());
            }
        }

        Ok(CommandRegistry { definitions })
    }

    pub fn get_command_definitions(&self, command_name: &str) -> Option<&Vec<CommandDefinition>> {
        self.definitions.get(command_name)
    }
}