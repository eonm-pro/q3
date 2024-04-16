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

mod parser;
use crate::parser::parse_query;

use tabled::{
    builder::Builder,
    settings::{Modify, object::Rows, Alignment, Style, Width}
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    pyo3::append_to_inittab!(q3);
    let args = Cli::parse();

    let config = std::fs::read_to_string(args.nsq)?;
    let config: Config = toml::from_str(&config)?;
    let mut queries: QStore = config.try_into()?;

    queries.expand()?;

    let mut builder = Builder::default();
    builder.push_record(["id".to_string(), "value".to_string()]);

    if let Some(id) = args.get {
        if let Some(query) = queries.get(id.clone()) {
            builder.push_record([id, query.to_string()]);
        }

        let table = builder.build()
            .with(Style::rounded())
            .with(Width::wrap(180))
            .modify(Rows::new(1..), Alignment::left())
            .to_string();

        println!("{}", table);
    } else {
        for (id, query) in queries.components.iter() {
            builder.push_record([query.to_string()]);
        }

        let table = builder.build()
            .with(Style::rounded())
            .with(Width::wrap(180))
            .modify(Rows::new(1..), Alignment::left())
            .to_string();

        println!("{}", table);
    }

    Ok(())
}
