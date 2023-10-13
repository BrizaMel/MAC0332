/*
	Module responsible for creating the intermediary representation
	of a query.
*/

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
    property: String,
    operator: Operator,
    value: Value,
}

#[derive(PartialEq, Debug)]
pub struct CompositeCommand {
	operation: Operation,
	command: Vec<Command>,
}


impl Value {
    pub fn new(value:String,data_type:DataType) -> Self {
        Self {value,data_type}
    }
}

impl SimpleCommand {
    pub fn new(property:String,operator:Operator,value:Value) -> Self {
        Self {property,operator,value}
    }
}

impl CompositeCommand {
    pub fn new(operation:Operation,command:Vec<Command>) -> Self {
        Self {operation,command}
    }
}