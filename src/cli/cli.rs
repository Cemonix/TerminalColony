use std::io::{self, Write};

use super::command::{CommandRegistry, CommandExecution, CommandError};

pub fn read_and_parse_input(prompt: &str, registry: &CommandRegistry) -> Result<CommandExecution, CommandError> {
    print!("{} > ", prompt);
    io::stdout().flush()
        .map_err(|e| CommandError::new(&format!("Failed to flush stdout: {}", e)))?;

    let mut input = String::new();
    io::stdin().read_line(&mut input)
        .map_err(|e| CommandError::new(&format!("Error reading input: {}", e)))?;

    let trimmed_input = input.trim();

    if trimmed_input.is_empty() {
        return Err(CommandError::new("No command entered."));
    }

    registry.parse(trimmed_input)
}


