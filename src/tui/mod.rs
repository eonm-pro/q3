use crate::QStore;

use tabled::{Table, Tabled};

use tabled::{
    builder::Builder,
    settings::{object::Rows, Alignment, Modify, Style, Width},
};

#[derive(Tabled)]
pub struct TableRow {
    pub id: String,
    query: String,
}

impl From<QStore> for Vec<TableRow> {
    fn from(value: QStore) -> Self {
        value
            .components
            .into_iter()
            .map(|(id, value)| TableRow {
                id: id.to_string(),
                query: value.to_string(),
            })
            .collect()
    }
}
