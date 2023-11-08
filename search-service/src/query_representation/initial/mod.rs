/*
	Module responsible for parsing the initial representation
	of a query to the intermediary representation. It receives a
	json from front-end.
*/

pub mod terminal_expression;
pub mod compound_expression;
mod tests;

use crate::traits::Expression;

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
use crate::query_representation::intermediary::composite_command::{
    Operation,
    CompositeCommand
};

use anyhow::{Error, Ok};

impl Expression for TerminalExpression {
    fn interpret(&self) -> Result<Command, Error> {
        let simple_command = terminal_expression_to_simple_command(self.expression.to_owned())?;

        Ok(simple_command)
    }
}

impl Expression for AndExpression {
    fn interpret(&self) -> Result<Command, Error> {
        let operation = Operation::And;
        let composite_command = compound_expression_to_composite_command(operation, self.left_expression.to_owned(), self.right_expression.to_owned())?;

        Ok(composite_command)
    }
}

impl Expression for OrExpression {
    fn interpret(&self) -> Result<Command, Error> {
        let operation = Operation::Or;
        let composite_command = compound_expression_to_composite_command(operation, self.left_expression.to_owned(), self.right_expression.to_owned())?;

        Ok(composite_command)
    }
}

pub fn initial_to_command(initial:serde_json::Value) -> Result<Command, Error> {
    let expression = initial["filters"].to_string().replace('"', "");
    println!("initial expression {}", expression);
    println!();
    
    let command = parse(expression)?;

    Ok(command)
}

fn parse(expression:String) -> Result<Command, Error> {
    let parts: Vec<&str>;
    let command;
    
    // in each compound expression parsed remove first ocurrence of '(' and last ocurrence of ')'
    if expression.contains(") AND (") {
        parts = expression.split(") AND (").collect();
        let left_expression = parts[0].replace("(", "");
        let mut right_expression = parts[1].to_string();
        let last_occurence = parts[1].rfind(")").unwrap();
        right_expression.remove(last_occurence);
        let and_expression = AndExpression::new(left_expression, right_expression);
        command = and_expression.interpret()?;
    } else if expression.contains(") OR (") {
        parts = expression.split(") OR (").collect();
        let left_expression = parts[0].replace("(", "");
        let mut right_expression = parts[1].to_string();
        let last_occurence = parts[1].rfind(")").unwrap();
        right_expression.remove(last_occurence);
        let or_expression = OrExpression::new(left_expression, right_expression);
        command = or_expression.interpret()?;
    } else if expression.contains(" AND ") {
        parts = expression.split(" AND ").collect();
        let left_expression = parts[0].replace("(", "");
        let right_expression = parts[1].to_string();
        let and_expression = AndExpression::new(left_expression, right_expression);
        command = and_expression.interpret()?;
    } else if expression.contains(" OR ") {
        parts = expression.split(" OR ").collect();
        let left_expression = parts[0].replace("(", "");
        let right_expression = parts[1].to_string();
        let or_expression = OrExpression::new(left_expression, right_expression);
        command = or_expression.interpret()?;
    } else {
        let terminal_expression = TerminalExpression::new(expression);
        command = terminal_expression.interpret()?;
    }

    Ok(command)
}

fn compound_expression_to_composite_command(operation:Operation, left_expression:String, right_expression:String) -> Result<Command, Error> {
    let mut commands = Vec::new();

    commands.push(parse(left_expression)?);
    commands.push(parse(right_expression)?);

    let composite_command = CompositeCommand::new(
        operation,
        commands
    );

    Ok(Command::CompositeCommand(composite_command))

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