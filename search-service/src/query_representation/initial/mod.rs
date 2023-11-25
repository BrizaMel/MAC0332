/*
    Module responsible for parsing the initial representation
    of a query to the intermediary representation. It receives a
    json from front-end.
*/

pub mod compound_expression;
pub mod terminal_expression;
mod tests;

use crate::traits::Expression;

use crate::query_representation::initial::compound_expression::{AndExpression, OrExpression};
use crate::query_representation::initial::terminal_expression::TerminalExpression;

use crate::query_representation::intermediary::composite_command::{
    CompositeCommand, LogicalOperator,
};
use crate::query_representation::intermediary::single_command::{
    DataType, Operator, SingleCommand, Value,
};
use crate::query_representation::intermediary::Command;

use anyhow::{Error, Ok};

impl Expression for TerminalExpression {
    fn interpret(&self) -> Result<Command, Error> {
        let simple_command = terminal_expression_to_simple_command(self.expression.to_owned())?;

        Ok(simple_command)
    }
}

impl Expression for AndExpression {
    fn interpret(&self) -> Result<Command, Error> {
        let operation = LogicalOperator::And;
        let composite_command = compound_expression_to_composite_command(
            operation,
            self.left_expression.to_owned(),
            self.right_expression.to_owned(),
        )?;

        Ok(composite_command)
    }
}

impl Expression for OrExpression {
    fn interpret(&self) -> Result<Command, Error> {
        let operation = LogicalOperator::Or;
        let composite_command = compound_expression_to_composite_command(
            operation,
            self.left_expression.to_owned(),
            self.right_expression.to_owned(),
        )?;

        Ok(composite_command)
    }
}

pub fn initial_to_command(initial: serde_json::Value) -> Result<Command, Error> {
    let expression = initial["filters"].to_string().replace('"', "");
    println!("initial expression {}", expression);
    println!();

    let command = parse(expression)?;

    Ok(command)
}

fn parse(expression: String) -> Result<Command, Error> {
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

fn compound_expression_to_composite_command(
    operation: LogicalOperator,
    left_expression: String,
    right_expression: String,
) -> Result<Command, Error> {
    let mut commands = Vec::new();

    commands.push(parse(left_expression)?);
    commands.push(parse(right_expression)?);

    let composite_command = CompositeCommand::new(operation, commands);

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
        &_ => panic!("Wrong Operator type"),
    };

    let value = match parts[2].to_string().parse::<f64>().is_ok() {
        true => Value::new(parts[2].to_string(), DataType::Integer),
        false => {
            if string_is_attribute(parts[2].to_string())? {
                Value::new(parts[2].to_string(), DataType::Attribute)
            }
            else{
                Value::new(parts[2].to_string(), DataType::String)
            }
        }
    };

    let command = SingleCommand::new(attribute, operator, value);

    Ok(Command::SingleCommand(command))
}

fn string_is_attribute(string: String) -> Result<bool, Error> {

    let split_by_dot = string.split(".");
    let collection = split_by_dot.collect::<Vec<&str>>();

    let is_attribute = collection.len() == 3;
    
    Ok(is_attribute)
}

#[cfg(test)]
mod private_tests {

    use super::*;

    #[test]
    fn test_parse_with_terminal_expression_only() -> Result<(), Error> {
        let expression = "movies.movie.revenue gt 100000".to_string();

        let simple_command = SingleCommand::new(
            "movies.movie.revenue".to_string(),
            Operator::GreaterThan,
            Value::new(100000.to_string(), DataType::Integer),
        );

        let command = Command::SingleCommand(simple_command);

        assert_eq!(parse(expression)?, command);

        Ok(())
    }

