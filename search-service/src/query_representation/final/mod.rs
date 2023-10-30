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

pub fn command_to_query(projection: Vec<String>, command: &Command) -> Result<String, Error> {
    let mut attributes_needed = projection.clone();
    attributes_needed.extend(get_command_attributes(command));

    let tables_needed = get_tables_needed(attributes_needed)?;

    let _select_query = create_select_query(projection);

    let _from_query = create_from_query(tables_needed);

    let _where_query = create_where_query(command, true)?;

    // let final_query = [select_query, from_query, where_query].join("\n");
    let final_query = "Command to query not implemented yet".to_string();

    Ok(final_query)
}

fn get_tables_needed(_attributes: Vec<String>) -> Result<Vec<String>, Error> {
    /* TODO: Given a list of attributes, return the tables needed to join all of them
    in one query (using src/relational/tableSearch) */

    let tables: Vec<String> = vec![];

    Ok(tables)
}

fn create_select_query(projection: Vec<String>) -> String {
    let mut select_query = "SELECT ".to_owned();
    let len = projection.len();

    for (idx, column) in projection.iter().enumerate() {
        select_query.push_str(column);
        if idx != len {
            select_query.push_str(",");
        }
    }

    select_query
}

fn create_from_query(tables: Vec<String>) -> String {
    let mut from_query = "FROM ".to_owned();
    let len = tables.len();

    for (idx, table) in tables.iter().enumerate() {
        from_query.push_str(table);
        if idx != len {
            from_query.push_str(",");
        }
    }

    from_query
}

fn create_where_query(command: &Command, initial_call: bool) -> Result<String, Error> {
    let mut where_query = "".to_owned();

    if initial_call {
        where_query.push_str("WHERE ")
        // TODO: Add JOIN filters here
    };

    match command {
        Command::CompositeCommand(_) => {
            let Command::CompositeCommand(ref composite_command) = command else {  panic!("Wrong Command type");};
            let nested_commands = &composite_command.commands;

            if !initial_call {
                where_query.push_str(&"(".to_string())
            }
            where_query.push_str(&create_where_query(&nested_commands[0], false)?);
            where_query.push_str(&translate_operation(&composite_command.operation)?);
            where_query.push_str(&create_where_query(&nested_commands[1], false)?);
            if !initial_call {
                where_query.push_str(&")".to_string())
            }
        }

        Command::SingleCommand(_) => {
            let Command::SingleCommand(ref simple_command) = command else {  panic!("Wrong Command type");};
            where_query.push_str(&simple_command.attribute);
            where_query.push_str(&translate_operator(&simple_command.operator)?);
            where_query.push_str(&simple_command.value.value);
        }
    }

    Ok(where_query)
}

fn translate_operation(operation: &Operation) -> Result<String, Error> {
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
