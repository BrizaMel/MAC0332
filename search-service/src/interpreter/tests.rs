#[cfg(test)]
mod tests {

use anyhow::Error;

use crate::interpreter::{
        Component,
		SimpleCommand,
		simple_command::DataType,
		simple_command::Operator,
		simple_command::Value,
		CompositeCommand,
		composite_command::LogicalOperator,
        postgres_visitor::PostgresVisitor,
	};

    fn create_simple_command() -> Result<SimpleCommand,Error> {
        
        let attribute =  "movies.movie.runtime".to_string();
		let value = Value::new(200.to_string(),DataType::Integer);
		let operator = Operator::GreaterThan;
    	let simple_command = SimpleCommand::new(attribute, operator, value);

    	Ok(simple_command)
    }

    fn create_composite_command() -> Result<CompositeCommand,Error> {
		let operation = LogicalOperator::And;
		let command_1 = SimpleCommand::new("movies.movie.runtime".to_string(),Operator::GreaterThan,Value::new(200.to_string(),DataType::Integer));
		let command_2 = SimpleCommand::new("movies.movie.revenue".to_string(),Operator::GreaterThan,Value::new(1000000.to_string(),DataType::Integer));

    	let composite_command = CompositeCommand::new(operation, command_1, command_2);

    	Ok(composite_command)
    }

	#[test]
	fn test_simple_command_creation() -> Result<(),Error> {

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
	fn test_composite_command_creation() -> Result<(),Error> {

		/*
			{
				operation: "AND",
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
        assert!(composite_command.commands.len() == 2);

		Ok(())
	}

    #[test]
	fn test_visitor_arquitecture() -> Result<(),Error> {

    	let simple_command = create_simple_command()?;
    	let composite_command = create_composite_command()?;

        let sc_return = simple_command.accept(vec!["projection".to_string()], &PostgresVisitor)?;
        let cc_return = composite_command.accept(vec!["projection".to_string()], &PostgresVisitor)?;

		assert_eq!(sc_return, "SimpleCommand interpretation not implemented for Postgres yet".to_string());
		assert_eq!(cc_return, "CompositeCommand interpretation not implemented for Postgres yet".to_string());

		Ok(())
	}
}
