use std::fmt::Display;

use super::{Id, Identify};
use crate::store::QStore;
use crate::Expand;
use crate::Q3Error;

use pyo3::prelude::*;
use pyo3::types::IntoPyDict;

/// Represents a list of value that can be manipulated with script
#[derive(Debug, Clone, PartialEq)]
pub enum List {
    Raw {
        id: Id,
        separator: String,
        value: String,
        script: Option<String>,
    },
    Expanded {
        id: Id,
        value: String,
    },
}

impl Display for List {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            List::Expanded { value, .. } => write!(f, "{}", value),
            List::Raw { .. } => write!(f, ""),
        }
    }
}

impl Identify for List {
    fn get_id(&self) -> &Id {
        match self {
            List::Raw { id, .. } => id,
            List::Expanded { id, .. } => id,
        }
    }
}

impl Expand for List {
    type State = QStore;

    fn expand(&mut self, state: QStore) -> Result<Self::State, Q3Error> {
        match self {
            Self::Raw {
                id,
                separator,
                script,
                value,
            } => {
                let value = if let Some(script) = script {
                    Python::with_gil(|py| {
                        let value: String = value.to_owned();
                        let separator: String = separator.to_owned();

                        let list_data = value
                            .split(&separator)
                            .filter(|elem| !elem.is_empty())
                            .collect::<Vec<&str>>();

                        let locals = [("value", list_data.clone())].into_py_dict_bound(py);
                        py.run_bound(script, Some(&locals), None)
                            .map_err(Q3Error::PythonScriptFailed)?;

                        let result_value = locals
                            .get_item("value")
                            .map_err(Q3Error::PythonScriptFailed)?
                            .ok_or(Q3Error::PythonScriptVariableNotAssigned)?
                            .extract()
                            .map_err(Q3Error::PythonScriptFailed)?;

                        Ok::<String, Q3Error>(result_value)
                    })?
                } else {
                    value.to_owned()
                };

                *self = Self::Expanded {
                    id: id.clone(),
                    value: value.to_string(),
                };
            }
            Self::Expanded { .. } => {}
        }

        Ok(state)
    }
}
