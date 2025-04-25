use ratatui::{
    widgets::{Block, Borders, Paragraph, List, ListItem},
    buffer::Buffer,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    Frame,
    text::{Line, Span, Text},
    style::{Color, Style},
};

use crate::game_core::{PlanetStatus, Resource};

pub struct UI {}

impl UI {
    pub fn new() -> Self {
        UI {}
    }

    pub fn draw(
        &self,
        frame: &mut Frame,
        command_input: &str,
        show_cursor: bool,
        command_input_focused: bool,
        status_focused: bool,
        current_turn: u32,
        player_name: &str,
        planet_status: Option<&PlanetStatus>,
    ) {
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

        // 1. Game Status (Top-Left)
        self.render_game_status(
            frame,
            top_layout[0],
            status_focused,
            current_turn,
            player_name,
            planet_status,
        );

        // 2. Message Log (Top-Left)
        self.render_log(frame, top_layout[1], "Message Log Placeholder");

        // 4. Command Input (Bottom)
        self.render_command_input(
            frame,
            bottom_layout[0],
            command_input,
            show_cursor,
            command_input_focused,
        );
    }

    fn render_game_status(
        &self,
        frame: &mut Frame,
        area: Rect,
        is_focused: bool,
        current_turn: u32,
        player_name: &str,
        planet_status: Option<&PlanetStatus>,
    ) {
        let border_style = if is_focused {
            Style::default().fg(Color::Cyan)
        } else {
            Style::default()
        };

        let status_block = Block::default()
            .title("Game Status")
            .borders(Borders::ALL)
            .border_style(border_style);

        // --- Status Pane Layout ---
        // This is a basic implementation, you'll refine data fetching/display
        let status_layout = Layout::default()
            .direction(Direction::Vertical)
            .margin(1) // Add margin inside the block
            .constraints([
                Constraint::Length(1), // Current Turn
                Constraint::Length(1), // Player Name
                Constraint::Length(1), // Planet Name + Arrows
                Constraint::Min(1),    // Building List (takes remaining space)
                Constraint::Length(1), // Production Rates Title
                Constraint::Length(1), // Energy Prod/Storage
                Constraint::Length(1), // Minerals Prod/Storage
                Constraint::Length(1), // Gas Prod/Storage
            ])
            .split(status_block.inner(area)); // Apply layout *inside* the block
        
        let turn_line = Line::from(format!("Turn: {}", current_turn)).alignment(Alignment::Center);
        frame.render_widget(Paragraph::new(turn_line), status_layout[0]);
    
        // Player Name (Centered)
        let player_line = Line::from(player_name).alignment(Alignment::Center);
        frame.render_widget(Paragraph::new(player_line), status_layout[1]);

        // --- Use data from planet_status if Some, otherwise show defaults ---
        if let Some(status) = planet_status {
            let planet_display = if status.planet_count > 1 {
                format!("< {} >", status.planet_name)
            } else {
                status.planet_name.clone()
            };
            let planet_line = Line::from(planet_display).alignment(Alignment::Center);
            frame.render_widget(Paragraph::new(planet_line), status_layout[2]);

            // Building List
            let building_items: Vec<ListItem> = status
                .buildings
                .iter()
                .map(|(name, level)| ListItem::new(format!("{} Lvl {}", name, level)))
                .collect();
            let building_list = List::new(building_items)
                .block(Block::default().title("Buildings"));
            frame.render_widget(building_list, status_layout[3]);

            // Production & Storage Title
            frame.render_widget(
                Paragraph::new("Production / Storage").alignment(Alignment::Center),
                status_layout[4]
            );

            // Helper closure to get prod/storage safely
            let get_res_info = |resource: Resource| {
                let prod = status.production.get(&resource).cloned().unwrap_or(0);
                let (current, capacity) = status.storage.get(&resource).cloned().unwrap_or((0, 0));
                (prod, current, capacity)
            };

            let (energy_prod, energy_curr, energy_cap) = get_res_info(Resource::Energy);
            let (min_prod, min_curr, min_cap) = get_res_info(Resource::Minerals);
            let (gas_prod, gas_curr, gas_cap) = get_res_info(Resource::Gas);


            // Display Production & Storage
            frame.render_widget(
                Paragraph::new(format!("Energy:   +{}/t | {}/{}", energy_prod, energy_curr, energy_cap)),
                status_layout[5]
            );
            frame.render_widget(
                Paragraph::new(format!("Minerals: +{}/t | {}/{}", min_prod, min_curr, min_cap)),
                status_layout[6]
            );
            frame.render_widget(
                Paragraph::new(format!("Gas:      +{}/t | {}/{}", gas_prod, gas_curr, gas_cap)),
                status_layout[7]
            );

        } else {
            let placeholder = Paragraph::new("No planet data available.")
                .alignment(Alignment::Center);
            frame.render_widget(placeholder, status_layout[2]);
        }

        frame.render_widget(status_block, area);
    }

    // TODO: WIP, replace with actual message log
    fn render_log(&self, frame: &mut Frame, area: Rect, log: &str) {
        let log_block = Block::default().title("Log").borders(Borders::ALL);
        let log_paragraph = Paragraph::new(Text::raw(log)).block(log_block);
        frame.render_widget(log_paragraph, area);
    }

    fn render_command_input(
        &self,
        frame: &mut Frame,
        area: Rect,
        input: &str,
        show_cursor: bool,
        is_focused: bool,
    ) {
        let cursor_char = if show_cursor { "|" } else { " " };

        let border_style = if is_focused {
            Style::default().fg(Color::Cyan)
        } else {
            Style::default()
        };

        let input_block = Block::default()
            .title("Command")
            .borders(Borders::ALL)
            .border_style(border_style);

        let input_paragraph = Paragraph::new(Text::raw(format!("> {}{}", input, cursor_char)))
            .block(input_block);
        frame.render_widget(input_paragraph, area);
    }
}