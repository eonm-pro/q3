use ratatui::{
    style::Style,
    widgets::{Block, BorderType, Borders, Paragraph, Widget, Wrap},
};

use super::App;

/// A preview
pub struct Preview {
    data: Option<String>,
    style: Style,
}

impl From<&App> for Preview {
    fn from(app: &App) -> Self {
        Preview {
            data: app
                .state
                .selected()
                .map(|selected| app.items[selected].query().into()),
            style: Style::default()
                .fg(app.colors.row_fg)
                .bg(app.colors.buffer_bg),
        }
    }
}

impl Widget for Preview {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer)
    where
        Self: Sized,
    {
        let preview = Paragraph::new(self.data.unwrap_or("".into()))
            .wrap(Wrap { trim: true })
            .style(self.style)
            .block(
                Block::default()
                    .borders(Borders::TOP)
                    .border_style(self.style)
                    .border_type(BorderType::Plain),
            );

        preview.render(area, buf)
    }
}
