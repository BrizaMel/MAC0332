#[cfg(test)]
mod tests {

	use crate::relational::general::{Table,ForeignKey};

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
		let tables: Vec<Table> = Vec::from([]);
		let fks: Vec<ForeignKey> = Vec::from([]);
		let postgres_visitor = PostgresVisitor::new(&tables,&fks);

        let sc_return = Command::SimpleCommand(simple_command).accept(vec!["projection".to_string()], &postgres_visitor)?;
        let cc_return = Command::CompositeCommand(composite_command).accept(vec!["projection".to_string()], &postgres_visitor)?;

		assert_eq!(sc_return, "Command to query not implemented yet".to_string());
		assert_eq!(cc_return, "Command to query not implemented yet".to_string());

		Ok(())
	}
}