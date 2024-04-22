use crate::QStore;
use arboard::Clipboard;
mod footer;

use footer::*;

use std::io;

use crossterm::event::{self, Event, KeyCode, KeyEventKind};

use ratatui::{prelude::*, widgets::*};
use style::palette::tailwind;
use unicode_width::UnicodeWidthStr;

pub struct TableRow {
    pub id: String,
    query: String,
    raw_query: String,
}

impl From<QStore> for Vec<TableRow> {
    fn from(value: QStore) -> Self {
        value
            .components
            .into_iter()
            .map(|(id, value)| TableRow {
                id: id.to_string(),
                query: value.to_string(),
                raw_query: value.raw(),
            })
            .collect()
    }
}

const PALETTES: [tailwind::Palette; 5] = [
    tailwind::BLUE,
    tailwind::EMERALD,
    tailwind::INDIGO,
    tailwind::RED,
    tailwind::SKY,
];

const ITEM_HEIGHT: usize = 4;

struct TableColors {
    buffer_bg: Color,
    header_bg: Color,
    header_fg: Color,
    row_fg: Color,
    selected_style_fg: Color,
    normal_row_color: Color,
    alt_row_color: Color,
    footer_border_color: Color,
}

impl TableColors {
    const fn new(color: &tailwind::Palette) -> Self {
        Self {
            buffer_bg: tailwind::SLATE.c950,
            header_bg: color.c900,
            header_fg: tailwind::SLATE.c200,
            row_fg: tailwind::SLATE.c200,
            selected_style_fg: color.c400,
            normal_row_color: tailwind::SLATE.c950,
            alt_row_color: tailwind::SLATE.c900,
            footer_border_color: color.c400,
        }
    }
}

impl TableRow {
    const fn ref_array(&self) -> [&String; 2] {
        [&self.id, &self.raw_query]
    }

    fn id(&self) -> &str {
        &self.id
    }

    fn query(&self) -> &str {
        &self.query
    }

    fn raw_query(&self) -> &str {
        &self.raw_query
    }
}

pub struct App {
    state: TableState,
    file: String,
    items: Vec<TableRow>,
    longest_item_lens: (u16, u16), // order is (id, query)
    scroll_state: ScrollbarState,
    colors: TableColors,
    color_index: usize,
}

impl App {
    pub fn new(data: Vec<TableRow>, file: String) -> Self {
        Self {
            state: TableState::default().with_selected(0),
            file,
            longest_item_lens: constraint_len_calculator(&data),
            scroll_state: ScrollbarState::new((data.len() - 1) * ITEM_HEIGHT),
            colors: TableColors::new(&PALETTES[0]),
            color_index: 0,
            items: data,
        }
    }
    pub fn next(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i >= self.items.len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
        self.scroll_state = self.scroll_state.position(i * ITEM_HEIGHT);
    }

