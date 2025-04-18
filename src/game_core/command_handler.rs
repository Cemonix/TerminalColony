use crate::cli::{command::ParsedCommand, CommandExecution, CommandRegistry};

pub struct CommandHandler {}

impl CommandHandler {
    pub fn handle_command(&self, command_registry: &CommandRegistry, command: CommandExecution) {
        match command {
            CommandExecution::Help(command_execution) => {
                self.handle_help(command_registry, command_execution);
            }
            CommandExecution::Status(_) => {
                self.handle_status();
            }
            CommandExecution::Build(_) => {
                self.handle_build();
            }
            CommandExecution::EndTurn(_) => {
                self.handle_end_turn();
            }
            CommandExecution::Quit(_) => {
                self.handle_quit();
            }
            CommandExecution::UnknownInternal(_) => {
                self.handle_unknown_internal();
            }
        }
    }

    fn handle_help(&self, command_registry: &CommandRegistry, command_execution: ParsedCommand) {
        // Help without specific command - show all available commands
        if command_execution.args.len() == 0 {
            for command in command_registry.get_all_command_definitions() {
                command.print();
            }
        } else {
            // Help with specific command - show details of that command
            let command_name = &command_execution.args[0];
            if let Some(command_definitions) = command_registry.get_command_definitions(command_name) {
                for command in command_definitions {
                    command.print();
                }
            } else {
                eprintln!("Error: Command '{}' not found.", command_name);
            }
        }
        
    }

    fn handle_status(&self) {
        println!("Status command executed.");
    }

    fn handle_quit(&self) {
        println!("Quiting game.");
        std::process::exit(0);
    }

    fn handle_build(&self) {
        println!("Build command executed.");
    }

    fn handle_end_turn(&self) {
        println!("End turn command executed.");
    }

    fn handle_unknown_internal(&self) {
        println!("Unknown internal command executed.");
    }
    
}