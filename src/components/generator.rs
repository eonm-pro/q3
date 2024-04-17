use std::fmt::Display;

use super::{Id, Identify};
use crate::store::QStore;
use crate::Expand;
use crate::Q3Error;

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

    fn expand(&mut self, state: QStore) -> Result<Self::State, Q3Error> {
        self.value = Some(Python::with_gil(|py| {
            let locals = [("value", None::<String>)].into_py_dict_bound(py);
            py.run_bound(&self.script, Some(&locals), None)
                .map_err(|err| Q3Error::PythonScriptFailed(err))?;

            let value: String = locals
                .get_item("value")
                .map_err(|err| Q3Error::PythonScriptFailed(err))?
                .ok_or(Q3Error::PythonScriptVariableNotAssigned)?
                .extract()
                .map_err(|err| Q3Error::PythonScriptFailed(err))?;

            Ok::<String, Q3Error>(value)
        })?);

        Ok(state)
    }
}
