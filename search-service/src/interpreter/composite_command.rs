#![allow(unused)]

use super::Component;
use super::SimpleCommand;
use super::Visitor;
use anyhow::Error;


#[derive(PartialEq, Debug)]
pub enum LogicalOperator {
	And,
	Or,
}

pub struct CompositeCommand {
	pub logical_operator: LogicalOperator,
	pub commands: Vec<Box<dyn Component>>,
}

impl CompositeCommand {
    pub fn new(logical_op: LogicalOperator, command_1: impl Component + 'static, command_2: impl Component + 'static) -> Self {
        Self {
            logical_operator: logical_op,
            commands: vec![Box::new(command_1), Box::new(command_2)],
        }
    }
}


impl Component for CompositeCommand {
    fn accept(&self, projection: Vec<String>, v: &'static dyn Visitor) -> Result<String, Error> {
        let query = v.visit_composite_command(projection, self)?;

        Ok(query)
    }
}