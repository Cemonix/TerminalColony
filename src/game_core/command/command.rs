use std::fmt;
use std::error::Error;

use super::{CommandDefinition, CommandRegistry};

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

#[derive(Debug, Clone)]
struct ParsedCommand {
    name: String,
    definition: CommandDefinition,
    args: Vec<String>,
}

#[derive(Debug)]
pub enum CommandExecution {
    Help(ParsedCommand),
    Build(BuildCommand),
    EndTurn(EndTurnCommand),
    Quit(QuitCommand),
    UnknownInternal(ParsedCommand),
}

impl CommandExecution {
    pub fn parse(command_registry: &CommandRegistry, input: &str) -> Result<CommandExecution, CommandError> {
        let parts: Vec<&str> = input.trim().split_whitespace().collect();
        if parts.is_empty() {
            return Err(CommandError::new("No command provided. Type 'help' for options."));
        }

        let command_name = parts[0].to_lowercase();
        let provided_args: Vec<String> = parts[1..].iter().map(|&s| s.to_string()).collect();
        let provided_arg_count = provided_args.len();

        match command_registry.get_command_definitions(&command_name) {
            Some(possible_defs) => {
                match possible_defs.iter().find(|def| def.expected_args == provided_arg_count) {
                    Some(matching_def) => {
                        let parsed_cmd = ParsedCommand {
                            name: command_name.clone(),
                            definition: matching_def.clone(),
                            args: provided_args,
                        };

                        match command_name.as_str() {
                            "help" => Ok(CommandExecution::Help(parsed_cmd)),
                            "build" => {
                                let build_cmd = BuildCommand::try_from(parsed_cmd)?;
                                Ok(CommandExecution::Build(build_cmd))
                            }
                            "endturn" => {
                                let end_turn_cmd = EndTurnCommand::try_from(parsed_cmd)?;
                                Ok(CommandExecution::EndTurn(end_turn_cmd))
                            }
                            "quit" => {
                                let quit_cmd = QuitCommand::try_from(parsed_cmd)?;
                                Ok(CommandExecution::Quit(quit_cmd))
                            }
                            _ => Ok(CommandExecution::UnknownInternal(parsed_cmd)),
                        }
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
}

#[derive(Debug)]
pub struct EndTurnCommand {
    name: String,
}

impl EndTurnCommand {
    pub fn new(name: &str) -> Self {
        EndTurnCommand {
            name: name.to_string(),
        }
    }
}

impl TryFrom<ParsedCommand> for EndTurnCommand {
    type Error = CommandError;

    fn try_from(parsed_command: ParsedCommand) -> Result<Self, Self::Error> {
        if parsed_command.args.is_empty() {
            Ok(EndTurnCommand::new(&parsed_command.name))
        } else {
            Err(CommandError::new("End turn command does not accept any arguments."))
        }
    }
}

#[derive(Debug)]
pub struct QuitCommand {
    name: String,
}

impl QuitCommand {
    pub fn new(name: &str) -> Self {
        QuitCommand {
            name: name.to_string(),
        }
    }
}

impl TryFrom<ParsedCommand> for QuitCommand {
    type Error = CommandError;

    fn try_from(parsed_command: ParsedCommand) -> Result<Self, Self::Error> {
        if parsed_command.args.is_empty() {
            Ok(QuitCommand::new(&parsed_command.name))
        } else {
            Err(CommandError::new("Quit command does not accept any arguments."))
        }
    }
}

#[derive(Debug)]
pub struct BuildCommand {
    name: String,
    building: String,
    planet: String,
}

impl BuildCommand {
    pub fn new(name: &str, building: &str, planet: &str) -> Self {
        BuildCommand {
            name: name.to_string(),
            building: building.to_string(),
            planet: planet.to_string(),
        }
    }

    pub fn get_building(&self) -> &str {
        &self.building
    }

    pub fn get_planet(&self) -> &str {
        &self.planet
    }
}

impl TryFrom<ParsedCommand> for BuildCommand {
    type Error = CommandError;

    fn try_from(parsed_command: ParsedCommand) -> Result<Self, Self::Error> {
        let args = parsed_command.args;
        if args.len() < 2 {
            return Err(CommandError::new("Not enough arguments for build command."));
        }

        if let Some(building) = args.get(0) {
            if let Some(planet) = args.get(1) {
                Ok(BuildCommand::new(&parsed_command.name, building, planet))
            } else {
                Err(CommandError::new("Planet argument is missing."))
            }
        } else {
            Err(CommandError::new("Building argument is missing."))
        }
    }
}