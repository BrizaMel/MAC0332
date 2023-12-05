#[cfg(test)]
pub mod tests {

    use crate::query_representation::intermediary::Command;

    use crate::query_representation::intermediary::single_command::{
        DataType, Operator, SingleCommand, Value,
    };

    use crate::query_representation::intermediary::composite_command::{
        CompositeCommand, LogicalOperator,
    };

    use crate::query_representation::initial::initial_to_command;

    use anyhow::Error;

    #[test]
    fn test_initial_to_single_command() -> Result<(), Error> {
        let filters = "movies.movie.runtime gt 200".to_string();

        let single_command = SingleCommand::new(
            "movies.movie.runtime".to_string(),
            Operator::GreaterThan,
            Value::new(200.to_string(), DataType::Integer),
        );

        let command = Command::SingleCommand(single_command);

        assert_eq!(initial_to_command(filters)?, command);

        Ok(())
    }

    #[test]
    fn test_initial_to_single_command_with_string() -> Result<(), Error> {
        let filters = "movies.movie.release_date lt 01-01-2000".to_string();

        let single_command = SingleCommand::new(
            "movies.movie.release_date".to_string(),
            Operator::LessThan,
            Value::new("01-01-2000".to_string(), DataType::String),
        );

        let command = Command::SingleCommand(single_command);

        assert_eq!(initial_to_command(filters)?, command);

        Ok(())
    }

    #[test]
    fn test_initial_to_composite_command() -> Result<(), Error> {
        let filters = "movies.movie.runtime gt 200 AND movies.movie.revenue gt 1000000".to_string();

        let operation = LogicalOperator::And;
        let mut commands: Vec<Command> = Vec::new();

        let single_command_1 = SingleCommand::new(
            "movies.movie.runtime".to_string(),
            Operator::GreaterThan,
            Value::new(200.to_string(), DataType::Integer),
        );
        let single_command_2 = SingleCommand::new(
            "movies.movie.revenue".to_string(),
            Operator::GreaterThan,
            Value::new(1000000.to_string(), DataType::Integer),
        );

        commands.push(Command::SingleCommand(single_command_1));
        commands.push(Command::SingleCommand(single_command_2));

        let composite_command = CompositeCommand::new(operation, commands);

        let command = Command::CompositeCommand(composite_command);

        assert_eq!(initial_to_command(filters)?, command);

        Ok(())
    }

    #[test]
    fn test_initial_to_nested_composite_command() -> Result<(), Error> {
        let filters = "(movies.movie.revenue gt 1000000 OR movies.movie.runtime gt 200) AND (movies.movie.runtime eq 50)".to_string();

        let nested_operation = LogicalOperator::Or;
        let mut nested_commands: Vec<Command> = Vec::new();

        let nested_single_command_1 = SingleCommand::new(
            "movies.movie.revenue".to_string(),
            Operator::GreaterThan,
            Value::new(1000000.to_string(), DataType::Integer),
        );
        let nested_single_command_2 = SingleCommand::new(
            "movies.movie.runtime".to_string(),
            Operator::GreaterThan,
            Value::new(200.to_string(), DataType::Integer),
        );

        nested_commands.push(Command::SingleCommand(nested_single_command_1));
        nested_commands.push(Command::SingleCommand(nested_single_command_2));

        let nested_composite_command = CompositeCommand::new(nested_operation, nested_commands);

        let single_command = SingleCommand::new(
            "movies.movie.runtime".to_string(),
            Operator::EqualTo,
            Value::new(50.to_string(), DataType::Integer),
        );
        let final_operation = LogicalOperator::And;
        let mut final_commands: Vec<Command> = Vec::new();

        final_commands.push(Command::CompositeCommand(nested_composite_command));
        final_commands.push(Command::SingleCommand(single_command));

        let final_nested_command = CompositeCommand::new(final_operation, final_commands);

        let command = Command::CompositeCommand(final_nested_command);

        assert_eq!(initial_to_command(filters)?, command);

        Ok(())
    }

