use indexmap::IndexMap;

use std::collections::HashSet;
use std::fmt::Display;

use crate::error::Q3Error;
use crate::{Expand, Id, Identify, Q3Components, Query};

/// A struct that holds queries and lists
#[derive(Debug, Clone, PartialEq)]
pub struct QStore {
    failed_expansions: HashSet<Id>,
    pub components: IndexMap<Id, Q3Components>,
}

impl QStore {
    pub fn new() -> Self {
        Self {
            components: IndexMap::new(),
            failed_expansions: HashSet::new(),
        }
    }

    pub fn get<S: Into<String>>(&self, id: S) -> Option<Q3Components> {
        self.components.get(&Id(id.into())).cloned()
    }

    pub fn get_raw<S: Into<String>>(&self, id: S) -> Option<String> {
        self.components
            .get(&Id(id.into()))
            .map(|component| component.raw())
    }

    pub fn insert(&mut self, component: Q3Components) {
        self.components
            .insert(component.get_id().clone(), component);
    }

    pub fn expand(&mut self) -> Result<Self, Box<dyn std::error::Error>> {
        Expand::expand(self, self.clone())?;

        Ok(self.clone())
    }

    pub fn set_failed_expansion(&mut self, id: Id) -> Result<(), Q3Error> {
        match self.failed_expansions.insert(id.clone()) {
            true => Ok(()),
            false => Err(Q3Error::FailedToExpand(id)),
        }
    }

    pub fn remove_failed_expansion(&mut self, id: &Id) {
        self.failed_expansions.remove(id);
    }
}

impl Display for QStore {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut result: Vec<String> = Vec::new();

        for (key, value) in &self.components {
            result.push(format!("{key} : {value}"));
        }

        write!(f, "{}", result.join("\n"))
    }
}

impl Expand for QStore {
    type State = Self;

    fn expand(&mut self, mut state: Self::State) -> Result<Self::State, Q3Error> {
        while self
            .components
            .values()
            .any(|value| matches!(value, Q3Components::Query(Query::Raw { .. })))
        {
            for component in self.components.values_mut() {
                component.expand(state.clone())?;
            }

            state = self.clone();
        }

        Ok(state)
    }
}
