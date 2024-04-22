use super::App;

use ratatui::{
    style::{Color, Stylize},
    text::{Line, Span},
    widgets::Widget,
};

#[derive(Clone)]
pub struct Footer<'a> {
    pub entries: Vec<Span<'a>>,
    pub separator: char,
    pub terminator: char,
}

impl From<&App> for Footer<'_> {
    fn from(app: &App) -> Self {
        let mut entries = vec![Span::from(" q3 ")
            .bg(app.colors.alt_row_color)
            .fg(Color::White)];

        let mut meta_spans: Vec<Span> = app.file.clone().into();
        meta_spans = meta_spans
            .into_iter()
            .map(|span| span.bg(app.colors.header_bg).fg(Color::White))
            .collect();

        entries.append(&mut meta_spans);

        if let Some(message) = &app.message {
            let mut spans: Vec<Span> = message.clone().into();

            spans = spans
                .into_iter()
                .map(|span| span.bg(app.colors.selected_style_fg))
                .collect();

            entries.append(&mut spans);
        }

        Self {
            entries,
            separator: '',
            terminator: '',
        }
    }
}

impl<'a> Widget for Footer<'a> {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer)
    where
        Self: Sized,
    {
        let mut line_elements = vec![];
        let mut iter = self.entries.into_iter().peekable();

        while let (Some(entry), next_element) = (iter.next(), iter.peek()) {
            let fg = entry.style.bg.unwrap_or_default();

            match next_element {
                Some(next_element) => {
                    let next_color = next_element.style.bg.unwrap_or_default();
                    line_elements.push(entry);
                    line_elements
                        .push(Span::from(self.separator.to_string()).bg(next_color).fg(fg));
                }
                None => {
                    line_elements.push(entry);
                    line_elements.push(Span::from(self.terminator.to_string()).fg(fg));
                }
            }
        }

        let line = Line::from(line_elements);
        buf.set_line(area.left(), area.top(), &line, 100);
    }
}
