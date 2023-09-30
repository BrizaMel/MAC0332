// Module responsible for hosting the TableSearch struct
// The struct is responsible for dealing with the feasibility
// of operations involving fields from different tables of a 
// given relational database.

use std::collections::HashMap;

use dict::{Dict, DictIface};
use petgraph::{graph::{Graph,NodeIndex}, Undirected}; // Graph used to represent foreign key links
// use union_find::QuickFindUf; // Union Find used to quickly determine if an operation is possible

use crate::relational::general::{ForeignKey, Table};

use petgraph::dot::{Dot, Config}; //Used for debugging graphs
use petgraph::algo::dijkstra;

pub struct TableSearch {
	indexes_dict: Dict::<NodeIndex>,
	table_graph: Graph::<String, String, Undirected>,
	// Is the UF really necessary?
}

impl TableSearch {
	pub fn new(tables: &Vec<Table>, foreign_keys: &Vec<ForeignKey>) -> Self {

		let mut this_graph = Graph::<String, String, Undirected>::new_undirected();
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

	pub fn pathTo(&self, origin: String, destiny: String) -> Vec<&String>{
		let origin_index = self.indexes_dict.get(&origin).unwrap();
		let destiny_index = self.indexes_dict.get(&destiny).unwrap();

		let mut nodes: Vec<&String> = Vec::new();

		let result_path = dijkstra(&self.table_graph,
			*origin_index,
			Some(*destiny_index),
			|_| 1
		);
		
		let mut oredered_nodes: Vec<_> = result_path.iter().collect();
		oredered_nodes.sort_by(|a, b| a.1.cmp(b.1));

		for entry in oredered_nodes{
			// nodes.set
			let table_identifier = self.table_graph.node_weight(*entry.0).unwrap();
			nodes.push(table_identifier);
		}

		if nodes[nodes.len()-1] != &destiny{
			nodes = Vec::new();
		}

		nodes
	}


}

#[cfg(test)]
mod tests {
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

	#[test]
	fn test_no_path(){
		let ts = TableSearch::new(&Vec::from([
			Table::new("A".to_string(),"B".to_string(),Vec::new(),Vec::new()),
			Table::new("C".to_string(),"D".to_string(),Vec::new(),
			Vec::new()),
			Table::new("AA".to_string(),"BB".to_string(),Vec::new(),Vec::new()),
			// Table::new("CA".to_string(),"DA".to_string(),Vec::new(),
			// Vec::new())
			]),
			&Vec::from([
				ForeignKey::new("A".to_string(), "B".to_string(), 
					"e".to_string(), "C".to_string(), 
					"D".to_string(), "f".to_string()),
			])); 
		let res = ts.pathTo("A.B".to_string(), "AA.BB".to_string());
		let expected: Vec<&String> = Vec::from([]);
		assert_eq!(res,expected);
	}

	#[test]
	fn test_ordered_edges(){
		let ts = TableSearch::new(&Vec::from([
			Table::new("A".to_string(),"B".to_string(),Vec::new(),Vec::new()),
			Table::new("C".to_string(),"D".to_string(),Vec::new(),
			Vec::new()),
			Table::new("AA".to_string(),"BB".to_string(),Vec::new(),Vec::new()),
			// Table::new("CA".to_string(),"DA".to_string(),Vec::new(),
			// Vec::new())
			]),
			&Vec::from([
				ForeignKey::new("A".to_string(), "B".to_string(), 
					"e".to_string(), "C".to_string(), 
					"D".to_string(), "f".to_string()),
				ForeignKey::new("C".to_string(), "D".to_string(), 
					"g".to_string(), "AA".to_string(), 
					"BB".to_string(), "h".to_string())
			])); 
		let res = ts.pathTo("A.B".to_string(), "AA.BB".to_string());
		let node1 = "A.B".to_string();
		let node2 = "C.D".to_string();
		let node3 = "AA.BB".to_string();
		let expected: Vec<&String> = Vec::from([
			&node1,
			&node2,
			&node3
		]);
		assert_eq!(res,expected);
	}

	#[test]
	fn test_inverted_edges(){
		let ts = TableSearch::new(&Vec::from([
			Table::new("A".to_string(),"B".to_string(),Vec::new(),Vec::new()),
			Table::new("C".to_string(),"D".to_string(),Vec::new(),
			Vec::new()),
			Table::new("AA".to_string(),"BB".to_string(),Vec::new(),Vec::new()),
			// Table::new("CA".to_string(),"DA".to_string(),Vec::new(),
			// Vec::new())
			]),
			&Vec::from([
				ForeignKey::new("AA".to_string(), "BB".to_string(), 
					"g".to_string(), "C".to_string(), 
					"D".to_string(), "h".to_string()),
				ForeignKey::new("A".to_string(), "B".to_string(), 
					"e".to_string(), "C".to_string(), 
					"D".to_string(), "f".to_string()),
			])); 
		let res = ts.pathTo("A.B".to_string(), "AA.BB".to_string());
		let node1 = "A.B".to_string();
		let node2 = "C.D".to_string();
		let node3 = "AA.BB".to_string();
		let expected: Vec<&String> = Vec::from([
			&node1,
			&node2,
			&node3
		]);
		assert_eq!(res,expected);
	}

}
