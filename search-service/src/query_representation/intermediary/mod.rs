/*
    Module responsible for creating the intermediary representation
    of a query.
*/

pub mod composite_command;
pub mod simple_command;
pub mod tests;

use std::sync::Arc;

use anyhow::Error;

use crate::traits::{Component, Visitor};

use crate::query_representation::intermediary::simple_command::SingleCommand;

use crate::query_representation::intermediary::composite_command::CompositeCommand;

#[derive(PartialEq, Debug)]
pub enum Command {
    SingleCommand(SingleCommand),
    CompositeCommand(CompositeCommand),
}

impl Component for Command {
    fn accept(&self, projection: Vec<String>, v: Arc<dyn Visitor>) -> Result<String, Error> {
        let query = v.visit_command(projection, self)?;

        Ok(query)
    }
}

pub fn get_command_attributes(command: &Command) -> Vec<String> {
    let mut command_attributes = match command {
        Command::CompositeCommand(cc) => cc
            .commands
            .iter()
            .flat_map(|c| get_command_attributes(&c))
            .collect::<Vec<String>>(),
        Command::SingleCommand(sc) => {
            let attribute = sc.attribute.to_owned();
            vec![attribute]
        }
    };

    command_attributes.sort();
    command_attributes.dedup();

    command_attributes
}
