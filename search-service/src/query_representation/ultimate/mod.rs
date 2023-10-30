/*
    Module responsible for creating the final (SQL) representation
    of a query. It receives the command created in the intermediary representation
    and the projection coming from the initial representation.
*/

use anyhow::Error;

use crate::query_representation::intermediary::{
    get_command_attributes, simple_command::Operator, Command,
};

pub fn command_to_query(projection: Vec<String>, command: &Command) -> Result<String, Error> {
    let mut attributes_needed = projection.clone();
    attributes_needed.extend(get_command_attributes(command));

    let tables_needed = get_tables_needed(attributes_needed)?;

    let _select_query = create_select_query(projection);

    let _from_query = create_from_query(tables_needed);

    let _where_query = create_where_query(command, true)?;

    // let final_query = [select_query, from_query, where_query].join("\n");
    let final_query = "Command to query not implemented yet".to_string();

    Ok(final_query)
}

fn get_tables_needed(_attributes: Vec<String>) -> Result<Vec<String>, Error> {
    /* TODO: Given a list of attributes, return the tables needed to join all of them
    in one query (using src/relational/tableSearch) */

    let tables: Vec<String> = vec![];

    Ok(tables)
}

fn create_select_query(projection: Vec<String>) -> String {
    let mut select_query = "SELECT ".to_owned();
    let len = projection.len();

    for (idx, column) in projection.iter().enumerate() {
        select_query.push_str(column);
        if idx != len - 1 {
            select_query.push_str(", ");
        }
    }

    select_query
}

fn create_from_query(tables: Vec<String>) -> String {
    let mut from_query = "FROM ".to_owned();
    let len = tables.len();

    for (idx, table) in tables.iter().enumerate() {
        from_query.push_str(table);
        if idx != len - 1 {
            from_query.push_str(", ");
        }
    }

    from_query
}

fn create_where_query(command: &Command, initial_call: bool) -> Result<String, Error> {
    let mut where_query = "".to_owned();

    if initial_call {
        where_query.push_str("WHERE ")
        // TODO: Add JOIN filters here
    };

    match command {
        Command::CompositeCommand(composite_command) => {
            let nested_commands = &composite_command.commands;

            if !initial_call {
                where_query.push_str("(")
            }

            where_query.push_str(&create_where_query(&nested_commands[0], false)?);
            where_query.push_str(&composite_command.logical_operator.to_string());
            where_query.push_str(&create_where_query(&nested_commands[1], false)?);

            if !initial_call {
                where_query.push_str(")")
            }
        }

        Command::SingleCommand(single_command) => {
            where_query.push_str(&single_command.attribute);
            where_query.push_str(&translate_operator(&single_command.operator)?);
            where_query.push_str(&single_command.value.value);
        }
    }

    Ok(where_query)
}

fn translate_operator(operator: &Operator) -> Result<String, Error> {
    let operator_translated;

    match operator {
        Operator::EqualTo => {
            operator_translated = " = ".to_owned();
        }

        Operator::GreaterThan => {
            operator_translated = " > ".to_owned();
        }

        Operator::LessThan => {
            operator_translated = " < ".to_owned();
        }

        Operator::GreaterThanOrEqualTo => {
            operator_translated = " >= ".to_owned();
        }

        Operator::LessThanOrEqualTo => {
            operator_translated = " <= ".to_owned();
        }

        Operator::NotEqualTo => {
            operator_translated = " <> ".to_owned();
        }
    }
    Ok(operator_translated)
}

#[cfg(test)]
mod tests {

    use crate::query_representation::intermediary::Command;
    use crate::query_representation::ultimate::create_select_query;

    use crate::query_representation::intermediary::simple_command::{
        DataType, Operator, SingleCommand, Value,
    };

    use crate::query_representation::intermediary::composite_command::{
        CompositeCommand, LogicalOperator,
    };

    use crate::query_representation::ultimate::command_to_query;

    use anyhow::Error;

    use super::create_from_query;

    #[test]
    fn test_create_select_query() {
        let projection = vec!["column1".into(), "column2".into(), "column3".into()];
        let select_query = create_select_query(projection);

        assert_eq!(select_query, "SELECT column1, column2, column3");
    }

    #[test]
    fn test_create_from_query() {
        let tables = vec!["table1".into(), "table2".into(), "table3".into()];
        let from_query = create_from_query(tables);

        assert_eq!(from_query, "FROM table1, table2, table3");
    }

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

        let nested_composite = CompositeCommand::new(LogicalOperator::Or, nested_2_commands);

        nested_commands.push(Command::CompositeCommand(nested_composite));
        nested_commands.push(Command::SingleCommand(simple_command));

        let composite_command = CompositeCommand::new(LogicalOperator::And, nested_commands);

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
