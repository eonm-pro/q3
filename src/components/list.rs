use std::fmt::Display;

use super::{Id, Identify};
use crate::store::QStore;
use crate::Expand;

use pyo3::prelude::*;
use pyo3::types::IntoPyDict;

#[derive(Debug, Clone, PartialEq)]
pub struct List {
    pub id: Id,
    pub value: String,
    pub separator: String,
    pub script: Option<String>,
}

impl Display for List {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl Identify for List {
    fn get_id(&self) -> &Id {
        &self.id
    }
}

impl Expand for List {
    type State = QStore;

    fn expand(&mut self, state: QStore) -> Result<Self::State, Box<dyn std::error::Error>> {
        if let Some(script) = &self.script {
            self.value = Python::with_gil(|py| {
                let list_data = self
                    .value
                    .split(&self.separator)
                    .filter(|elem| !elem.is_empty())
                    .collect::<Vec<&str>>();

                let locals = [("value", list_data.clone())].into_py_dict_bound(py);
                py.run_bound(script, Some(&locals), None)?;

                let value: String = locals
                    .get_item("value")?
                    .ok_or("value not found")?
                    .extract()?;

                Ok::<String, Box<dyn std::error::Error>>(value)
            })?;
        };

        Ok(state)
    }
}
