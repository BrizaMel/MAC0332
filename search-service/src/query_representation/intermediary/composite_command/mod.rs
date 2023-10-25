use crate::query_representation::intermediary::Command;

#[derive(PartialEq, Debug)]
pub struct CompositeCommand {
	pub operation: Operation,
	pub commands: Vec<Command>,
}

#[derive(PartialEq, Debug)]
pub enum Operation {
	And,
	Or,
}

impl CompositeCommand {
    pub fn new(operation:Operation,commands:Vec<Command>) -> Self {
        Self {operation,commands}
    }
}