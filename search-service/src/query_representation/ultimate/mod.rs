/*
    Module responsible for creating the final (SQL) representation
    of a query. It receives the command created in the intermediary representation
    and the projection coming from the initial representation.
*/

use anyhow::Error;

pub mod test_utils;

use crate::{
    query_representation::intermediary::{
        get_command_attributes, simple_command::Operator, Command,
    },
    relational::table_search::TableSearch,
};

pub fn command_to_query(
    projection: Vec<String>,
    command: &Command,
    table_search: &TableSearch,
) -> Result<String, Error> {
    let mut attributes_needed = projection.clone();
    attributes_needed.extend(get_command_attributes(command));

    let (tables_needed, atributes_pairs_for_join) =
        table_search.get_join_requirements(&attributes_needed);

    let select_query = create_select_query(projection);

    let from_query = create_from_query(tables_needed);

    let where_query = create_where_query(command, true, &atributes_pairs_for_join)?;

    let mut final_query = [select_query, from_query, where_query].join("\n");

    final_query.push_str(";");

    Ok(final_query)
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

fn create_where_query(
    command: &Command,
    initial_call: bool,
    join_atribute_pairs: &Vec<String>,
) -> Result<String, Error> {
    let mut where_query = "".to_owned();

    if initial_call {
        where_query.push_str("WHERE ");

        for pair in join_atribute_pairs {
            let atributes: Vec<&str> = pair.split(":").collect();

            let section = format!("{} = {} AND ", atributes[0], atributes[1]);

            where_query.push_str(&section);
        }
    };

    match command {
        Command::CompositeCommand(composite_command) => {
            let nested_commands = &composite_command.commands;

            if !initial_call {
                where_query.push_str("(")
            }

            let logical_operator = format!(" {} ", composite_command.logical_operator.to_string());
            where_query.push_str(&create_where_query(&nested_commands[0], false, &vec![])?);
            where_query.push_str(&logical_operator);
            where_query.push_str(&create_where_query(&nested_commands[1], false, &vec![])?);

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
    use crate::relational::entities::ForeignKey;
    use crate::relational::table_search::entities::TableSearchInfo;
    use crate::relational::table_search::TableSearch;

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

        let tables: Vec<TableSearchInfo> = vec![TableSearchInfo {
            schema: "movies".into(),
            name: "movie".into(),
        }];
        let fks: Vec<ForeignKey> = vec![];
        let ts = TableSearch::new(tables, fks);

        let query = command_to_query(projection, &command, &ts)?;

        assert_eq!(
            query,
            format!(
                "{}\n{}\n{}",
                "SELECT movies.movie.title, movies.movie.runtime",
                "FROM movies.movie",
                "WHERE movies.movie.runtime > 200;"
            )
        );

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
        let mut nested_commands_2: Vec<Command> = Vec::new();

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

        nested_commands_2.push(Command::SingleCommand(nested_simple_command_1));
        nested_commands_2.push(Command::SingleCommand(nested_simple_command_2));

        let nested_composite = CompositeCommand::new(LogicalOperator::Or, nested_commands_2);

        nested_commands.push(Command::CompositeCommand(nested_composite));
        nested_commands.push(Command::SingleCommand(simple_command));

        let composite_command = CompositeCommand::new(LogicalOperator::And, nested_commands);

        let command = Command::CompositeCommand(composite_command);

        let tables: Vec<TableSearchInfo> = vec![TableSearchInfo {
            schema: "movies".into(),
            name: "movie".into()
        }];
        let fks: Vec<ForeignKey> = vec![];
        let ts = TableSearch::new(tables, fks);

        let query = command_to_query(projection, &command, &ts)?;

        assert_eq!(
            query, 
            format!("{}\n{}\n{}", 
            "SELECT movies.movie.title, movies.movie.revenue, movies.movie.runtime, movies.movie.budget", 
            "FROM movies.movie", 
            "WHERE (movies.movie.runtime > 200 OR movies.movie.revenue > 1000000) AND movies.movie.budget > 1000000;"
        ));

        Ok(())
    }

    #[test]
    fn test_intermediary_to_final_composite_command_2() -> Result<(), Error> {
        let mut projection: Vec<String> = Vec::new();
        projection.push("movies.movie.movie_id".to_string());
        projection.push("movies.movie.title".to_string());

        let simple_command = SingleCommand::new(
            "movies.country.country_name".to_string(),
            Operator::EqualTo,
            Value::new("Brazil".into(), DataType::String),
        );

        let command = Command::SingleCommand(simple_command);

        let tables: Vec<TableSearchInfo> = vec![TableSearchInfo {
            schema: "movies".into(),
            name: "movie".into(),
        }, 
        TableSearchInfo {
            schema: "movies".into(),
            name: "production_country".into(),
        },
        TableSearchInfo {
            schema: "movies".into(), 
            name: "country".into(),
        }];

        let fks: Vec<ForeignKey> = vec![ForeignKey {
            schema_name: "movies".into(),
            table_name: "movie".into(),
            attribute_name: "movie_id".into(),
            schema_name_foreign: "movies".into(),
            table_name_foreign: "production_country".into(),
            attribute_name_foreign: "movie_id".into(),
        }, ForeignKey {
            schema_name: "movies".into(),
            table_name: "production_country".into(),
            attribute_name: "country_id".into(),
            schema_name_foreign: "movies".into(),
            table_name_foreign: "country".into(),
            attribute_name_foreign: "country_id".into(),
        }];
        let ts = TableSearch::new(tables, fks);

        let query = command_to_query(projection, &command, &ts)?;

        assert_eq!(query, format!(
            "{}\n{}\n{}", 
            "SELECT movies.movie.movie_id, movies.movie.title", 
            "FROM movies.country, movies.movie, movies.production_country",
            "WHERE movies.movie.movie_id = movies.production_country.movie_id AND movies.production_country.country_id = movies.country.country_id AND movies.country.country_name = Brazil;"
        ));

        Ok(())
    }
}
