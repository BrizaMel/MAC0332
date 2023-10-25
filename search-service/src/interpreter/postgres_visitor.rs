use super::Visitor;
use crate::{interpreter::SimpleCommand, interpreter::CompositeCommand};
use anyhow::Error;

#[derive(Default, Debug)]
pub struct PostgresVisitor;

impl Visitor for PostgresVisitor {

    fn visit_simple_command(&self, _projection: Vec<String>, _command: &SimpleCommand) -> Result<String, Error> {
        // TODO: Write functions to create Postgres query
        let query = "SimpleCommand interpretation not implemented for Postgres yet".to_string();

        Ok(query)

    }
    fn visit_composite_command(&self, _projection: Vec<String>, _command: &CompositeCommand) -> Result<String, Error> {
        // TODO: Write function to create Postgres query
        let query = "CompositeCommand interpretation not implemented for Postgres yet".to_string();

        Ok(query)
    }
}