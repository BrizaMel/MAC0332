/*
    Module responsible for creating the intermediary representation
    of a query.
*/

pub mod composite_command;
pub mod single_command;
pub mod tests;

use std::sync::Arc;

use anyhow::Error;

use crate::traits::{Component, Visitor};

use crate::query_representation::intermediary::single_command::{SingleCommand,DataType};

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

            let mut attributes = vec![attribute];
            if let DataType::Attribute = sc.value.data_type {
               attributes.push(sc.value.value.to_owned());
            }

            attributes
        }
    };

    command_attributes.sort();
    command_attributes.dedup();

    command_attributes
}
