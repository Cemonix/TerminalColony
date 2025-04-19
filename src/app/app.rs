use std::io::Stdout;
use std::time::Duration;
use std::error::Error;

use ratatui::buffer::Buffer;
use ratatui::crossterm::event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEvent, KeyEventKind, MouseEventKind};
use ratatui::crossterm::execute;
use ratatui::crossterm::terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen};
use ratatui::layout::Rect;
use ratatui::prelude::CrosstermBackend;
use ratatui::style::{Color, Style};
use ratatui::widgets::Widget;
use ratatui::{Frame, Terminal};

use crate::game_core::game_core::GameCoreError;
use crate::game_core::GameCore;

use super::ui::UI;

#[derive(Debug)]
pub enum AppError {
    Io(std::io::Error),
    GameCoreError(GameCoreError),
}

impl std::fmt::Display for AppError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AppError::Io(err) => write!(f, "IO error: {}", err),
            AppError::GameCoreError(err) => write!(f, "GameCore error: {}", err),
        }
    }
}

impl Error for AppError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            AppError::Io(err) => Some(err),
            AppError::GameCoreError(err) => Some(err),
        }
    }
}

impl From<std::io::Error> for AppError {
    fn from(err: std::io::Error) -> Self {
        AppError::Io(err)
    }
}

impl From<GameCoreError> for AppError {
    fn from(err: GameCoreError) -> Self {
        AppError::GameCoreError(err)
    }
}

// =================================================================================================

pub struct App {
    ui: UI,
    game_core: GameCore,
    input_buffer: String,
    exit: bool,
}

impl App {
    pub fn new() -> Result<Self, AppError> {
        Ok(
            App {
                ui: UI::new(),
                game_core: GameCore::new(None, None)?,
                input_buffer: String::new(),
                exit: false,
            }
        )
    }
    
    pub fn run(&mut self) -> Result<(), AppError> {
        // Initialize terminal
        let mut terminal = Self::init_terminal()?;

        while !self.exit {
            terminal.draw(|f| {
                self.ui.draw(f);
            })?;

            // TODO: Maybe poll will not be necessary, game is static most of the time
            if event::poll(Duration::from_millis(100))? {
                if let Event::Key(key_event) = event::read()? {
                    // Only process key presses, not releases
                    if key_event.kind == KeyEventKind::Press {
                        self.handle_key_event(key_event);
                    }
                }
                // TODO: handle other events like Mouse or Resize here if needed
            }
        }

        // Restore terminal
        Self::restore(&mut terminal)?;

        Ok(())
    }

    fn init_terminal() -> Result<Terminal<CrosstermBackend<Stdout>>, AppError> {
        enable_raw_mode()?;
        let mut stdout = std::io::stdout();
        execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
        let backend = CrosstermBackend::new(stdout);
        Terminal::new(backend).map_err(AppError::Io)
    }

    fn restore(terminal: &mut Terminal<CrosstermBackend<Stdout>>) -> Result<(), AppError> {
        disable_raw_mode()?;
        execute!(
            terminal.backend_mut(),
            LeaveAlternateScreen,
            DisableMouseCapture
        )?;
        terminal.show_cursor()?;

        Ok(())
    }

    fn handle_key_event(&mut self, key_event: KeyEvent) {
        match key_event.code {
            KeyCode::Esc => {
                self.exit = true;
            }
            KeyCode::Enter => {
                // TODO: Process the command
            }
            KeyCode::Char(c) => {
                self.input_buffer.push(c);
            }
            KeyCode::Backspace => {
                self.input_buffer.pop();
            }
            // TODO: Handle other keys like arrow keys for history, Home/End, Delete if needed
            _ => {}
        }
    }
}
