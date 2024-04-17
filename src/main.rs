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

use tabled::Table;

mod parser;
use crate::parser::parse_query;

use tabled::settings::object::Rows;
use tabled::settings::Style;
use tabled::settings::{measurement::Percent, Width};

use tabled::settings::{
    peaker::{PriorityMax, PriorityMin},
    Settings, Padding,
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    pyo3::append_to_inittab!(q3);
    let args = Cli::parse();

    let config = std::fs::read_to_string(args.nsq)?;
    let config: Config = toml::from_str(&config)?;
    let mut queries: QStore = config.try_into()?;

    queries.expand()?;

    let mut table_data: Vec<TableRow> = queries.into();

    let mut table = if let Some(id) = args.get {
        table_data = table_data
            .into_iter()
            .filter(|elem| elem.id == id)
            .collect();
        Table::new(table_data)
    } else {
        Table::new(table_data)
    };

    let settings = Settings::new(
        Width::increase(40).priority::<PriorityMin>(),
        Width::wrap(Percent(70)).priority::<PriorityMax>(),
    );

    table
        .with(settings)
        .modify(Rows::new(1..), Padding::new(0, 0, 0, 1))
        .with(Style::rounded());

    println!("{}", table.to_string());

    Ok(())
}
