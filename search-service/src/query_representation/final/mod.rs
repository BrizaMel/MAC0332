/*
    Module responsible for creating the final (SQL) representation
    of a query. It receives the command created in the intermediary representation
    and the projection coming from the initial representation.
*/

use anyhow::Error;

use crate::query_representation::intermediary::{
    composite_command::Operation, get_command_attributes, simple_command::Operator, Command,
};

mod tests;

pub fn command_to_query(projection: &Vec<String>, command: &Command) -> Result<String, Error> {
    let attributes_needed = get_attributes_needed(projection, command)?;

    let _tables_needed = get_tables_needed(&attributes_needed)?;

    // Passing this so the function does not panic
    let _from_query = create_from_query(&vec!["fake table".to_string()])?;

    println!("{:?}", from_query);

    let final_query = [select_query, from_query, where_query].join("\n");

    Ok(final_query)
}

fn get_attributes_needed(
    projection: &Vec<String>,
    command: &Command,
) -> Result<Vec<String>, Error> {
    let mut attributes = Vec::new();

    for attribute in projection.iter() {
        attributes.push(attribute.to_string());
    }

    let mut command_attributes = get_command_attributes(&command)?;
    attributes.append(&mut command_attributes);

    attributes.sort();
    attributes.dedup();

    Ok(attributes)
}

fn get_tables_needed(_attributes: &Vec<String>) -> Result<Vec<String>, Error> {
    /* TODO: Given a list of attributes, return the tables needed to join all of them
    in one query (using src/relational/tableSearch) */

    let tables: Vec<String> = Vec::new();

    Ok(tables)
}

fn create_select_query(projection: &Vec<String>) -> Result<String, Error> {
    let mut select_query = "SELECT ".to_owned();
    let mut peekable_projection = projection.iter().peekable();
    while let Some(project) = peekable_projection.next() {
        select_query.push_str(&project);
        if !peekable_projection.peek().is_none() {
            select_query.push_str(&",".to_string());
        }
    }

    if projection.len() == 0 {
        select_query = "SELECT *".to_string()
    }

    Ok(select_query)
}

fn create_from_query(tables: &Vec<String>) -> Result<String, Error> {
    let mut from_query = "FROM ".to_owned();
    let mut peekable_tables = tables.iter().peekable();
    while let Some(table) = peekable_tables.next() {
        from_query.push_str(&table);
        if !peekable_tables.peek().is_none() {
            from_query.push_str(&",".to_string());
        }
    }

    if tables.len() == 0 {
        panic!("Empty table to create the FROM clause")
    }

    Ok(from_query)
}

// Private functions tests //

#[cfg(test)]
mod private_tests {

    use super::*;