    #[test]
    fn test_parse_with_composite_expression_no_parenthesis() -> Result<(), Error> {
        let expression =
            "movies.movie.revenue gt 100000 AND movies.movie.genre eq Comedy".to_string();

        let operation = LogicalOperator::And;
        let mut commands: Vec<Command> = Vec::new();

        let simple_command_1 = SingleCommand::new(
            "movies.movie.revenue".to_string(),
            Operator::GreaterThan,
            Value::new(100000.to_string(), DataType::Integer),
        );
        let simple_command_2 = SingleCommand::new(
            "movies.movie.genre".to_string(),
            Operator::EqualTo,
            Value::new("Comedy".to_string(), DataType::String),
        );

        commands.push(Command::SingleCommand(simple_command_1));
        commands.push(Command::SingleCommand(simple_command_2));

        let composite_command = CompositeCommand::new(operation, commands);

        let command = Command::CompositeCommand(composite_command);

        assert_eq!(parse(expression)?, command);

        Ok(())
    }

    #[test]
    fn test_parse_with_composite_expression_with_parenthesis() -> Result<(), Error> {
        let expression =
            "(movies.movie.revenue gt 100000) AND (movies.movie.genre eq Comedy)".to_string();

        let operation = LogicalOperator::And;
        let mut commands: Vec<Command> = Vec::new();

        let simple_command_1 = SingleCommand::new(
            "movies.movie.revenue".to_string(),
            Operator::GreaterThan,
            Value::new(100000.to_string(), DataType::Integer),
        );
        let simple_command_2 = SingleCommand::new(
            "movies.movie.genre".to_string(),
            Operator::EqualTo,
            Value::new("Comedy".to_string(), DataType::String),
        );

        commands.push(Command::SingleCommand(simple_command_1));
        commands.push(Command::SingleCommand(simple_command_2));

        let composite_command = CompositeCommand::new(operation, commands);

        let command = Command::CompositeCommand(composite_command);

        assert_eq!(parse(expression)?, command);

        Ok(())
    }

    #[test]
    fn test_compound_expression_to_composite_command() -> Result<(), Error> {
        let left_expression = "movies.movie.release_date lt 01-01-2000".to_string();
        let right_expression = "movies.movie.genre eq Comedy".to_string();

        let operation = LogicalOperator::Or;
        let mut commands: Vec<Command> = Vec::new();

        let simple_command_1 = SingleCommand::new(
            "movies.movie.release_date".to_string(),
            Operator::LessThan,
            Value::new("01-01-2000".to_string(), DataType::String),
        );
        let simple_command_2 = SingleCommand::new(
            "movies.movie.genre".to_string(),
            Operator::EqualTo,
            Value::new("Comedy".to_string(), DataType::String),
        );

        commands.push(Command::SingleCommand(simple_command_1));
        commands.push(Command::SingleCommand(simple_command_2));

        let composite_command = CompositeCommand::new(operation, commands);

        let command = Command::CompositeCommand(composite_command);

        assert_eq!(
            compound_expression_to_composite_command(
                LogicalOperator::Or,
                left_expression,
                right_expression
            )?,
            command
        );

        Ok(())
    }

    #[test]
    fn test_terminal_expression_to_simple_command() -> Result<(), Error> {
        let expression = "movies.movie.revenue gt 100000".to_string();

        let simple_command = SingleCommand::new(
            "movies.movie.revenue".to_string(),
            Operator::GreaterThan,
            Value::new(100000.to_string(), DataType::Integer),
        );

        let command = Command::SingleCommand(simple_command);

        assert_eq!(terminal_expression_to_simple_command(expression)?, command);

        Ok(())
    }

    #[test]
    fn test_terminal_expression_to_simple_with_attr_as_value() -> Result<(), Error> {
        let expression = "movies.movie.revenue lt movies.movie.budget".to_string();

        let simple_command = SingleCommand::new(
            "movies.movie.revenue".to_string(),
            Operator::LessThan,
            Value::new("movies.movie.budget".to_string(), DataType::Attribute),
        );

        let command = Command::SingleCommand(simple_command);

        assert_eq!(terminal_expression_to_simple_command(expression)?, command);

        Ok(())
    }

    #[test]
    fn test_string_is_attribute() -> Result<(), Error> {

        let normal_string = "Disney".into();
        assert_eq!(string_is_attribute(normal_string)?, false);

        let attribute_string = "movies.movie.title".into();
        assert_eq!(string_is_attribute(attribute_string)?, true);

        let string_with_dot = "www.google.com.br".into();
        assert_eq!(string_is_attribute(string_with_dot)?, false);

        Ok(())
    }
}
