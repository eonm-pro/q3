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

fn main() -> Result<(), Box<dyn std::error::Error>> {
    pyo3::append_to_inittab!(q3);
    let args = Cli::parse();

    let config = std::fs::read_to_string(args.nsq)?;
    let config: Config = toml::from_str(&config)?;
    let mut queries: QStore = config.try_into()?;

    queries.expand()?;

    if let Some(id) = args.get {
        if let Some(query) = queries.get(id) {
            println!("{}", query);
        }
    } else {
        println!("{}", queries);
    }

    Ok(())
}
