use thiserror::Error;

use crate::components::Id;

#[derive(Error, Debug)]
pub enum Q3Error {
    #[error("Failed to parse query")]
    FailedToParseQuery,
    #[error("Recursive query {0}")]
    FailedToExpand(Id),
}