    #[test]
    fn test_initial_to_nested_composite_command_inverted() -> Result<(), Error> {
        let filters = "(movies.movie.runtime eq 50) AND (movies.movie.revenue gt 1000000 OR movies.movie.runtime gt 200)".to_string();

        let nested_operation = LogicalOperator::Or;
        let mut nested_commands: Vec<Command> = Vec::new();

        let nested_single_command_1 = SingleCommand::new(
            "movies.movie.revenue".to_string(),
            Operator::GreaterThan,
            Value::new(1000000.to_string(), DataType::Integer),
        );
        let nested_single_command_2 = SingleCommand::new(
            "movies.movie.runtime".to_string(),
            Operator::GreaterThan,
            Value::new(200.to_string(), DataType::Integer),
        );

        nested_commands.push(Command::SingleCommand(nested_single_command_1));
        nested_commands.push(Command::SingleCommand(nested_single_command_2));

        let nested_composite_command = CompositeCommand::new(nested_operation, nested_commands);

        let single_command = SingleCommand::new(
            "movies.movie.runtime".to_string(),
            Operator::EqualTo,
            Value::new(50.to_string(), DataType::Integer),
        );
        let final_operation = LogicalOperator::And;
        let mut final_commands: Vec<Command> = Vec::new();

        final_commands.push(Command::SingleCommand(single_command));
        final_commands.push(Command::CompositeCommand(nested_composite_command));

        let final_nested_command = CompositeCommand::new(final_operation, final_commands);

        let command = Command::CompositeCommand(final_nested_command);

        assert_eq!(initial_to_command(filters)?, command);

        Ok(())
    }

    #[test]
    fn test_initial_to_super_nested_composite_command() -> Result<(), Error> {
        let filters = "(movies.movie.runtime eq 50 AND movies.movie.release_date lt 01-01-2000) AND ((movies.movie.revenue gt 1000000 OR movies.movie.runtime gt 200) OR (movies.movie.revenue eq 2000000))".to_string();

        let nested_operation_1 = LogicalOperator::And;
        let mut nested_commands_1: Vec<Command> = Vec::new();

        let nested_single_command_1 = SingleCommand::new(
            "movies.movie.runtime".to_string(),
            Operator::EqualTo,
            Value::new(50.to_string(), DataType::Integer),
        );
        let nested_single_command_2 = SingleCommand::new(
            "movies.movie.release_date".to_string(),
            Operator::LessThan,
            Value::new("01-01-2000".to_string(), DataType::String),
        );

        nested_commands_1.push(Command::SingleCommand(nested_single_command_1));
        nested_commands_1.push(Command::SingleCommand(nested_single_command_2));

        let nested_composite_command_1 =
            CompositeCommand::new(nested_operation_1, nested_commands_1);

        let nested_operation_2 = LogicalOperator::Or;
        let mut nested_commands_2: Vec<Command> = Vec::new();

        let nested_single_command_3 = SingleCommand::new(
            "movies.movie.revenue".to_string(),
            Operator::GreaterThan,
            Value::new(1000000.to_string(), DataType::Integer),
        );
        let nested_single_command_4 = SingleCommand::new(
            "movies.movie.runtime".to_string(),
            Operator::GreaterThan,
            Value::new(200.to_string(), DataType::Integer),
        );

        nested_commands_2.push(Command::SingleCommand(nested_single_command_3));
        nested_commands_2.push(Command::SingleCommand(nested_single_command_4));

        let nested_composite_command_2 =
            CompositeCommand::new(nested_operation_2, nested_commands_2);

        let single_command = SingleCommand::new(
            "movies.movie.revenue".to_string(),
            Operator::EqualTo,
            Value::new(2000000.to_string(), DataType::Integer),
        );
        let nested_operation_3 = LogicalOperator::Or;
        let mut nested_commands_3: Vec<Command> = Vec::new();

        nested_commands_3.push(Command::CompositeCommand(nested_composite_command_2));
        nested_commands_3.push(Command::SingleCommand(single_command));

        let nested_composite_command_3 =
            CompositeCommand::new(nested_operation_3, nested_commands_3);

        let final_operation = LogicalOperator::And;
        let mut final_commands: Vec<Command> = Vec::new();
        final_commands.push(Command::CompositeCommand(nested_composite_command_1));
        final_commands.push(Command::CompositeCommand(nested_composite_command_3));

        let final_nested_command = CompositeCommand::new(final_operation, final_commands);

        let command = Command::CompositeCommand(final_nested_command);

        assert_eq!(initial_to_command(filters)?, command);

        Ok(())
    }

    #[test]
    fn test_initial_to_super_with_attribute_as_value() -> Result<(), Error> {
        let filters = "movies.person.person_name eq movies.movie_cast.character_name".to_string();

        let single_command = SingleCommand::new(
            "movies.person.person_name".to_string(),
            Operator::EqualTo,
            Value::new(
                "movies.movie_cast.character_name".to_string(),
                DataType::Attribute,
            ),
        );

        let command = Command::SingleCommand(single_command);

        assert_eq!(initial_to_command(filters)?, command);

        Ok(())
    }
}
