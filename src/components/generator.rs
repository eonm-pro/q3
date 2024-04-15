use std::fmt::Display;

use super::{Id, Identify};
use crate::store::QStore;
use crate::Expand;

use pyo3::prelude::*;
use pyo3::types::IntoPyDict;

#[derive(Debug, Clone, PartialEq)]
pub struct Generator {
    pub id: Id,
    pub script: String,
    pub value: Option<String>,
}

impl Display for Generator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.value {
            Some(value) => write!(f, "{}", value),
            None => write!(f, ""),
        }
    }
}

impl Identify for Generator {
    fn get_id(&self) -> &Id {
        &self.id
    }
}

impl Expand for Generator {
    type State = QStore;

    fn expand(&mut self, state: QStore) -> Result<Self::State, Box<dyn std::error::Error>> {
        self.value = Some(Python::with_gil(|py| {
            let locals = [("value", None::<String>)].into_py_dict_bound(py);
            py.run_bound(&self.script, Some(&locals), None)?;

            let value: String = locals
                .get_item("value")?
                .ok_or("value not found")?
                .extract()?;

            Ok::<String, Box<dyn std::error::Error>>(value)
        })?);

        Ok(state)
    }
}
