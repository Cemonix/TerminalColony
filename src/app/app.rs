use crate::cli::{read_and_parse_input, CommandLoadError, CommandRegistry};
use crate::game_core::GameCore;

pub struct App {
    game_core: GameCore,
    exit: bool,
}

impl App {
    pub fn new() -> Self {
        App {
            game_core: GameCore::new(),
            exit: false,
        }
    }
    
    pub fn run(&mut self) -> Result<(), CommandLoadError> {
        let mut terminal = ratatui::init();
        
        let command_registry = CommandRegistry::load("data/commands.toml")?;
        println!("Commands loaded.");

        while !self.exit {
            terminal.draw(|f| {
                self.draw(f);
            })?;

            match read_and_parse_input("Game", &command_registry) {
                Ok(command_execution) => {
                    self.game_core.get_command_handler().handle_command(&command_registry, command_execution);
                }
                Err(e) => {
                    eprintln!("{}", e);
                    break;
                }
            }
        }

        ratatui::restore();

        Ok(())
    }

    fn draw(&self, f: &mut ratatui::Frame) {
        // TODO: Implement drawing logic
    }

}