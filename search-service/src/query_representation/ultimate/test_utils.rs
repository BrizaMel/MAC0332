use crate::query_representation::intermediary::composite_command::{
    CompositeCommand, LogicalOperator,
};
use crate::query_representation::intermediary::simple_command::{
    DataType, Operator, SingleCommand, Value,
};
use crate::query_representation::intermediary::Command;

use anyhow::Error;

pub fn simple_command_creation() -> Result<SingleCommand, Error> {
    let simple_command = SingleCommand::new(
        "movies.movie.runtime".to_string(),
        Operator::GreaterThan,
        Value::new(200.to_string(), DataType::Integer),
    );

    Ok(simple_command)
}

pub fn composite_command_creation() -> Result<CompositeCommand, Error> {
    let mut nested_commands: Vec<Command> = Vec::new();
    let mut nested_2_commands: Vec<Command> = Vec::new();

    let simple_command = SingleCommand::new(
        "movies.movie.budget".to_string(),
        Operator::GreaterThan,
        Value::new(1000000.to_string(), DataType::Integer),
    );

    let nested_simple_command_1 = SingleCommand::new(
        "movies.movie.runtime".to_string(),
        Operator::GreaterThan,
        Value::new(200.to_string(), DataType::Integer),
    );

    let nested_simple_command_2 = SingleCommand::new(
        "movies.movie.revenue".to_string(),
        Operator::GreaterThan,
        Value::new(1000000.to_string(), DataType::Integer),
    );

    nested_2_commands.push(Command::SingleCommand(nested_simple_command_1));
    nested_2_commands.push(Command::SingleCommand(nested_simple_command_2));

    let nested_composite = CompositeCommand::new(LogicalOperator::Or, nested_2_commands);

    nested_commands.push(Command::CompositeCommand(nested_composite));
    nested_commands.push(Command::SingleCommand(simple_command));

    let composite_command = CompositeCommand::new(LogicalOperator::And, nested_commands);

    Ok(composite_command)
}
