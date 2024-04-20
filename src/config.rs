use std::path::PathBuf;

use serde::Deserialize;

use crate::{Id, Q3Error, QStore};

use crate::components::Q3Components;
use indexmap::IndexMap;

#[derive(Debug, Deserialize)]
pub struct Config(IndexMap<String, ConfigElement>);

#[derive(Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ConfigElement {
    Query(QueryConfig),
    List(ListConfig),
    Generator(GeneratorConfig),
}

#[derive(Debug, Deserialize)]
pub struct QueryConfig {
    pub value: String,
}

#[derive(Debug, Deserialize)]
pub struct ListConfig {
    #[serde(flatten)]
    pub data: PathOrValue,
    pub separator: String,
    pub script: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum PathOrValue {
    File(PathBuf),
    Value(String),
}

#[derive(Debug, Deserialize)]
pub struct GeneratorConfig {
    pub script: String,
}

impl TryFrom<Config> for QStore {
    type Error = Q3Error;

    fn try_from(config: Config) -> Result<Self, Self::Error> {
        let mut qstore = QStore::new();

        for (key, elem) in config.0 {
            let component: Q3Components = match elem {
                ConfigElement::Query(config) => (Id(key), config).try_into()?,
                ConfigElement::List(config) => (Id(key), config).try_into()?,
                ConfigElement::Generator(config) => (Id(key), config).try_into()?,
            };

            qstore.insert(component);
        }

        Ok(qstore)
    }
}
