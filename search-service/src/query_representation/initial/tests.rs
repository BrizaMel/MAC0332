#[cfg(test)]
pub mod tests {


	use crate::query_representation::intermediary::Command;

	use crate::query_representation::intermediary::simple_command::{
		SimpleCommand,
		Operator,
		Value,
		DataType
	};

	use crate::query_representation::intermediary::composite_command::{
		CompositeCommand,
		Operation,
	};

	use crate::query_representation::initial::initial_to_command;

	use anyhow::Error;

	#[test]
	fn test_initial_to_simple_command() -> Result<(),Error> {

		let _initial = serde_json::json!({
			"projection": "[]",
  			"filters": "movies.movie.runtime gt 200"
		});

		let simple_command = SimpleCommand::new(
			"movies.movie.runtime".to_string(),
			Operator::GreaterThan,
			Value::new(
				200.to_string(),
				DataType::Integer
			)
		);

		let _command = Command::SimpleCommand(simple_command);

		/* TODO: Uncomment the test after full implementation */
		assert_eq!(initial_to_command(_initial)?,_command);

		Ok(())
	}

	#[test]
	fn test_initial_to_composite_command() -> Result<(),Error> {

		let _initial = serde_json::json!({
			"projection": "[]",
  			"filters": "movies.movie.runtime gt 200 AND movies.movie.revenue gt 1000000"
		});

		let operation = Operation::And;
		let mut commands : Vec<Command> = Vec::new();

		let simple_command_1 = SimpleCommand::new("movies.movie.runtime".to_string(),Operator::GreaterThan,Value::new(200.to_string(),DataType::Integer));
		let simple_command_2 = SimpleCommand::new("movies.movie.revenue".to_string(),Operator::GreaterThan,Value::new(1000000.to_string(),DataType::Integer));
		
		commands.push(Command::SimpleCommand(simple_command_1));
		commands.push(Command::SimpleCommand(simple_command_2));

    	let composite_command = CompositeCommand::new(operation,commands);

    	let _command = Command::CompositeCommand(composite_command);
		
		/* TODO: Uncomment the test after full implementation */
		assert_eq!(initial_to_command(_initial)?,_command);


		Ok(())
	}

	#[test]
	fn test_initial_to_nested_composite_command() -> Result<(),Error> {

		let _initial = serde_json::json!({
			"projection": "[]",
  			"filters": "(movies.movie.revenue gt 1000000 OR movies.movie.runtime gt 200) AND (movies.movie.runtime eq 50)"
		});


		let nested_operation = Operation::Or;
		let mut nested_commands : Vec<Command> = Vec::new();

		let nested_simple_command_1 = SimpleCommand::new("movies.movie.revenue".to_string(),Operator::GreaterThan,Value::new(1000000.to_string(),DataType::Integer));
		let nested_simple_command_2 = SimpleCommand::new("movies.movie.runtime".to_string(),Operator::GreaterThan,Value::new(200.to_string(),DataType::Integer));
		
		nested_commands.push(Command::SimpleCommand(nested_simple_command_1));
		nested_commands.push(Command::SimpleCommand(nested_simple_command_2));

		let nested_composite_command = CompositeCommand::new(nested_operation,nested_commands);

		let simple_command = SimpleCommand::new("movies.movie.runtime".to_string(),Operator::EqualTo,Value::new(50.to_string(),DataType::Integer));
		let final_operation = Operation::And;
		let mut final_commands : Vec<Command> = Vec::new();

		final_commands.push(Command::CompositeCommand(nested_composite_command));
		final_commands.push(Command::SimpleCommand(simple_command));

    	let final_nested_command = CompositeCommand::new(final_operation,final_commands);

    	let _command = Command::CompositeCommand(final_nested_command );

		/* TODO: Uncomment the test after full implementation */
		assert_eq!(initial_to_command(_initial)?,_command);

		Ok(())
	}

	#[test]
	fn test_initial_to_nested_composite_command_inverted() -> Result<(),Error> {

		let _initial = serde_json::json!({
			"projection": "[]",
  			"filters": "(movies.movie.runtime eq 50) AND (movies.movie.revenue gt 1000000 OR movies.movie.runtime gt 200)"
		});


		let nested_operation = Operation::Or;
		let mut nested_commands : Vec<Command> = Vec::new();

		let nested_simple_command_1 = SimpleCommand::new("movies.movie.revenue".to_string(),Operator::GreaterThan,Value::new(1000000.to_string(),DataType::Integer));
		let nested_simple_command_2 = SimpleCommand::new("movies.movie.runtime".to_string(),Operator::GreaterThan,Value::new(200.to_string(),DataType::Integer));
		
		nested_commands.push(Command::SimpleCommand(nested_simple_command_1));
		nested_commands.push(Command::SimpleCommand(nested_simple_command_2));

		let nested_composite_command = CompositeCommand::new(nested_operation,nested_commands);

		let simple_command = SimpleCommand::new("movies.movie.runtime".to_string(),Operator::EqualTo,Value::new(50.to_string(),DataType::Integer));
		let final_operation = Operation::And;
		let mut final_commands : Vec<Command> = Vec::new();

		final_commands.push(Command::SimpleCommand(simple_command));
		final_commands.push(Command::CompositeCommand(nested_composite_command));

    	let final_nested_command = CompositeCommand::new(final_operation,final_commands);

    	let _command = Command::CompositeCommand(final_nested_command );

		/* TODO: Uncomment the test after full implementation */
		assert_eq!(initial_to_command(_initial)?,_command);

		Ok(())
	}
}