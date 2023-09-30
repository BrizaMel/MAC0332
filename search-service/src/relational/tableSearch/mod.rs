// Module responsible for hosting the TableSearch struct
// The struct is responsible for dealing with the feasibility
// of operations involving fields from different tables of a 
// given relational database.

use dict::{Dict, DictIface};
use petgraph::graph::{Graph,NodeIndex}; // Graph used to represent foreign key links
// use union_find::QuickFindUf; // Union Find used to quickly determine if an operation is possible

use crate::relational::general::{ForeignKey, Table};

use petgraph::dot::{Dot, Config}; //Used for debugging graphs

pub struct TableSearch {
	indexes_dict: Dict::<NodeIndex>,
	table_graph: Graph::<String, String>,
	// Is the UF really necessary?
}

impl TableSearch {
	pub fn new(tables: &Vec<Table>, foreign_keys: &Vec<ForeignKey>) -> Self {

		let mut this_graph = Graph::<String, String>::new();
		let mut this_dict  = Dict::<NodeIndex>::new();

		for table in tables{
			let table_identifier  = Clone::clone(&table.schema) + "." + &table.name;
			let graph_index = this_graph.add_node(Clone::clone(&table_identifier));

			this_dict.add(table_identifier, graph_index);
		}

		for fk in foreign_keys{
			let origin_identifier = Clone::clone(&fk.schema_name) + "." + &fk.table_name;
			let foreign_identifier = Clone::clone(&fk.schema_name_foreign) + "." + &fk.table_name_foreign;

			let weight = Clone::clone(&fk.attribute_name) + ":" + &fk.attribute_name_foreign;

			let origin_index = this_dict.get(&origin_identifier).unwrap();
			let foreign_index = this_dict.get(&foreign_identifier).unwrap();

			this_graph.add_edge(*origin_index, *foreign_index, weight);
		}

		println!("{:?}", Dot::new(&this_graph));

		Self { 
			indexes_dict: this_dict,
			table_graph: this_graph
		}
	}

}

#[cfg(test)]mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

	// fn initialize_table() -> TableSearch {
	// 	TableSearch::new(); 
	// }

    #[test]
    fn test_creates_empty_table() {
		TableSearch::new(&Vec::new(),&Vec::new()); 
    }

	#[test]
	fn test_creates_only_tables(){
		TableSearch::new(&Vec::from(
			[Table::new("A".to_string(),"B".to_string(),Vec::new(),Vec::new())]),
			&Vec::new()); 
	}

	#[test]
	fn test_creates_tables_and_fks(){
		TableSearch::new(&Vec::from([
			Table::new("A".to_string(),"B".to_string(),Vec::new(),Vec::new()),
			Table::new("C".to_string(),"D".to_string(),Vec::new(),Vec::new())]),
			&Vec::from([
				ForeignKey::new("A".to_string(), "B".to_string(), 
					"e".to_string(), "C".to_string(), 
					"D".to_string(), "f".to_string())
			])); 
	}

}
