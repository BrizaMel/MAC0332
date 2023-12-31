#[cfg(test)]
pub mod tests {

    use crate::query_representation::intermediary::{get_command_attributes, Command};

    use crate::query_representation::intermediary::single_command::{
        DataType, Operator, SingleCommand, Value,
    };

    use crate::query_representation::intermediary::composite_command::{
        CompositeCommand, LogicalOperator,
    };

    use anyhow::Error;

    pub fn create_simple_command() -> Result<SingleCommand, Error> {
        let value = Value::new(200.to_string(), DataType::Integer);
        let operator = Operator::GreaterThan;
        let simple_command =
            SingleCommand::new("movies.movie.runtime".to_string(), operator, value);

        Ok(simple_command)
    }

    pub fn create_composite_command() -> Result<CompositeCommand, Error> {
        let logical_operator = LogicalOperator::And;
        let mut commands: Vec<Command> = Vec::new();

        let simple_command_1 = SingleCommand::new(
            "movies.movie.runtime".to_string(),
            Operator::GreaterThan,
            Value::new(200.to_string(), DataType::Integer),
        );
        let simple_command_2 = SingleCommand::new(
            "movies.movie.revenue".to_string(),
            Operator::GreaterThan,
            Value::new(1000000.to_string(), DataType::Integer),
        );

        commands.push(Command::SingleCommand(simple_command_1));
        commands.push(Command::SingleCommand(simple_command_2));

        let composite_command = CompositeCommand::new(logical_operator, commands);

        Ok(composite_command)
    }

    fn create_nested_composite_command() -> Result<CompositeCommand, Error> {
        let nested_operator = LogicalOperator::Or;
        let mut nested_commands: Vec<Command> = Vec::new();

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

        nested_commands.push(Command::SingleCommand(nested_simple_command_1));
        nested_commands.push(Command::SingleCommand(nested_simple_command_2));

        let nested_composite_command = CompositeCommand::new(nested_operator, nested_commands);

        let simple_command = SingleCommand::new(
            "movies.movie.runtime".to_string(),
            Operator::EqualTo,
            Value::new(50.to_string(), DataType::Integer),
        );
        let final_operator = LogicalOperator::And;
        let mut final_commands: Vec<Command> = Vec::new();

        final_commands.push(Command::CompositeCommand(nested_composite_command));
        final_commands.push(Command::SingleCommand(simple_command));

        let final_composite_command = CompositeCommand::new(final_operator, final_commands);

        Ok(final_composite_command)
    }

    #[test]
    fn test_simple_command_creation() -> Result<(), Error> {
        /*
            {
                attribute: "movies.movie.runtime",
                operator: "gt",
                value: 200
            }
        */

        let simple_command = create_simple_command()?;

        assert_eq!(simple_command.attribute, "movies.movie.runtime".to_string());
        assert_eq!(simple_command.operator, Operator::GreaterThan);
        assert_eq!(simple_command.value.value, 200.to_string());
        assert_eq!(simple_command.value.data_type, DataType::Integer);
        Ok(())
    }

    #[test]
    fn test_composite_command_creation() -> Result<(), Error> {
        /*
            {
                LogicalOperator: "AND",
                command: [
                    {
                        attribute: "movies.movie.runtime",
                        operator: "gt",
                        value: 200
                    },
                    {
                        attribute: "movies.movie.revenue",
                        operator: "gt",
                        value: 1000000
                    }
                ]
            }
        */

        let composite_command = create_composite_command()?;

        assert_eq!(composite_command.logical_operator, LogicalOperator::And);

        let Command::SingleCommand(ref first_command) = composite_command.commands[0] else {  panic!("Wrong Command type in index 0");};

        assert_eq!(first_command.attribute, "movies.movie.runtime".to_string());
        assert_eq!(first_command.value.data_type, DataType::Integer);

        let Command::SingleCommand(ref second_command) = composite_command.commands[1] else {  panic!("Wrong Command type in index 1");};

        assert_eq!(second_command.attribute, "movies.movie.revenue".to_string());
        assert_eq!(second_command.value.value, 1000000.to_string());

        Ok(())
    }

