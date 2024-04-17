#![feature(iter_intersperse)]

use clap::Parser;

mod config;
use config::Config;

mod error;
use error::Q3Error;

mod expand;
use expand::*;

mod script;
use script::q3;

mod components;
use components::*;

mod store;
use store::*;

mod cli;
use cli::Cli;

mod tui;
use tui::*;

use std::{error::Error, io};

use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEventKind},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};

use ratatui::prelude::*;

mod parser;
use crate::parser::parse_query;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    pyo3::append_to_inittab!(q3);
    let args = Cli::parse();

    let config = std::fs::read_to_string(args.nsq)?;
    let config: Config = toml::from_str(&config)?;
    let mut queries: QStore = config.try_into()?;

    queries.expand()?;

    let table_data: Vec<TableRow> = queries.into();
    //
    // let mut table = if let Some(id) = args.get {
    //     table_data = table_data
    //         .into_iter()
    //         .filter(|elem| elem.id == id)
    //         .collect();
    //     Table::new(table_data)
    // } else {
    //     Table::new(table_data)
    // };
    //
    // let settings = Settings::new(
    //     Width::increase(40).priority::<PriorityMin>(),
    //     Width::wrap(Percent(70)).priority::<PriorityMax>(),
    // );
    //
    // table
    //     .with(settings)
    //     .modify(Rows::new(1..), Padding::new(0, 0, 0, 1))
    //     .with(Style::rounded());

    // println!("{}", table.to_string());

    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // create app and run it
    let app = App::new(table_data);
    let res = run_app(&mut terminal, app);

    // restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        println!("{err:?}");
    }

    Ok(())
}
