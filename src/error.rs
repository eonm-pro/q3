use thiserror::Error;

use crate::components::Id;

#[derive(Error, Debug)]
pub enum Q3Error {
    #[error("Failed to parse query")]
    FailedToParseQuery,
    #[error("Recursive query {0}")]
    FailedToExpand(Id),
    #[error("Failed to parse config file: {0}")]
    FailedToParseConfigFile(#[from] toml::de::Error),
    #[error("Failed to read data from disk: {0}")]
    FailedToReadDataFromDisk(#[from] std::io::Error),
    #[error("Id not found. Id {0} cannot be found in the store")]
    IdNotFound(String),
    #[error("Variable `value` not assigned inside python script")]
    PythonScriptVariableNotAssigned,
    #[error("Python script failed: {0}")]
    PythonScriptFailed(#[from] pyo3::prelude::PyErr),
}
