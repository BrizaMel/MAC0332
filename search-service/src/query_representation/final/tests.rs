#[cfg(test)]
mod tests {

    use crate::query_representation::intermediary::Command;

    use crate::query_representation::intermediary::simple_command::{
        DataType, Operator, SingleCommand, Value,
    };

    use crate::query_representation::intermediary::composite_command::{
        CompositeCommand, Operation,
    };

    use crate::query_representation::r#final::command_to_query;

    use anyhow::Error;

    #[test]
    fn test_command_to_query_simple_command() -> Result<(), Error> {
        let mut projection: Vec<String> = Vec::new();
        projection.push("movies.movie.title".to_string());
        projection.push("movies.movie.runtime".to_string());

        let simple_command = SingleCommand::new(
            "movies.movie.runtime".to_string(),
            Operator::GreaterThan,
            Value::new(200.to_string(), DataType::Integer),
        );

        let command = Command::SingleCommand(simple_command);
        let _query = command_to_query(projection, &command)?;

        /* TODO: Uncomment the test after full implementation */

        // assert_eq!(query,"
        // 	SELECT movies.movie.title,movies.movie.runtime
        // 	FROM movies.movie
        // 	WHERE movis.movie.runtime > 200;
        // 	".to_string());

        Ok(())
    }

    #[test]
    fn test_intermediary_to_final_composite_command() -> Result<(), Error> {
        let mut projection: Vec<String> = Vec::new();
        projection.push("movies.movie.title".to_string());
        projection.push("movies.movie.revenue".to_string());
        projection.push("movies.movie.runtime".to_string());
        projection.push("movies.movie.budget".to_string());

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

        let nested_composite = CompositeCommand::new(Operation::Or, nested_2_commands);

        nested_commands.push(Command::CompositeCommand(nested_composite));
        nested_commands.push(Command::SingleCommand(simple_command));

        let composite_command = CompositeCommand::new(Operation::And, nested_commands);

        let command = Command::CompositeCommand(composite_command);
        let _query = command_to_query(projection, &command)?;

        /* TODO: Uncomment the test after full implementation */

        // assert_eq!(query,"
        // 	SELECT movies.movie.title, movies.movie.revenue, movies.movie.runtime, movies.movie.release_date
        // 	FROM movies.movie
        // 	WHERE (movies.movie.revenue>1000000 OR movies.movie.runtime>200) AND movies.movie.budget > 1000000
        // ".to_string());

        Ok(())
    }
}
