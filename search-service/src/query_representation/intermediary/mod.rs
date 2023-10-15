/*
	Module responsible for creating the intermediary representation
	of a query.
*/

use anyhow::Error;

mod tests;

#[derive(PartialEq, Debug)]
pub enum DataType {
	Integer,
	String,
}

#[derive(PartialEq, Debug)]
pub enum Command {
	SimpleCommand(SimpleCommand),
	CompositeCommand(CompositeCommand),
}

#[derive(PartialEq, Debug)]
pub enum Operator {
	Equal,
	GreaterThan,
}

#[derive(PartialEq, Debug)]
pub enum Operation {
	And,
	Or,
}

#[derive(PartialEq, Debug)]
pub struct Value {
	value:String,
	data_type:DataType,
}

#[derive(PartialEq, Debug)]
pub struct SimpleCommand {
    attribute: String,
    operator: Operator,
    value: Value,
}

#[derive(PartialEq, Debug)]
pub struct CompositeCommand {
	operation: Operation,
	commands: Vec<Command>,
}


impl Value {
    pub fn new(value:String,data_type:DataType) -> Self {
        Self {value,data_type}
    }
}

impl SimpleCommand {
    pub fn new(attribute:String,operator:Operator,value:Value) -> Self {
        Self {attribute,operator,value}
    }
}

impl CompositeCommand {
    pub fn new(operation:Operation,commands:Vec<Command>) -> Self {
        Self {operation,commands}
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