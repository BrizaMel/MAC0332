#[cfg(test)]
mod tests {

	use crate::query_representation::intermediary::{
		DataType,
		Command,
		Operator,
		Operation,
		Value,
		SimpleCommand,
		CompositeCommand
	};

    use anyhow::Error;

	#[test]
	fn test_simple_command_creation() -> Result<(),Error> {

		/*
			{
				property: "movies.movie.runtime",
				operator: "gt",
				value: 200
			}
		*/

		let value = Value::new(200.to_string(),DataType::Integer);
		let operator = Operator::GreaterThan;
    	let simple_command = SimpleCommand::new("movies.movie.runtime".to_string(),operator,value);

		assert_eq!(simple_command.property,"movies.movie.runtime".to_string());
		assert_eq!(simple_command.operator,Operator::GreaterThan);
		assert_eq!(simple_command.value.value,200.to_string());
		assert_eq!(simple_command.value.data_type,DataType::Integer);
		Ok(())
	}

	#[test]
	fn test_composite_command_creation() -> Result<(),Error> {

		/*
			{
				operation: "AND",
				command: [
					{
						property: "movies.movie.runtime",
						operator: "gt",
						value: 200
					},
					{
						property: "movies.movie.revenue",
						operator: "gt",
						value: 1000000
					}
				]
			}
		*/

		let operation = Operation::And;
		let mut commands : Vec<Command> = Vec::new();

		let simple_command_1 = SimpleCommand::new("movies.movie.runtime".to_string(),Operator::GreaterThan,Value::new(200.to_string(),DataType::Integer));
		let simple_command_2 = SimpleCommand::new("movies.movie.revenue".to_string(),Operator::GreaterThan,Value::new(1000000.to_string(),DataType::Integer));
		
		commands.push(Command::SimpleCommand(simple_command_1));
		commands.push(Command::SimpleCommand(simple_command_2));

    	let composite_command = CompositeCommand::new(operation,commands);

		assert_eq!(composite_command.operation,Operation::And);

		let Command::SimpleCommand(ref first_command) = composite_command.command[0] else {  panic!("Wrong Command type in index 0");};
		
		assert_eq!(first_command.property,"movies.movie.runtime".to_string());
		assert_eq!(first_command.value.data_type,DataType::Integer);
		
		let Command::SimpleCommand(ref second_command) = composite_command.command[1] else {  panic!("Wrong Command type in index 1");};
		
		assert_eq!(second_command.property,"movies.movie.revenue".to_string());
		assert_eq!(second_command.value.value,1000000.to_string());

		Ok(())
	}

	#[test]
	fn test_composite_command_recursive() -> Result<(),Error> {

		/*
			{
				operation: "AND",
				command: [
					{
						operation: "OR",
						command: [
							{
								property: "movies.movie.runtime",
								operator: "gt",
								value: 200
							},
							{
								property: "movies.movie.revenue",
								operator: "gt",
								value: 1000000
							}
						]
					},
					{
						property: "movies.movie.runtime",
						operator: "eq",
						value: 50
					}
				]
			}
		*/


		let nested_operation = Operation::Or;
		let mut nested_commands : Vec<Command> = Vec::new();

		let nested_simple_command_1 = SimpleCommand::new("movies.movie.runtime".to_string(),Operator::GreaterThan,Value::new(200.to_string(),DataType::Integer));
		let nested_simple_command_2 = SimpleCommand::new("movies.movie.revenue".to_string(),Operator::GreaterThan,Value::new(1000000.to_string(),DataType::Integer));
		
		nested_commands.push(Command::SimpleCommand(nested_simple_command_1));
		nested_commands.push(Command::SimpleCommand(nested_simple_command_2));

		let nested_composite_command = CompositeCommand::new(nested_operation,nested_commands);

		let simple_command = SimpleCommand::new("movies.movie.runtime".to_string(),Operator::Equal,Value::new(50.to_string(),DataType::Integer));
		let final_operation = Operation::And;
		let mut final_commands : Vec<Command> = Vec::new();

		final_commands.push(Command::CompositeCommand(nested_composite_command));
		final_commands.push(Command::SimpleCommand(simple_command));

    	let final_composite_command = CompositeCommand::new(final_operation,final_commands);

		assert_eq!(final_composite_command.operation,Operation::And);

		let Command::SimpleCommand(ref checking_simple_command) = final_composite_command.command[1] else {  panic!("Wrong Command type in index 1");};

		assert_eq!(checking_simple_command.property,"movies.movie.runtime".to_string());
		assert_eq!(checking_simple_command.value.value,50.to_string());

		let Command::CompositeCommand(ref checking_composite_command) = final_composite_command.command[0] else {  panic!("Wrong Command type in index 0");};

		assert_eq!(checking_composite_command.operation,Operation::Or);

		let Command::SimpleCommand(ref checking_nested_simple_command_1) = checking_composite_command.command[0] else {  panic!("Wrong Command type in nested index 0");};
		assert_eq!(checking_nested_simple_command_1.property,"movies.movie.runtime".to_string());
		assert_eq!(checking_nested_simple_command_1.value.value,200.to_string());
		
		let Command::SimpleCommand(ref checking_nested_simple_command_2) = checking_composite_command.command[1] else {  panic!("Wrong Command type in nested index 1");};

		assert_eq!(checking_nested_simple_command_2.property,"movies.movie.revenue".to_string());
		assert_eq!(checking_nested_simple_command_2.value.value,1000000.to_string());

		Ok(())
	}
}

