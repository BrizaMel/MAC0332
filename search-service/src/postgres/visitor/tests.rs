#[cfg(test)]
mod tests {

	use crate::relational::entities::{Table,ForeignKey};

	use crate::query_representation::intermediary::Command;

	use crate::query_representation::intermediary::tests::tests::{
		create_simple_command,
		create_composite_command
	};

	use crate::postgres::visitor::PostgresVisitor;

	use crate::traits::Component;

    use anyhow::Error;

   	#[test]
	fn test_visitor_architecture() -> Result<(),Error> {

    	let simple_command = create_simple_command()?;
    	let composite_command = create_composite_command()?;

		//TODO: Pass correct lists of Tables and ForeignKeys to visitor
		let tables: Vec<Table> = Vec::from(
			[Table::new("movies".to_string(), "movie".to_string(), vec![], vec![])]);
		let fks: Vec<ForeignKey> = Vec::from([]);
		let postgres_visitor = PostgresVisitor::new(&tables,&fks);

        let sc_return = Command::SimpleCommand(simple_command).accept(vec!["movies.movie.runtime".to_string(),"movies.movie.revenue".to_string()], &postgres_visitor)?;
        let cc_return = Command::CompositeCommand(composite_command).accept(vec!["movies.movie.runtime".to_string(),"movies.movie.revenue".to_string()], &postgres_visitor)?;

		assert_eq!(sc_return, "SELECT movies.movie.runtime,movies.movie.revenue\nFROM movies.movie\nWHERE movies.movie.runtime > 200;".to_string());
		assert_eq!(cc_return, "SELECT movies.movie.runtime,movies.movie.revenue\nFROM movies.movie\nWHERE movies.movie.runtime > 200ANDmovies.movie.revenue > 1000000;".to_string());

		Ok(())
	}
}