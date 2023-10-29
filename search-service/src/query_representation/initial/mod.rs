/*
	Module responsible for parsing the initial representation
	of a query to the intermediary representation. It receives the
	json from front-end.
*/

pub mod terminal_expression;
mod tests;

use crate::traits::Interpreter;

use crate::query_representation::initial::terminal_expression::TerminalExpression;

use crate::query_representation::intermediary::Command;

use crate::query_representation::intermediary::simple_command::{
    SimpleCommand,
    Operator,
    Value,
    DataType
};

use anyhow::{Error, Ok};

pub enum Expression {
    TerminalExpression(TerminalExpression),
    // CompoundExpression(CompoundExpression),
}

impl Interpreter for Expression {
    fn interpret(&self, context:serde_json::Value) -> Result<Command, Error> {
        let command = initial_to_command(context)?;
        
        Ok(command)
    }
}

pub fn initial_to_command(initial:serde_json::Value) -> Result<Command, Error> {
    let filters = initial["filters"].to_string().replace('"', "");
    let parts: Vec<&str> = filters.split(" ").collect();

    let attribute = parts[0].to_string();

    let operator = match parts[1] {
        "eq" => Operator::EqualTo,
        "gt" => Operator::GreaterThan,
        "lt" => Operator::LessThan,
        "ge" => Operator::GreaterThanOrEqualTo,
        "le" => Operator::LessThanOrEqualTo,
        "ne" => Operator::NotEqualTo,
        &_ => panic!("Wrong Operator type")
    };

    let value = match parts[2].to_string().parse::<f64>().is_ok() {
        true => Value::new(parts[2].to_string(), DataType::Integer),
        false => Value::new(parts[2].to_string(), DataType::String),
    };

    let command = SimpleCommand::new(
        attribute,
        operator,
        value
    );

    Ok(Command::SimpleCommand(command))
}