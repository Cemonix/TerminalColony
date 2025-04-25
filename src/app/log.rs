use ratatui::style::{Color, Style};

#[derive(Clone, Debug)]
pub enum LogLevel {
    Info,
    Error,
    Success,
}

#[derive(Clone, Debug)]
pub struct LogMessage {
    pub level: LogLevel,
    pub text: String,
}

impl LogMessage {
    pub fn info(text: &str) -> Self {
        Self { level: LogLevel::Info, text: text.to_string() }
    }

    pub fn error(text: &str) -> Self {
        Self { level: LogLevel::Error, text: text.to_string() }
    }

     pub fn success(text: &str) -> Self {
        Self { level: LogLevel::Success, text: text.to_string() }
     }
}