/*
	Module responsible for creating the final (SQL) representation
	of a query. It receives the command created in the intermediary representation
	and the projection coming from the initial representation.
*/

use anyhow::Error;

use crate::query_representation::intermediary::{
	Command,
	get_command_attributes
};

mod tests;

pub fn command_to_query(projection:&Vec<String>,command:&Command) -> Result<String,Error>{


	let attributes_needed = get_attributes_needed(projection,command)?;

	let tables_needed = get_tables_needed(&attributes_needed)?;

	let _from_query = create_from_query(&tables_needed)?;
	
	// println!("{:?}",from_query);

	/* TODO: function that creates the where clause*/
	/* TODO: Concatenate the select,from and where strings */

	let _select_query = create_select_query(&projection)?;
 	let final_query = "Command to query not implemented yet".to_string();


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
