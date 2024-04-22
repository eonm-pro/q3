use ratatui::{
    style::{Color, Style},
    text::{Line, Span},
    widgets::Widget,
};

#[derive(Clone)]
pub struct Footer(pub Vec<FooterEntry>);

#[derive(Clone)]
pub struct FooterEntry {
    before: Option<char>,
    text: String,
    after: Option<char>,
    bg_color: Color,
    text_color: Color,
}

impl FooterEntry {
    pub fn set_before(&mut self, char: char) -> &mut Self {
        self.before = Some(char);
        self
    }

    pub fn set_after(&mut self, char: char) -> &mut Self {
        self.after = Some(char);
        self
    }

    pub fn set_text<S: Into<String>>(&mut self, text: S) -> &mut Self {
        self.text = text.into();
        self
    }

    pub fn set_bg_color<C: Into<Color>>(&mut self, color: C) -> &mut Self {
        self.bg_color = color.into();
        self
    }

    pub fn set_text_color<C: Into<Color>>(&mut self, color: C) -> &mut Self {
        self.text_color = color.into();
        self
    }
}

impl Default for FooterEntry {
    fn default() -> Self {
        FooterEntry {
            before: None,
            text: "".to_string(),
            after: Some(''),
            bg_color: Color::default(),
            text_color: Color::default(),
        }
    }
}

pub struct FooterEntries<'a>(Vec<Span<'a>>);

impl<'a> Into<FooterEntries<'a>> for (FooterEntry, Option<Color>) {
    fn into(self) -> FooterEntries<'a> {
        let (entry, next_color) = self;
        let mut spans = Vec::with_capacity(3);

        if let Some(before) = entry.before {
            spans.push(Span::styled(
                before.to_string(),
                Style::default().fg(entry.bg_color),
            ))
        }

        spans.push(Span::styled(
            entry.text,
            Style::default().fg(entry.text_color).bg(entry.bg_color),
        ));

        match (entry.after, next_color) {
            (Some(after), Some(color)) => spans.push(Span::styled(
                after.to_string(),
                Style::default().fg(entry.bg_color).bg(color),
            )),
            (Some(after), None) => spans.push(Span::styled(
                after.to_string(),
                Style::default().fg(entry.bg_color),
            )),
            _ => (),
        }

        FooterEntries(spans)
    }
}

impl Widget for Footer {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer)
    where
        Self: Sized,
    {
        let mut line_elements = vec![];
        let mut iter = self.0.into_iter().peekable();

        while let Some(entry) = iter.next() {
            let next_color = iter.peek().map(|next_entry| next_entry.bg_color).to_owned();
            line_elements.append(&mut Into::<FooterEntries>::into((entry, next_color)).0)
        }

        let line = Line::from(line_elements);
        buf.set_line(area.left(), area.top(), &line, 100);
    }
}