    use crate::query_representation::r#final::tests::tests::{
        clean_query, composite_command_creation, simple_command_creation,
    };

    #[test]
    fn test_get_attributes_needed_with_simple_command() -> Result<(), Error> {
        let simple_command = Command::SimpleCommand(simple_command_creation()?);
        let mut projection: Vec<String> = Vec::new();

        let mut expected_attributes_needed: Vec<String> = vec!["movies.movie.runtime".to_string()];
        assert_eq!(
            get_attributes_needed(&projection, &simple_command)?,
            expected_attributes_needed
        );

        projection.push("movies.movie.title".to_string());
        projection.push("fake attribute".to_string());

        // Result is order alphabetically
        expected_attributes_needed = vec![
            "fake attribute".to_string(),
            "movies.movie.runtime".to_string(),
            "movies.movie.title".to_string(),
        ];

        assert_eq!(
            get_attributes_needed(&projection, &simple_command)?,
            expected_attributes_needed
        );

        Ok(())
    }

    #[test]
    fn test_get_attributes_needed_with_composite_command() -> Result<(), Error> {
        let composite_command = Command::CompositeCommand(composite_command_creation()?);
        let mut projection: Vec<String> = Vec::new();

        let mut expected_attributes_needed: Vec<String> = vec![
            "movies.movie.budget".to_string(),
            "movies.movie.revenue".to_string(),
            "movies.movie.runtime".to_string(),
        ];

        assert_eq!(
            get_attributes_needed(&projection, &composite_command)?,
            expected_attributes_needed
        );

        projection.push("movies.movie.title".to_string());
        projection.push("fake attribute".to_string());

        // Result is order alphabetically
        expected_attributes_needed = vec![
            "fake attribute".to_string(),
            "movies.movie.budget".to_string(),
            "movies.movie.revenue".to_string(),
            "movies.movie.runtime".to_string(),
            "movies.movie.title".to_string(),
        ];

        assert_eq!(
            get_attributes_needed(&projection, &composite_command)?,
            expected_attributes_needed
        );

        Ok(())
    }

    #[test]
    fn test_get_tables_needed() -> Result<(), Error> {
        /* TODO: Uncomment the test after full implementation */

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

    #[test]
    fn test_select_clause_creation() -> Result<(), Error> {
        let mut attribute_list: Vec<String>;
        let mut expected_query;

        attribute_list = vec![];
        expected_query = "SELECT *".to_string();

        assert_eq!(
            clean_query(&create_select_query(&attribute_list)?)?,
            clean_query(&expected_query)?
        );

        attribute_list = vec!["movies.movie.title".to_string()];

        expected_query = "SELECT movies.movie.title".to_string();

        assert_eq!(
            clean_query(&create_select_query(&attribute_list)?)?,
            clean_query(&expected_query)?
        );

        attribute_list = vec![
            "movies.movie.title".to_string(),
            "movies.movie_languages.movie_id".to_string(),
            "movies.language.language_name".to_string(),
        ];

        expected_query = "
			SELECT movies.movie.title,
				movies.movie_languages.movie_id,
				movies.language.language_name"
            .to_string();

        assert_eq!(
            clean_query(&create_select_query(&attribute_list)?)?,
            clean_query(&expected_query)?
        );

        attribute_list = vec![
            "movies.movie.title".to_string(),
            "movies.movie_languages.movie_id".to_string(),
            "movies.language.language_name".to_string(),
        ];

        expected_query = "
			SELECT movies.movie.title,
				movies.movie_languages.movie_id,
				movies.language.language_name"
            .to_string();

        assert_eq!(
            clean_query(&create_select_query(&attribute_list)?)?,
            clean_query(&expected_query)?
        );

        Ok(())
    }

    #[test]
    fn test_from_clause_creation() -> Result<(), Error> {
        let mut table_list: Vec<String>;
        let mut expected_query: String;

        table_list = vec![];

        let empty_from_not_allowed = std::panic::catch_unwind(|| create_from_query(&table_list));
        assert!(empty_from_not_allowed.is_err());

        table_list = vec!["movies.movie".to_string()];

        expected_query = "
			FROM movies.movie
		"
        .to_string();

        assert_eq!(
            clean_query(&create_from_query(&table_list)?)?,
            clean_query(&expected_query)?
        );

        table_list = vec![
            "movies.language".to_string(),
            "movies.movie".to_string(),
            "movies.movie_languages".to_string(),
        ];

        expected_query = "
			FROM movies.language,
			movies.movie,
			movies.movie_languages
		"
        .to_string();

        assert_eq!(
            clean_query(&create_from_query(&table_list)?)?,
            clean_query(&expected_query)?
        );

        table_list = vec![
            "movies.country".to_string(),
            "movies.language".to_string(),
            "movies.movie".to_string(),
            "movies.movie_languages".to_string(),
            "movies.production_country".to_string(),
        ];

        expected_query = "
			FROM movies.country,
			movies.language,
			movies.movie,
			movies.movie_languages,
			movies.production_country
		"
        .to_string();

        assert_eq!(
            clean_query(&create_from_query(&table_list)?)?,
            clean_query(&expected_query)?
        );

        table_list = vec![
            "movies.country".to_string(),
            "movies.department".to_string(),
            "movies.movie".to_string(),
            "movies.movie_crew".to_string(),
            "movies.production_country".to_string(),
        ];

        expected_query = "
			FROM movies.country,
			movies.department,
			movies.movie,
			movies.movie_crew,
			movies.production_country
		"
        .to_string();

        assert_eq!(
            clean_query(&create_from_query(&table_list)?)?,
            clean_query(&expected_query)?
        );

        Ok(())
    }
}
