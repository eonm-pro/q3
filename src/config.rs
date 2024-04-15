use std::collections::HashMap;
use std::path::PathBuf;

use serde::Deserialize;

use crate::Id;
use crate::QStore;

use crate::components::Q3Components;

#[derive(Debug, Deserialize)]
pub struct Config {
    #[serde(rename = "query")]
    queries: HashMap<String, QueryConfig>,
    #[serde(rename = "list")]
    lists: Option<HashMap<String, ListConfig>>,
    #[serde(rename = "generator")]
    generators: Option<HashMap<String, GeneratorConfig>>,
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
    type Error = Box<dyn std::error::Error>;

    fn try_from(config: Config) -> Result<Self, Self::Error> {
        let mut qstore = QStore::new();

        if let Some(lists) = config.lists {
            for (id, list_config) in lists {
                let list: Q3Components = ((Id(id), list_config)).try_into()?;
                qstore.insert(list);
            }
        }

        if let Some(generators) = config.generators {
            for (id, generator_config) in generators {
                let generator: Q3Components = ((Id(id), generator_config)).try_into()?;
                qstore.insert(generator);
            }
        }

        for (id, query) in config.queries {
            let query: Q3Components = ((Id(id), query)).try_into()?;
            qstore.insert(query);
        }

        println!("{:?}", qstore);

        Ok(qstore)
    }
}
