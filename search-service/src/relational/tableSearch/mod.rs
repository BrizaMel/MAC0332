// Module responsible for hosting the TableSearch struct
// The struct is responsible for dealing with the feasibility
// of operations involving fields from different tables of a
// given relational database.

use dict::{Dict, DictIface};
use petgraph::{
    graph::{Graph, NodeIndex},
    Undirected,
}; // Graph used to represent foreign key links
   // use union_find::QuickFindUf; // Union Find used to quickly determine if an operation is possible

use crate::relational::general::{ForeignKey, Table};

use petgraph::algo::dijkstra;
use petgraph::dot::Dot; //Used for debugging graphs

pub struct TableSearch {
    // maps table identifiers (in the format schema_name.table_name) to their corresponding node indices in the graph
    indexes_dict: Dict<NodeIndex>,
    // undirected graph: nodes represent tables and edges represent foreign keys
    table_graph: Graph<String, String, Undirected>,
    // Is the UF really necessary?
}

impl TableSearch {
    pub fn new(tables: &Vec<Table>, foreign_keys: &Vec<ForeignKey>) -> Self {
        let mut this_graph = Graph::<String, String, Undirected>::new_undirected();
        let mut this_dict = Dict::<NodeIndex>::new();

        // add all tables as nodes in the graph
        for table in tables {
            let table_identifier = format!("{}.{}", table.schema, table.name);
            let graph_index = this_graph.add_node(table_identifier.clone());

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

        println!(
            "{:?}",
            Dot::with_config(&this_graph, &[Config::EdgeNoLabel])
        );

        Self {
            indexes_dict: this_dict,
            table_graph: this_graph,
        }
    }

    pub fn path_to(&self, origin: String, destiny: String) -> Vec<&String> {
        let origin_index = self.indexes_dict.get(&origin).unwrap();
        let destiny_index = self.indexes_dict.get(&destiny).unwrap();

        let mut nodes: Vec<&String> = Vec::new();

        let result_path = dijkstra(
            &self.table_graph,
            *origin_index,
            Some(*destiny_index),
            |_| 1,
        );

        let mut oredered_nodes: Vec<_> = result_path.iter().collect();
        oredered_nodes.sort_by(|a, b| a.1.cmp(b.1));

        for entry in oredered_nodes {
            // nodes.set
            let table_identifier = self.table_graph.node_weight(*entry.0).unwrap();
            nodes.push(table_identifier);
        }

        if nodes[nodes.len() - 1] != &destiny {
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
        TableSearch::new(&Vec::new(), &Vec::new());
    }

    #[test]
    fn test_creates_only_tables() {
        TableSearch::new(
            &Vec::from([Table::new(
                "A".to_string(),
                "B".to_string(),
                Vec::new(),
                Vec::new(),
            )]),
            &Vec::new(),
        );
    }

    #[test]
    fn test_creates_tables_and_fks() {
        TableSearch::new(
            &Vec::from([
                Table::new("A".to_string(), "B".to_string(), Vec::new(), Vec::new()),
                Table::new("C".to_string(), "D".to_string(), Vec::new(), Vec::new()),
            ]),
            &Vec::from([ForeignKey::new(
                "A".to_string(),
                "B".to_string(),
                "e".to_string(),
                "C".to_string(),
                "D".to_string(),
                "f".to_string(),
            )]),
        );
    }

    #[test]
    fn test_no_path() {
        let ts = TableSearch::new(
            &Vec::from([
                Table::new("A".to_string(), "B".to_string(), Vec::new(), Vec::new()),
                Table::new("C".to_string(), "D".to_string(), Vec::new(), Vec::new()),
                Table::new("AA".to_string(), "BB".to_string(), Vec::new(), Vec::new()),
                // Table::new("CA".to_string(),"DA".to_string(),Vec::new(),
                // Vec::new())
            ]),
            &Vec::from([ForeignKey::new(
                "A".to_string(),
                "B".to_string(),
                "e".to_string(),
                "C".to_string(),
                "D".to_string(),
                "f".to_string(),
            )]),
        );
        let res = ts.path_to("A.B".to_string(), "AA.BB".to_string());
        let expected: Vec<&String> = Vec::from([]);
        assert_eq!(res, expected);
    }

    #[test]
    fn test_ordered_edges() {
        let ts = TableSearch::new(
            &Vec::from([
                Table::new("A".to_string(), "B".to_string(), Vec::new(), Vec::new()),
                Table::new("C".to_string(), "D".to_string(), Vec::new(), Vec::new()),
                Table::new("AA".to_string(), "BB".to_string(), Vec::new(), Vec::new()),
                // Table::new("CA".to_string(),"DA".to_string(),Vec::new(),
                // Vec::new())
            ]),
            &Vec::from([
                ForeignKey::new(
                    "A".to_string(),
                    "B".to_string(),
                    "e".to_string(),
                    "C".to_string(),
                    "D".to_string(),
                    "f".to_string(),
                ),
                ForeignKey::new(
                    "C".to_string(),
                    "D".to_string(),
                    "g".to_string(),
                    "AA".to_string(),
                    "BB".to_string(),
                    "h".to_string(),
                ),
            ]),
        );
        let res = ts.path_to("A.B".to_string(), "AA.BB".to_string());
        let node1 = "A.B".to_string();
        let node2 = "C.D".to_string();
        let node3 = "AA.BB".to_string();
        let expected: Vec<&String> = Vec::from([&node1, &node2, &node3]);
        assert_eq!(res, expected);
    }

    #[test]
    fn test_inverted_edges() {
        let ts = TableSearch::new(
            &Vec::from([
                Table::new("A".to_string(), "B".to_string(), Vec::new(), Vec::new()),
                Table::new("C".to_string(), "D".to_string(), Vec::new(), Vec::new()),
                Table::new("AA".to_string(), "BB".to_string(), Vec::new(), Vec::new()),
                // Table::new("CA".to_string(),"DA".to_string(),Vec::new(),
                // Vec::new())
            ]),
            &Vec::from([
                ForeignKey::new(
                    "AA".to_string(),
                    "BB".to_string(),
                    "g".to_string(),
                    "C".to_string(),
                    "D".to_string(),
                    "h".to_string(),
                ),
                ForeignKey::new(
                    "A".to_string(),
                    "B".to_string(),
                    "e".to_string(),
                    "C".to_string(),
                    "D".to_string(),
                    "f".to_string(),
                ),
            ]),
        );
        let res = ts.path_to("A.B".to_string(), "AA.BB".to_string());
        let node1 = "A.B".to_string();
        let node2 = "C.D".to_string();
        let node3 = "AA.BB".to_string();
        let expected: Vec<&String> = Vec::from([&node1, &node2, &node3]);
        assert_eq!(res, expected);
    }
}
