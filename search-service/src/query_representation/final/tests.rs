
#[cfg(test)]
pub mod tests {

	use crate::relational::entities::{Table,ForeignKey};

	use crate::query_representation::intermediary::Command;

	use crate::query_representation::intermediary::simple_command::{
		SimpleCommand,
		Value,
		Operator,
		DataType
	};

	use crate::query_representation::intermediary::composite_command::{
		CompositeCommand,
		Operation,

	};

	use crate::query_representation::r#final::command_to_query;
	use crate::relational::table_search::TableSearch;
	use crate::relational::table_search::entities::TableSearchInfo;

    use anyhow::Error;

    pub fn clean_query(query: &String) -> Result<String,Error> {
	
		let cleaned_query = query.replace(&['\n','\t',' '],"");    	

    	Ok(cleaned_query)
    }

    pub fn simple_command_creation() -> Result<SimpleCommand,Error> {

		let simple_command = SimpleCommand::new(
			"movies.movie.runtime".to_string(),
			Operator::GreaterThan,
			Value::new(
				200.to_string(),
				DataType::Integer
			)
		);

		Ok(simple_command)    	
    }

    pub fn composite_command_creation() -> Result<CompositeCommand,Error> {

		let mut nested_commands : Vec<Command> = Vec::new();
		let mut nested_2_commands : Vec<Command> = Vec::new();

		let simple_command = SimpleCommand::new(
			"movies.movie.budget".to_string(),
			Operator::GreaterThan,
			Value::new(
				1000000.to_string(),
				DataType::Integer
			)
		);

		let nested_simple_command_1 = SimpleCommand::new(
			"movies.movie.runtime".to_string(),
			Operator::GreaterThan,
			Value::new(
				200.to_string(),
				DataType::Integer
			)
		);

		let nested_simple_command_2 = SimpleCommand::new(
			"movies.movie.revenue".to_string(),
			Operator::GreaterThan,
			Value::new(
				1000000.to_string(),
				DataType::Integer
			)
		);
		
		nested_2_commands.push(Command::SimpleCommand(nested_simple_command_1));
		nested_2_commands.push(Command::SimpleCommand(nested_simple_command_2));

		let nested_composite = CompositeCommand::new(
			Operation::Or,
			nested_2_commands
		);

		nested_commands.push(Command::CompositeCommand(nested_composite));
		nested_commands.push(Command::SimpleCommand(simple_command));

		let composite_command = CompositeCommand::new(
			Operation::And,
			nested_commands
		);

		Ok(composite_command)

    }

	#[test]
	fn test_simple_command_to_query() -> Result<(),Error> {

		let mut projection : Vec<String> = Vec::new();
		projection.push("movies.movie.title".to_string());
		projection.push("movies.movie.runtime".to_string());

		let simple_command = simple_command_creation()?;
		
		// TODO: Pass correct lists of Tables and ForeignKeys to table_search
		let tables: Vec<Table> = Vec::from([]);
		let fks: Vec<ForeignKey> = Vec::from([]);

		let tables_search_info: Vec<TableSearchInfo> = tables
        .clone()
        .into_iter()
        .map(TableSearchInfo::from)
        .collect();
		let ts = TableSearch::new(&tables_search_info, &fks);

		let command = Command::SimpleCommand(simple_command);
		let _query = command_to_query(&projection,&command,&ts)?;

		/* TODO: Uncomment the test after full implementation */
		
		// let ideal_query = "
		// 	SELECT movies.movie.title,movies.movie.runtime
		// 	FROM movies.movie
		// 	WHERE movis.movie.runtime > 200;
		// ".to_string();

		// assert_eq!(clean_query(&query)?,clean_query(&ideal_query)?);

		Ok(())
	}

	#[test]
	fn test_composite_command_to_query() -> Result<(),Error> {

		let mut projection : Vec<String> = Vec::new();
		projection.push("movies.movie.title".to_string());
		projection.push("movies.movie.revenue".to_string());
		projection.push("movies.movie.runtime".to_string());
		projection.push("movies.movie.budget".to_string());

		let composite_command = composite_command_creation()?;
		
		// TODO: Pass correct lists of Tables and ForeignKeys to table_search
		let tables: Vec<Table> = Vec::from([]);
		let fks: Vec<ForeignKey> = Vec::from([]);
		let tables_search_info: Vec<TableSearchInfo> = tables
        .clone()
        .into_iter()
        .map(TableSearchInfo::from)
        .collect();
		let ts = TableSearch::new(&tables_search_info, &fks);

		let command = Command::CompositeCommand(composite_command);
		let _query = command_to_query(&projection,&command,&ts)?;

		/* TODO: Uncomment the test after full implementation */
		// let ideal_query = "
		// 	SELECT movies.movie.title, movies.movie.revenue, movies.movie.runtime, movies.movie.release_date
		// 	FROM movies.movie
		// 	WHERE (movies.movie.revenue>1000000 OR movies.movie.runtime>200) AND movies.movie.budget > 1000000;
		// ".to_string();

		// assert_eq!(clean_query(&query)?,clean_query(&ideal_query)?);

		Ok(())
	}

}