use crate::traits::Visitor;

use crate::query_representation::intermediary::Command;

use crate::query_representation::ultimate::command_to_query;

use anyhow::Error;

mod tests;

#[derive(Default, Debug)]
pub struct PostgresVisitor;

impl Visitor for PostgresVisitor {
    fn visit_command(&self, projection: Vec<String>, command: &Command) -> Result<String, Error> {
        let query = command_to_query(projection, &command)?;

        Ok(query)
    }
}