    pub fn previous(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i == 0 {
                    self.items.len() - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
        self.scroll_state = self.scroll_state.position(i * ITEM_HEIGHT);
    }

    pub fn copy_to_clipboard(&mut self) {
        let mut clipboard = Clipboard::new().unwrap();

        if let Some(selected) = self.state.selected() {
            let query = self.items[selected].query();
            clipboard.set_text(query).unwrap();
        }
    }

    pub fn next_color(&mut self) {
        self.color_index = (self.color_index + 1) % PALETTES.len();
    }

    pub fn previous_color(&mut self) {
        let count = PALETTES.len();
        self.color_index = (self.color_index + count - 1) % count;
    }

    pub fn set_colors(&mut self) {
        self.colors = TableColors::new(&PALETTES[self.color_index]);
    }
}

pub fn run_app<B: Backend>(terminal: &mut Terminal<B>, mut app: App) -> io::Result<()> {
    loop {
        terminal.draw(|f| ui(f, &mut app))?;

        if let Event::Key(key) = event::read()? {
            if key.kind == KeyEventKind::Press {
                use KeyCode::*;
                match key.code {
                    Char('q') | Esc => return Ok(()),
                    Char('j') | Down => app.next(),
                    Char('k') | Up => app.previous(),
                    Char('l') | Right => app.next_color(),
                    Char('h') | Left => app.previous_color(),
                    Char('c') | Enter => app.copy_to_clipboard(),
                    _ => {}
                }
            }
        }
    }
}

fn ui(f: &mut Frame, app: &mut App) {
    let rects = Layout::vertical([
        Constraint::Min(5),
        Constraint::Min(5),
        Constraint::Length(1),
    ])
    .split(f.size());

    app.set_colors();
    render_table(f, app, rects[0]);
    render_scrollbar(f, app, rects[0]);
    render_preview(f, app, rects[1]);
    render_footer(f, app, rects[2]);
}

fn render_table(f: &mut Frame, app: &mut App, area: Rect) {
    let header_style = Style::default()
        .fg(app.colors.header_fg)
        .bg(app.colors.header_bg);

    let selected_style = Style::default()
        .add_modifier(Modifier::REVERSED)
        .fg(app.colors.selected_style_fg);

    let header = ["Id", "Query"]
        .into_iter()
        .map(Cell::from)
        .collect::<Row>()
        .style(header_style)
        .height(1);

    let rows = app.items.iter().enumerate().map(|(i, data)| {
        let color = match i % 2 {
            0 => app.colors.normal_row_color,
            _ => app.colors.alt_row_color,
        };

        let item = data.ref_array();

        item.into_iter()
            .map(|content| Cell::from(Text::from(format!("\n{content}\n"))))
            .collect::<Row>()
            .style(Style::new().fg(app.colors.row_fg).bg(color))
            .height(4)
    });

    let bar = " █ ";

    let t = Table::new(
        rows,
        [
            Constraint::Min(app.longest_item_lens.0 + 1),
            Constraint::Min(app.longest_item_lens.1 + 1),
        ],
    )
    .header(header)
    .highlight_style(selected_style)
    .highlight_symbol(Text::from(vec![
        "".into(),
        bar.into(),
        bar.into(),
        "".into(),
    ]))
    .bg(app.colors.buffer_bg)
    .highlight_spacing(HighlightSpacing::Always);
    f.render_stateful_widget(t, area, &mut app.state);
}

fn constraint_len_calculator(items: &[TableRow]) -> (u16, u16) {
    let id_len = items
        .iter()
        .map(TableRow::id)
        .map(UnicodeWidthStr::width)
        .max()
        .unwrap_or(0);

    let query_len = items
        .iter()
        .map(TableRow::raw_query)
        .flat_map(str::lines)
        .map(UnicodeWidthStr::width)
        .max()
        .unwrap_or(0);

    #[allow(clippy::cast_possible_truncation)]
    (id_len as u16, query_len as u16)
}

fn render_scrollbar(f: &mut Frame, app: &mut App, area: Rect) {
    f.render_stateful_widget(
        Scrollbar::default()
            .orientation(ScrollbarOrientation::VerticalRight)
            .begin_symbol(None)
            .end_symbol(None),
        area.inner(&Margin {
            vertical: 1,
            horizontal: 1,
        }),
        &mut app.scroll_state,
    );
}

fn render_footer(f: &mut Frame, app: &App, area: Rect) {
    let footer = Footer(vec![
        FooterEntry::default()
            .set_text(" q3 ")
            .set_bg_color(app.colors.alt_row_color)
            .set_text_color(Color::White)
            .clone(),
        FooterEntry::default()
            .set_text(&app.file)
            .set_bg_color(app.colors.header_bg)
            .set_text_color(Color::White)
            .clone(),
        FooterEntry::default()
            .set_text(" lorem ipsum ")
            .set_bg_color(app.colors.selected_style_fg)
            .set_text_color(Color::Black)
            .set_after('')
            .clone(),
    ]);

    f.render_widget(footer, area);
}

fn render_preview(f: &mut Frame, app: &App, area: Rect) {
    let selected_text = match app.state.selected() {
        Some(selected) => app.items[selected].query(),
        None => "",
    };

    let preview = Paragraph::new(selected_text)
        .wrap(Wrap { trim: true })
        .style(Style::new().fg(app.colors.row_fg).bg(app.colors.buffer_bg))
        .block(
            Block::default()
                .borders(Borders::TOP)
                .border_style(Style::new().fg(app.colors.footer_border_color))
                .border_type(BorderType::Plain),
        );
    f.render_widget(preview, area);
}
