/*
	Module responsible for parsing the initial representation
	of a query to the intermediary representation. It receives the
	json from front-end.
*/

pub mod terminal_expression;
pub mod compound_expression;
mod tests;

use crate::traits::Interpreter;

use crate::query_representation::initial::terminal_expression::TerminalExpression;
use crate::query_representation::initial::compound_expression::{
    OrExpression,
    AndExpression
};

use crate::query_representation::intermediary::Command;

use crate::query_representation::intermediary::simple_command::{
    SimpleCommand,
    Operator,
    Value,
    DataType
};

use anyhow::{Error, Ok};

use super::intermediary::composite_command::{Operation, CompositeCommand};

pub enum Expression {
    TerminalExpression(TerminalExpression),
    OrExpression(OrExpression),
    AndExpression(AndExpression)
}

impl Interpreter for Expression {
    fn interpret(&self, context:serde_json::Value) -> Result<Command, Error> {
        let command = initial_to_command(context)?;
        
        Ok(command)
    }
}

pub fn initial_to_command(initial:serde_json::Value) -> Result<Command, Error> {
    let expression = initial["filters"].to_string().replace('"', "");
    
    let command = assemble_command(expression)?;

    Ok(command)
}

fn assemble_command(expression:String) -> Result<Command, Error> {
    if expression.contains(") AND (") || expression.contains(") OR (") {
        let operation;
        let mut commands = Vec::new();
        
        let parts: Vec<&str>;
        if expression.contains(") AND (") {
            operation = Operation::And;
            parts = expression.split(") AND (").collect();
        } else {
            operation = Operation::Or;
            parts = expression.split(") OR (").collect()
        }
        
        let left = parts[0].replace("(", "");
        let right = parts[1].replace(")", "");
        commands.push(assemble_command(left)?);
        commands.push(assemble_command(right)?);

        let composite_command = CompositeCommand::new(
            operation,
            commands
        );

        Ok(Command::CompositeCommand(composite_command))
    } else if expression.contains(" AND ") || expression.contains(" OR ") {
        let operation;
        let mut commands = Vec::new();
        
        let parts: Vec<&str>;
        if expression.contains(" AND ") {
            operation = Operation::And;
            parts = expression.split(" AND ").collect();
        } else {
            operation = Operation::Or;
            parts = expression.split(" OR ").collect()
        }
        
        let left = parts[0].replace("(", "");
        let right = parts[1].to_string();
        commands.push(assemble_command(left)?);
        commands.push(assemble_command(right)?);

        let composite_command = CompositeCommand::new(
            operation,
            commands
        );
        
        Ok(Command::CompositeCommand(composite_command))
    } else {
        return terminal_expression_to_simple_command(expression);
    }
}

fn terminal_expression_to_simple_command(expression: String) -> Result<Command, Error> {
    let parts: Vec<&str> = expression.split(" ").collect();

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