#![allow(unused)]

use super::Component;
use super::Visitor;
use anyhow::Error;


#[derive(PartialEq, Debug)]
pub enum Operator {
	Equal,
	GreaterThan,
}

#[derive(PartialEq, Debug)]
pub enum DataType {
	Integer,
	String,
}

#[derive(PartialEq, Debug)]
pub struct Value {
	pub value: String,
	pub data_type: DataType,
}

pub struct SimpleCommand {
    pub attribute: String,
    pub operator: Operator,
    pub value: Value,
}

impl SimpleCommand {
    pub fn new(attribute: String, operator: Operator, value: Value) -> Self {
        Self { attribute, operator, value }
    }
}

impl Component for SimpleCommand {
    fn accept(&self, projection:Vec<String>, v: &'static dyn Visitor) -> Result<String, Error> {
        let query = v.visit_simple_command(projection, self)?;

        Ok(query)
    }
}

impl Value {
    pub fn new(value:String, data_type:DataType) -> Self {
        Self {value,data_type}
    }
}
