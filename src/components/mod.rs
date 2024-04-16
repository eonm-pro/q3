use std::fmt::Display;

mod list;
use list::*;

mod generator;
use generator::*;

mod query;
pub use query::Query;

use crate::{
    config::{GeneratorConfig, ListConfig, QueryConfig},
    expand::Expand,
    store::QStore,
    Q3Error
};

#[derive(Debug, Clone, PartialEq)]
pub enum Q3Components {
    List(List),
    Query(Query),
    Generator(Generator),
}

impl TryFrom<(Id, ListConfig)> for Q3Components {
    type Error = Q3Error;

    fn try_from(value: (Id, ListConfig)) -> Result<Self, Self::Error> {
        let (id, config) = value;

        let value = match config.data {
            crate::config::PathOrValue::File(path) => std::fs::read_to_string(path).map_err(|err| {
                Q3Error::FailedToReadDataFromDisk(err)
            })?,
            crate::config::PathOrValue::Value(data) => data,
        };

        Ok(Self::List(List {
            id,
            value,
            separator: config.separator,
            script: config.script,
        }))
    }
}

impl TryFrom<(Id, GeneratorConfig)> for Q3Components {
    type Error = Q3Error;

    fn try_from(value: (Id, GeneratorConfig)) -> Result<Self, Self::Error> {
        let (id, config) = value;

        Ok(Self::Generator(Generator {
            id,
            value: None,
            script: config.script,
        }))
    }
}

impl Expand for Q3Components {
    type State = QStore;

    fn expand(&mut self, state: Self::State) -> Result<Self::State, Q3Error> {
        let state = match self {
            Self::List(list) => list.expand(state)?,
            Self::Query(query) => query.expand(state)?,
            Self::Generator(generator) => generator.expand(state)?,
        };

        Ok(state)
    }
}

impl TryFrom<(Id, QueryConfig)> for Q3Components {
    type Error = Q3Error;

    fn try_from(value: (Id, QueryConfig)) -> Result<Self, Self::Error> {
        let (id, config) = value;
        let query: Query = Query::new(id.0, config.value)?;

        Ok(Q3Components::Query(query))
    }
}

impl Identify for Q3Components {
    fn get_id(&self) -> &Id {
        match self {
            Self::List(list) => list.get_id(),
            Self::Query(query) => query.get_id(),
            Self::Generator(generator) => generator.get_id(),
        }
    }
}

impl Display for Q3Components {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::List(list) => write!(f, "{}", list),
            Self::Query(query) => write!(f, "{}", query),
            Self::Generator(generator) => write!(f, "{}", generator),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Id(pub String);

impl Display for Id {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

pub trait Identify {
    fn get_id(&self) -> &Id;
}
