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
        let mut this_graph = Graph::<String, String, Undirected>::new_undirected();
        let mut this_dict = Dict::<NodeIndex>::new();

		let mut this_graph = Graph::<String, String>::new();
		let mut this_dict  = Dict::<NodeIndex>::new();

            this_dict.add(table_identifier, graph_index);
        }

        for fk in foreign_keys {
            let ForeignKey {
                schema_name,
                table_name,
                attribute_name,
                schema_name_foreign,
                table_name_foreign,
                attribute_name_foreign,
            } = fk;

            // we will connect origin table to foreign table
            let origin_table = format!("{}.{}", schema_name, table_name);
            let foreign_table = format!("{}.{}", schema_name_foreign, table_name_foreign);

            // the weight of the edge is the foreign key
            let weight = format!("{}:{}", attribute_name, attribute_name_foreign);

            let origin_index = this_dict.get(&origin_table).unwrap();
            let foreign_index = this_dict.get(&foreign_table).unwrap();

            this_graph.add_edge(*origin_index, *foreign_index, weight);
        }

		println!("{:?}", Dot::with_config(&this_graph, &[Config::EdgeNoLabel]));

        Self {
            indexes_dict: this_dict,
            table_graph: this_graph,
        }
    }

}
