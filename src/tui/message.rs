use ratatui::{
    style::{Color, Stylize},
    text::Span,
};

#[derive(Debug, Clone)]
pub enum Message {
    Info(String),
    Warning(String),
    Error(String),
}

impl<'a> From<Message> for Vec<Span<'a>> {
    fn from(message: Message) -> Self {
        match message {
            Message::Info(msg) => vec![Span::from(msg).fg(Color::Black)],
            Message::Warning(msg) => vec![
                Span::from("  ").bold().fg(Color::Yellow),
                Span::from(msg).fg(Color::Black),
            ],
            Message::Error(msg) => vec![
                Span::from("  ").bold().fg(Color::Red),
                Span::from(msg).fg(Color::Black).bold(),
            ],
        }
    }
}
