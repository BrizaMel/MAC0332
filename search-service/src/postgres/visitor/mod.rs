use crate::relational::general::{ForeignKey, Table};
use crate::relational::tableSearch::TableSearch;
use crate::traits::Visitor;

use crate::query_representation::intermediary::Command;

use crate::query_representation::ultimate::command_to_query;

use anyhow::Error;

mod tests;

#[derive(Default)]
pub struct PostgresVisitor {
    //TableSearch struct with information on the db's tables
    table_search: TableSearch,
}

impl PostgresVisitor {
    pub fn new(tables: &Vec<Table>, foreign_keys: &Vec<ForeignKey>) -> Self {
        Self {
            table_search: TableSearch::new(tables, foreign_keys),
        }
    }
}

impl Visitor for PostgresVisitor {
    fn visit_command(&self, projection: Vec<String>, command: &Command) -> Result<String, Error> {
        let query = command_to_query(&projection, command, &self.table_search)?;

        Ok(query)
    }
}
