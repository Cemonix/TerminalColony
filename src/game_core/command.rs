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


#[derive(Debug)]
pub struct CommandError {
    pub message: String,
}

impl CommandError {
    pub fn new(message: &str) -> Self {
        CommandError {
            message: message.to_string(),
        }
    }
}

impl fmt::Display for CommandError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl Error for CommandError {}

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

impl CommandDefinition {
    pub fn print(&self) {
        // TODO: Implement colorized output for command name and description in a table format
        let args_str = self.arg_hints
            .iter()
            .map(|arg| format!("<{}>", arg))
            .collect::<Vec<String>>()
            .join(" ");

        println!("Command: {} {}", self.name, args_str);
        println!("{}", self.description);
        println!()
    }
}

#[derive(Deserialize, Debug)]
struct CommandsConfig {
    commands: Vec<CommandDefinition>,
}

#[derive(Debug, Clone)]
pub struct ParsedCommand {
    pub name: String,
    pub definition: CommandDefinition,
    pub args: Vec<String>,
}

#[derive(Debug)]
pub enum CommandExecution {
    Help(ParsedCommand),
    Status(ParsedCommand),
    Build(ParsedCommand),
    EndTurn(ParsedCommand),
    Quit(ParsedCommand),
    UnknownInternal(ParsedCommand),
}

impl From<ParsedCommand> for CommandExecution {
    fn from(parsed_command: ParsedCommand) -> Self {
        match parsed_command.definition.name.as_str() {
            "help" => CommandExecution::Help(parsed_command),
            "status" => CommandExecution::Status(parsed_command),
            "build" => CommandExecution::Build(parsed_command),
            "endturn" => CommandExecution::EndTurn(parsed_command),
            "quit" => CommandExecution::Quit(parsed_command),
            _ => {
                eprintln!(
                    "Error: Command '{}' has no associated execution logic. Wrong configuration!",
                    parsed_command.definition.name
                );
                CommandExecution::UnknownInternal(parsed_command)
            }
        }
    }
}

#[derive(Debug)]
pub struct CommandRegistry {
    definitions: HashMap<String, Vec<CommandDefinition>>,
}

impl CommandRegistry {
    pub fn load(config_path: &str) -> Result<Self, CommandLoadError> {
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

    pub fn parse(&self, input: &str) -> Result<CommandExecution, CommandError> {
        let parts: Vec<&str> = input.trim().split_whitespace().collect();
        if parts.is_empty() {
            return Err(CommandError::new("No command provided. Type 'help' for options."));
        }

        let command_name = parts[0].to_lowercase();
        let provided_args: Vec<String> = parts[1..].iter().map(|&s| s.to_string()).collect();
        let provided_arg_count = provided_args.len();

        match self.definitions.get(&command_name) {
            Some(possible_defs) => {
                match possible_defs.iter().find(|def| def.expected_args == provided_arg_count) {
                    Some(matching_def) => {
                        let parsed_cmd = ParsedCommand {
                            name: command_name.clone(),
                            definition: matching_def.clone(),
                            args: provided_args,
                        };

                        Ok(parsed_cmd.into())
                    }
                    None => {
                        let expected_counts: Vec<String> = possible_defs.iter()
                            .map(|d| d.expected_args.to_string())
                            .collect();
                        Err(CommandError::new(&format!(
                            "Error: Wrong number of arguments for command '{}'. Got {}, expected {}.",
                            command_name, provided_arg_count, expected_counts.join(" or ")
                        )))
                    }
                }
            }
            None => Err(CommandError::new(&format!(
                "Unknown command: '{}'. Type 'help' for available commands.",
                command_name
            ))),
        }
    }

    pub fn get_all_command_definitions(&self) -> Vec<CommandDefinition> {
        self.definitions.values().flat_map(|defs| defs.clone()).collect()
    }

    pub fn get_command_definitions(&self, command_name: &str) -> Option<&Vec<CommandDefinition>> {
        self.definitions.get(command_name)
    }
}
