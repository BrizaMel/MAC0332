/*
	Module responsible for creating the final (SQL) representation
	of a query. It receives the command created in the intermediary representation
	and the projection coming from the initial representation.
*/

use anyhow::Error;

use crate::query_representation::intermediary::{
	Command,
	get_command_attributes,
	composite_command::Operation, 
	simple_command::Operator
};

mod tests;

pub fn command_to_query(projection:&Vec<String>,command:&Command) -> Result<String,Error>{


	let attributes_needed = get_attributes_needed(projection,command)?;

	let tables_needed = get_tables_needed(&attributes_needed)?;

	let select_query = create_select_query(&projection)?;

	let from_query = create_from_query(&tables_needed)?;

	let where_query = create_where_query(&command, true)?;
	
	println!("{:?}",from_query);

 	let final_query = [select_query, from_query, where_query].join("\n");


	Ok(final_query)
}

fn get_attributes_needed(projection: &Vec<String>,command:&Command) -> Result<Vec<String>,Error>{
	let mut attributes = Vec::new();

	for attribute in projection.iter() {
		attributes.push(attribute.to_string());
	}

	let mut command_attributes = get_command_attributes(&command)?;
	attributes.append(&mut command_attributes);

	Ok(attributes)
}

fn get_tables_needed(_attributes: &Vec<String>) -> Result<Vec<String>,Error>{
	/* TODO: Given a list of attributes, return the tables needed to join all of them
	in one query (using src/relational/tableSearch) */

	let tables: Vec<String> = Vec::new();


	Ok(tables)

}

fn create_select_query(projection: &Vec<String>) -> Result<String,Error> {

	let mut select_query = "SELECT ".to_owned();
	let mut peekable_projection = projection.iter().peekable();
	while let Some(project) = peekable_projection.next(){
		select_query.push_str(&project);
		if !peekable_projection.peek().is_none() {
			select_query.push_str(&",".to_string());
		}
	}

	Ok(select_query)

}

fn create_from_query(tables:&Vec<String>) -> Result<String,Error> {

	let mut from_query = "FROM ".to_owned();
	let mut peekable_tables = tables.iter().peekable();
	while let Some(table) = peekable_tables.next(){
		from_query.push_str(&table);
		if !peekable_tables.peek().is_none() {
			from_query.push_str(&",".to_string());
		}
	}

	Ok(from_query)

}

fn create_where_query(command: &Command, initial_call: bool) -> Result<String,Error> {

	let mut where_query = "".to_owned();
	
	if initial_call {
		where_query.push_str("WHERE ")
		// TODO: Add JOIN filters here
	};

	match command {

		Command::CompositeCommand(_) => {
			let Command::CompositeCommand(ref composite_command) = command else {  panic!("Wrong Command type");};
			let nested_commands = &composite_command.commands;
			
			if !initial_call {where_query.push_str(&"(".to_string())}
			where_query.push_str(&create_where_query(&nested_commands[0], false)?);
			where_query.push_str(&translate_operation(&composite_command.operation)?);
			where_query.push_str(&create_where_query(&nested_commands[1], false)?);
			if !initial_call {where_query.push_str(&")".to_string())}
		}
	
		Command::SimpleCommand(_) => {
			let Command::SimpleCommand(ref simple_command) = command else {  panic!("Wrong Command type");};
			where_query.push_str(&simple_command.attribute);
			where_query.push_str(&translate_operator(&simple_command.operator)?);
			where_query.push_str(&simple_command.value.value);
		}
		
	}

	Ok(where_query)

}

fn translate_operation(operation: &Operation) -> Result<String,Error> {

	let operation_translated;

	match operation {
		
		Operation::And => {
			operation_translated = "AND".to_owned();
		}

		Operation::Or => {
			operation_translated = "OR".to_owned();
		}
	}
	Ok(operation_translated)
}

fn translate_operator(operator: &Operator) -> Result<String,Error> {

	let operator_translated;

	match operator {
		
		Operator::Equal => {
			operator_translated = " = ".to_owned();
		}

		Operator::GreaterThan => {
			operator_translated = " > ".to_owned();
		}
	}
	Ok(operator_translated)
}