mod game_core;
mod app;

use crate::app::App;

fn main() {
    // TODO: Handle error
    let _ = App::new().unwrap().run();
}

// TODO: Implement command execution.
// TODO: Change help command for question mark which will show help for all commands
// TODO: Change quit command for exiting the app and ask for confirmation
// TODO: Main menu - new game, load game, settings