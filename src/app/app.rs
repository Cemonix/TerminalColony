use std::io::Stdout;
use std::time::{Duration, Instant};
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

use crate::game_core::GameCoreError;
use crate::game_core::GameCore;

use super::log::LogMessage;
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

#[derive(PartialEq, Eq)]
pub enum FocusedPane {
    Status,
    CommandInput,
    // Potentially add Log later if needed
}

pub struct App {
    ui: UI,
    game_core: GameCore,
    input_buffer: String,
    exit: bool,
    show_cursor: bool,
    focused_pane: FocusedPane,
    current_planet_idx: usize,
    logs: Vec<LogMessage>,
}

impl App {
    pub fn new() -> Result<Self, AppError> {
        Ok(
            App {
                ui: UI::new(),
                game_core: GameCore::new(None, None)?,
                input_buffer: String::new(),
                exit: false,
                show_cursor: true,
                focused_pane: FocusedPane::CommandInput,
                current_planet_idx: 0,
                logs: Vec::new(),
            }
        )
    }
    
    pub fn run(&mut self) -> Result<(), AppError> {
        // Initialize terminal
        let mut terminal = Self::init_terminal()?;

        let mut last_blink = Instant::now();
        let blink_interval = Duration::from_millis(500);

        while !self.exit {
            if !self.game_core.is_running() {
                self.exit = true;
                break;
            }

            if last_blink.elapsed() >= blink_interval {
                self.show_cursor = !self.show_cursor;
                last_blink = Instant::now();
            }

            let current_turn = self.game_core.get_current_turn();

            let player_name = self.game_core.get_current_player_name();

            let planet_names = self.game_core.get_current_player_planet_names();
            if self.current_planet_idx >= planet_names.len() {
                self.current_planet_idx = 0;
            }
            let planet_name = &planet_names[self.current_planet_idx];
            let planet_status = self
                .game_core
                .get_current_player_planet_status(planet_name);


            let command_focused = self.focused_pane == FocusedPane::CommandInput;
            let status_focused = self.focused_pane == FocusedPane::Status;

            terminal.draw(|f| {
                self.ui.draw(
                    f,
                    &self.input_buffer,
                    self.show_cursor && command_focused,
                    command_focused,
                    status_focused,
                    current_turn,
                    &player_name,
                    planet_status.as_ref(),
                    &self.logs
                );
            })?;

            // TODO: Maybe poll will not be necessary, game is static most of the time
            if event::poll(Duration::from_millis(100))? {
                if let Event::Key(key_event) = event::read()? {
                    // Only process key presses, not releases
                    if key_event.kind == KeyEventKind::Press {
                        self.handle_key_event(key_event)?;
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

    fn add_log(&mut self, message: LogMessage) {
        const MAX_LOGS: usize = 100; // TODO: Make this configurable
        if self.logs.len() >= MAX_LOGS {
            self.logs.remove(0);
        }
        self.logs.push(message);
    }

    fn handle_key_event(&mut self, key_event: KeyEvent) -> Result<(), AppError> {
        match key_event.code {
            KeyCode::Up => {
                if self.focused_pane == FocusedPane::CommandInput {
                   self.focused_pane = FocusedPane::Status;
                }
            }
            KeyCode::Down => {
                if self.focused_pane == FocusedPane::Status {
                    self.focused_pane = FocusedPane::CommandInput;
                }
            }
            KeyCode::Left => {
                if self.focused_pane == FocusedPane::Status {
                    self.current_planet_idx = (self.current_planet_idx + 1) % self.game_core.get_planet_count();
                }
            }
            KeyCode::Right => {
                if self.focused_pane == FocusedPane::Status {
                    self.current_planet_idx = (
                        self.current_planet_idx + self.game_core.get_planet_count() - 1
                    ) % self.game_core.get_planet_count();
                }
            }
            KeyCode::Tab => {
                if self.focused_pane == FocusedPane::CommandInput {
                    self.focused_pane = FocusedPane::Status;
                } else {
                    self.focused_pane = FocusedPane::CommandInput;
                }
            }
            KeyCode::Esc => {
                self.add_log(LogMessage::info("Quit requested."));
                self.exit = true;
            }
            KeyCode::Enter => {
                if self.focused_pane == FocusedPane::CommandInput {
                    let input = self.input_buffer.trim().to_string();
                    if !input.is_empty() {
                        match self.game_core.execute_command(&input) {
                            Ok(Some(success_msg)) => {
                                self.add_log(LogMessage::success(&success_msg));
                            }
                            Ok(None) => {
                                self.add_log(LogMessage::success("Command executed successfully."));
                            }
                            Err(err) => {
                                self.add_log(LogMessage::error(&err.to_string()));
                            }
                        }
                    }
                    self.input_buffer.clear(); // Clear buffer after processing
                }
            }
            KeyCode::Char(c) => {
                if self.focused_pane == FocusedPane::CommandInput {
                    self.input_buffer.push(c);
                }
            }
            KeyCode::Backspace => {
                if self.focused_pane == FocusedPane::CommandInput && !self.input_buffer.is_empty() {
                    self.input_buffer.pop();
                }
            }
            _ => {}
        }

        Ok(())
    }
}
