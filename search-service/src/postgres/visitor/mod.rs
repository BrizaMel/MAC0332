use crate::relational::table_search::TableSearch;
use crate::relational::table_search::entities::TableSearchInfo;
use crate::relational::entities::{Table,ForeignKey};
use crate::traits::Visitor;

use crate::query_representation::intermediary::Command;

use crate::query_representation::r#final::command_to_query;

use anyhow::Error;

mod tests;

pub struct PostgresVisitor {
    //TableSearch struct with information on the db's tables
    table_search: TableSearch,
}

impl PostgresVisitor {
    pub fn new(tables: &Vec<Table>, foreign_keys: &Vec<ForeignKey>) -> Self {
        let tables_search_info: Vec<TableSearchInfo> = tables
        .clone()
        .into_iter()
        .map(TableSearchInfo::from)
        .collect();

        Self {
            table_search: TableSearch::new(&tables_search_info, foreign_keys)
        }
    }
}

impl Visitor for PostgresVisitor {

    fn visit_command(&self, projection: Vec<String>, command: &Command)  -> Result<String, Error> {

        let query = command_to_query(&projection,command,&self.table_search)?;

        Ok(query)
    }

}