    #[test]
    fn test_composite_command_recursive() -> Result<(), Error> {
        /*
            {
                LogicalOperator: "AND",
                command: [
                    {
                        LogicalOperator: "OR",
                        command: [
                            {
                                attribute: "movies.movie.runtime",
                                operator: "gt",
                                value: 200
                            },
                            {
                                attribute: "movies.movie.revenue",
                                operator: "gt",
                                value: 1000000
                            }
                        ]
                    },
                    {
                        attribute: "movies.movie.runtime",
                        operator: "eq",
                        value: 50
                    }
                ]
            }
        */

        let final_composite_command = create_nested_composite_command()?;

        assert_eq!(
            final_composite_command.logical_operator,
            LogicalOperator::And
        );

        let Command::SingleCommand(ref checking_simple_command) = final_composite_command.commands[1] else {  panic!("Wrong Command type in index 1");};

        assert_eq!(
            checking_simple_command.attribute,
            "movies.movie.runtime".to_string()
        );
        assert_eq!(checking_simple_command.value.value, 50.to_string());

        let Command::CompositeCommand(ref checking_composite_command) = final_composite_command.commands[0] else {  panic!("Wrong Command type in index 0");};

        assert_eq!(
            checking_composite_command.logical_operator,
            LogicalOperator::Or
        );

        let Command::SingleCommand(ref checking_nested_simple_command_1) = checking_composite_command.commands[0] else {  panic!("Wrong Command type in nested index 0");};
        assert_eq!(
            checking_nested_simple_command_1.attribute,
            "movies.movie.runtime".to_string()
        );
        assert_eq!(
            checking_nested_simple_command_1.value.value,
            200.to_string()
        );

        let Command::SingleCommand(ref checking_nested_simple_command_2) = checking_composite_command.commands[1] else {  panic!("Wrong Command type in nested index 1");};

        assert_eq!(
            checking_nested_simple_command_2.attribute,
            "movies.movie.revenue".to_string()
        );
        assert_eq!(
            checking_nested_simple_command_2.value.value,
            1000000.to_string()
        );

        Ok(())
    }

    #[test]
    fn test_get_command_attributes() -> Result<(), Error> {
        let simple_command = Command::SingleCommand(create_simple_command()?);
        let composite_command = Command::CompositeCommand(create_composite_command()?);
        let nested_composite_command =
            Command::CompositeCommand(create_nested_composite_command()?);

        let simple_command_attributes = get_command_attributes(&simple_command);
        assert_eq!(
            simple_command_attributes,
            vec!["movies.movie.runtime".to_string()]
        );

        let composite_command_attributes = get_command_attributes(&composite_command);
        assert_eq!(
            composite_command_attributes,
            vec![
                "movies.movie.revenue".to_string(),
                "movies.movie.runtime".to_string()
            ]
        );

        let nested_composite_command_attributes = get_command_attributes(&nested_composite_command);
        assert_eq!(
            nested_composite_command_attributes,
            vec![
                "movies.movie.revenue".to_string(),
                "movies.movie.runtime".to_string()
            ]
        );


        // Testing attribute in the value field
        let simple_command_with_attribute = Command::SingleCommand(
            SingleCommand::new(
                "movies.movie.revenue".to_string(),
                Operator::GreaterThan,
                Value::new("movies.movie.budget".to_string(), DataType::Attribute)
            )
        );
        let simple_command_with_attribute_attributes = get_command_attributes(&simple_command_with_attribute);
        assert_eq!(
            simple_command_with_attribute_attributes,
            vec![
                "movies.movie.budget".to_string(),
                "movies.movie.revenue".to_string()
            ]
        );


        Ok(())
    }
}
