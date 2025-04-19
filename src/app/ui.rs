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

    pub fn draw(&self, frame: &mut Frame) {
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
        let main_block = Block::default().title("Status").borders(Borders::ALL);
        let main_paragraph = Paragraph::new(Text::raw("Status")).block(main_block);
        frame.render_widget(main_paragraph, top_layout[0]);

        // 2. Message Log (Top-Left)
        let log_block = Block::default().title("Log").borders(Borders::ALL);
        // Simple display, join messages with newline. Consider List widget for more complex logs.
        // let log_text = state.message_log.join("\n");
        let log_paragraph = Paragraph::new(Text::raw("Log text")).block(log_block);
        frame.render_widget(log_paragraph, top_layout[1]);

        // 4. Command Input (Bottom)
        let input_block = Block::default().title("Command").borders(Borders::ALL);
        let input_paragraph = Paragraph::new(Text::raw(">")).block(input_block);
        frame.render_widget(input_paragraph, bottom_layout[0]);

        // Optional: Place cursor at the end of the input line
        // frame.set_cursor_position(
        //     // Put cursor past the end of the input text
        //     bottom_layout[1].x + state.current_input.len() as u16 + 1,
        //     // Position cursor vertically in the middle of the input block
        //     bottom_layout[1].y + 1,
        // );
    }
    
}