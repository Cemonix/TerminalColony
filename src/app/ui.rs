use ratatui::text::Text;
use ratatui::Frame;
use ratatui::widgets::{Block, Borders, Paragraph, Widget};
use ratatui::buffer::Buffer;
use ratatui::layout::{Constraint, Direction, Layout, Rect};


pub struct UI {}

impl UI {
    pub fn new() -> Self {
        UI {}
    }

    pub fn draw(&self, frame: &mut Frame, command_input: &str, show_cursor: bool) {
        let main_layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Percentage(75),
                Constraint::Percentage(25),
            ])
            .split(frame.area());

        let top_layout = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Percentage(50),
                Constraint::Percentage(50),
            ])
            .split(main_layout[0]);

        let bottom_layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Percentage(100)
            ])
            .split(main_layout[1]);
        
        // --- Render Widgets ---

        // 1.Game Status (Top-Left)
        self.render_game_status(frame, top_layout[0], "Game Status Placeholder");

        // 2. Message Log (Top-Left)
        self.render_log(frame, top_layout[1], "Message Log Placeholder");

        // 4. Command Input (Bottom)
        self.render_command_input(frame, bottom_layout[0], command_input, show_cursor);

        // Optional: Place cursor at the end of the input line
        // frame.set_cursor_position(
        //     // Put cursor past the end of the input text
        //     bottom_layout[1].x + state.current_input.len() as u16 + 1,
        //     // Position cursor vertically in the middle of the input block
        //     bottom_layout[1].y + 1,
        // );
    }

    // TODO: Game status is placeholder, replace with actual game status
    fn render_game_status(&self, frame: &mut Frame, area: Rect, game_status: &str) {
        let status_block = Block::default().title("Game Status").borders(Borders::ALL);
        let status_paragraph = Paragraph::new(Text::raw(game_status)).block(status_block);
        frame.render_widget(status_paragraph, area);
    }

    // TODO: WIP, replace with actual message log
    fn render_log(&self, frame: &mut Frame, area: Rect, log: &str) {
        let log_block = Block::default().title("Log").borders(Borders::ALL);
        let log_paragraph = Paragraph::new(Text::raw(log)).block(log_block);
        frame.render_widget(log_paragraph, area);
    }

    fn render_command_input(&self, frame: &mut Frame, area: Rect, input: &str, show_cursor: bool) {
        let cursor_char = if show_cursor { "|" } else { " " };
        let input_block = Block::default().title("Command").borders(Borders::ALL);
        let input_paragraph = Paragraph::new(Text::raw(format!("> {}{}", input, cursor_char)))
            .block(input_block);
        frame.render_widget(input_paragraph, area);
    }
    
}