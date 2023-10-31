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

            where_query.push_str(&create_where_query(&nested_commands[0], false, &vec![])?);
            where_query.push_str(&composite_command.logical_operator.to_string());
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
    use crate::query_representation::ultimate::test_utils::clean_query;
    use crate::relational::entities::ForeignKey;
    use crate::relational::table_search::entities::TableSearchInfo;
    use crate::relational::table_search::TableSearch;

    use anyhow::Error;

    use super::create_from_query;
    use super::test_utils::{composite_command_creation, simple_command_creation};

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

        // TODO: Pass correct lists of Tables and ForeignKeys to table_search
        let tables: Vec<TableSearchInfo> = vec![];
        let fks: Vec<ForeignKey> = vec![];
        let ts = TableSearch::new(tables, fks);

        let _query = command_to_query(projection, &command, &ts)?;

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

        // TODO: Pass correct lists of Tables and ForeignKeys to table_search
        let tables: Vec<TableSearchInfo> = vec![];
        let fks: Vec<ForeignKey> = vec![];
        let ts = TableSearch::new(tables, fks);

        let _query = command_to_query(projection, &command, &ts)?;

        /* TODO: Uncomment the test after full implementation */

        // assert_eq!(query,"
        // 	SELECT movies.movie.title, movies.movie.revenue, movies.movie.runtime, movies.movie.release_date
        // 	FROM movies.movie
        // 	WHERE (movies.movie.revenue>1000000 OR movies.movie.runtime>200) AND movies.movie.budget > 1000000
        // ".to_string());

        Ok(())
    }

    #[test]
    fn test_simple_command_to_query() -> Result<(), Error> {
        let mut projection: Vec<String> = Vec::new();
        projection.push("movies.movie.title".to_string());
        projection.push("movies.movie.runtime".to_string());

        let simple_command = simple_command_creation()?;

        let tables = vec![TableSearchInfo::new(
            "movies".to_string(),
            "movie".to_string(),
        )];
        let fks: Vec<ForeignKey> = vec![];
        let ts = TableSearch::new(tables, fks);

        let command = Command::SingleCommand(simple_command);
        let query = command_to_query(projection, &command, &ts)?;

        let ideal_query = "
			SELECT movies.movie.title,movies.movie.runtime
			FROM movies.movie
			WHERE movies.movie.runtime > 200;
		"
        .to_string();

        assert_eq!(clean_query(&query)?, clean_query(&ideal_query)?);

        Ok(())
    }

    #[test]
    fn test_composite_command_to_query() -> Result<(), Error> {
        let mut projection: Vec<String> = Vec::new();
        projection.push("movies.movie.title".to_string());
        projection.push("movies.movie.revenue".to_string());
        projection.push("movies.movie.runtime".to_string());
        projection.push("movies.movie.budget".to_string());

        let composite_command = composite_command_creation()?;

        let tables = vec![TableSearchInfo::new(
            "movies".to_string(),
            "movie".to_string(),
        )];
        let fks: Vec<ForeignKey> = vec![];
        let ts = TableSearch::new(tables, fks);

        let command = Command::CompositeCommand(composite_command);
        let query = command_to_query(projection, &command, &ts)?;

        let ideal_query = "
			SELECT movies.movie.title, movies.movie.revenue, movies.movie.runtime, movies.movie.budget
			FROM movies.movie
			WHERE (movies.movie.runtime>200 OR movies.movie.revenue>1000000) AND movies.movie.budget > 1000000;
		"
        .to_string();

        assert_eq!(clean_query(&query)?, clean_query(&ideal_query)?);

        Ok(())
    }

    #[test]
    fn test_get_tables_needed() -> Result<(), Error> {
        /* TODO: Uncomment the test after full implementation */
        /* TODO: Refactor these tests to use table_search.get_join_requirements */

        // let mut attribute_list : Vec<String> = vec![];
        // let mut expected_tables_needed : Vec<String> = vec![];
        // assert_eq!(get_tables_needed(&attribute_list)?,expected_tables_needed);
        // attribute_list = vec![
        // 	"movies.movie.title".to_string()
        // ];
        // expected_tables_needed = vec![
        // 	"movies.movie".to_string()
        // ];
        // assert_eq!(get_tables_needed(&attribute_list)?,expected_tables_needed);
        // attribute_list = vec![
        // 	"movies.movie.title".to_string(),
        // 	"movies.movie_languages.movie_id".to_string(),
        // 	"movies.language.language_name".to_string(),
        // ];
        // expected_tables_needed = vec![
        // 	"movies.language".to_string(),
        // 	"movies.movie".to_string(),
        // 	"movies.movie_languages".to_string(),
        // ];
        // assert_eq!(get_tables_needed(&attribute_list)?,expected_tables_needed);
        // attribute_list = vec![
        // 	"movies.language.language_name".to_string(),
        // 	"movies.country.country_name".to_string()
        // ];
        // expected_tables_needed = vec![
        // 	"movies.country".to_string(),
        // 	"movies.language".to_string(),
        // 	"movies.movie".to_string(),
        // 	"movies.movie_languages".to_string(),
        // 	"movies.production_country".to_string()
        // ];
        // assert_eq!(get_tables_needed(&attribute_list)?,expected_tables_needed);
        // attribute_list = vec![
        // 	"movies.country.country_name".to_string(),
        // 	"movies.department.department_name".to_string(),
        // ];
        // expected_tables_needed = vec![
        // 	"movies.country".to_string(),
        // 	"movies.department".to_string(),
        // 	"movies.movie".to_string(),
        // 	"movies.movie_crew".to_string(),
        // 	"movies.production_country".to_string()
        // ];
        // assert_eq!(get_tables_needed(&attribute_list)?,expected_tables_needed);
        Ok(())
    }
}
