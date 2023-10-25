/*
	Module responsible for creating the intermediary representation
	of a query.
*/

pub mod simple_command;
pub mod composite_command;
pub mod tests;

use anyhow::Error;

use crate::traits::{Component,Visitor};

use crate::query_representation::intermediary::simple_command::SimpleCommand;

use crate::query_representation::intermediary::composite_command::CompositeCommand;

#[derive(PartialEq, Debug)]
pub enum Command {
	SimpleCommand(SimpleCommand),
	CompositeCommand(CompositeCommand),
}

impl Component for Command {
    fn accept(&self, projection:Vec<String>, v: &'static dyn Visitor) -> Result<String, Error> {
        let query = v.visit_command(projection, self)?;

        Ok(query)
    }	
}

pub fn get_command_attributes(command:&Command) -> Result<Vec<String>,Error> {

	let mut command_attributes = Vec::new();

	match command {

		Command::CompositeCommand(_) => {
			let Command::CompositeCommand(ref composite_command) = command else {  panic!("Wrong Command type");};
			let nested_commands = &composite_command.commands;
			for nested_command in nested_commands.iter() {
				let mut nested_command_attributes = get_command_attributes(&nested_command)?;
				command_attributes.append(&mut nested_command_attributes);
			}
		}

		Command::SimpleCommand(_) => {
			let Command::SimpleCommand(ref simple_command) = command else {  panic!("Wrong Command type");};
			let attribute = simple_command.attribute.to_owned();
			command_attributes.push(attribute);
		}
	}

	command_attributes.sort();
	command_attributes.dedup();

	Ok(command_attributes)

}