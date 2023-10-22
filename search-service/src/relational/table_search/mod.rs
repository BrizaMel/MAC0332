// Module responsible for hosting the TableSearch struct
// The struct is responsible for dealing with the feasibility
// of operations involving fields from different tables of a
// given relational database.

use anyhow::{anyhow, Result};
use petgraph::{
    algo::dijkstra,
    dot::Dot,
    graph::{Graph, NodeIndex},
    Undirected,
};
use std::collections::HashMap;

use crate::relational::general::{ForeignKey, Table};

pub struct TableSearch {
    // maps table identifiers (in the format schema_name.table_name) to their corresponding node indices in the graph
    table_identifier_to_node_index: HashMap<String, NodeIndex>,
    // undirected graph: nodes represent tables and edges represent foreign keys
    table_search_graph: Graph<String, String, Undirected>,
    // Is the UF really necessary?
}

impl TableSearch {
    pub fn new(tables: &[Table], foreign_keys: &[ForeignKey]) -> Self {
        let mut table_search_graph = Graph::<String, String, Undirected>::new_undirected();
        let table_identifier_to_node_index = tables
            .iter()
            .map(|t| {
                let table_identifier = format!("{}.{}", t.schema, t.name);
                let graph_index = table_search_graph.add_node(table_identifier.clone());

                (table_identifier, graph_index)
            })
            .collect::<HashMap<String, NodeIndex>>();

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

            let origin_index = table_identifier_to_node_index.get(&origin_table).unwrap();
            let foreign_index = table_identifier_to_node_index.get(&foreign_table).unwrap();

            table_search_graph.add_edge(*origin_index, *foreign_index, weight);
        }

        println!("{:?}", Dot::new(&table_search_graph));

        Self {
            table_identifier_to_node_index,
            table_search_graph,
        }
    }

    pub fn path_to(&self, origin: String, destiny: String) -> Result<(Vec<String>, Vec<String>)> {
        let origin_index = self
            .table_identifier_to_node_index
            .get(&origin)
            .ok_or_else(|| anyhow!("origin table not found in graph"))?;
        let destiny_index = self
            .table_identifier_to_node_index
            .get(&destiny)
            .ok_or_else(|| anyhow!("destiny table not found in graph"))?;

        let (mut tables, mut ordered_edges) = self.get_paths(*origin_index, Some(*destiny_index));

        if let Some(last_table) = tables.last() {
            if *last_table != destiny {
                tables = vec![];
                ordered_edges = vec![];
            }
        }

        Ok((tables, ordered_edges))
    }

    pub fn joinable_tables(&self, origin: String) -> Result<(Vec<String>, Vec<String>)> {
        let origin_index = self
            .table_identifier_to_node_index
            .get(&origin)
            .ok_or_else(|| anyhow!("origin table not found in graph"))?;
        let (tables, ordered_edges) = self.get_paths(*origin_index, None);
        Ok((tables, ordered_edges))
    }

    fn get_paths(
        &self,
        origin_index: NodeIndex,
        destiny_index: Option<NodeIndex>,
    ) -> (Vec<String>, Vec<String>) {
        let node_to_path_cost =
            dijkstra(&self.table_search_graph, origin_index, destiny_index, |_| 1);

        let mut ordered_nodes: Vec<(NodeIndex, i32)> = node_to_path_cost.into_iter().collect();
        ordered_nodes.sort_by(|a, b| a.1.cmp(&b.1));

        let mut tables = vec![];
        let mut ordered_edges = vec![];

        let num_of_nodes = match destiny_index {
            None => ordered_nodes.len() - 1,
            Some(_) => ordered_nodes.len(),
        };

        for i in 0..ordered_nodes.len() {
            let (node_index, _) = ordered_nodes[i];
            let table_identifier = self.table_search_graph.node_weight(node_index).unwrap();
            tables.push(table_identifier.clone());

            if i > 0 && i < num_of_nodes {
                let (previous_node_index, _) = ordered_nodes[i - 1];
                let edge = self
                    .table_search_graph
                    .find_edge(previous_node_index, node_index)
                    .unwrap();

                ordered_edges.push(self.table_search_graph.edge_weight(edge).unwrap().clone())
            }
        }

        (tables, ordered_edges)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_creates_tables_and_fks() {
        TableSearch::new(
            &[
                Table::new("A".to_string(), "B".to_string(), Vec::new(), Vec::new()),
                Table::new("C".to_string(), "D".to_string(), Vec::new(), Vec::new()),
            ],
            &[ForeignKey::new(
                "A".to_string(),
                "B".to_string(),
                "e".to_string(),
                "C".to_string(),
                "D".to_string(),
                "f".to_string(),
            )],
        );
    }

    #[test]
    fn test_no_path() -> Result<()> {
        let ts = TableSearch::new(
            &[
                Table::new("A".to_string(), "B".to_string(), Vec::new(), Vec::new()),
                Table::new("C".to_string(), "D".to_string(), Vec::new(), Vec::new()),
                Table::new("AA".to_string(), "BB".to_string(), Vec::new(), Vec::new()),
                // Table::new("CA".to_string(),"DA".to_string(),Vec::new(),
                // Vec::new())
            ],
            &[ForeignKey::new(
                "A".to_string(),
                "B".to_string(),
                "e".to_string(),
                "C".to_string(),
                "D".to_string(),
                "f".to_string(),
            )],
        );
        let res = ts.path_to("A.B".to_string(), "AA.BB".to_string())?;
        let expected_nodes: Vec<String> = vec![];
        let expected_edges: Vec<String> = vec![];
        let expected = (expected_nodes, expected_edges);
        assert_eq!(res, expected);

        Ok(())
    }

    #[test]
    fn test_ordered_edges() -> Result<()> {
        let ts = TableSearch::new(
            &[
                Table::new("A".to_string(), "B".to_string(), Vec::new(), Vec::new()),
                Table::new("C".to_string(), "D".to_string(), Vec::new(), Vec::new()),
                Table::new("AA".to_string(), "BB".to_string(), Vec::new(), Vec::new()),
                // Table::new("CA".to_string(),"DA".to_string(),Vec::new(),
                // Vec::new())
            ],
            &[
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
            ],
        );
        let res = ts.path_to("A.B".to_string(), "AA.BB".to_string())?;
        let node1 = "A.B".to_string();
        let node2 = "C.D".to_string();
        let node3 = "AA.BB".to_string();
        let expected_nodes: Vec<String> = Vec::from([node1, node2, node3]);
        let edge1 = "e:f".to_string();
        let edge2 = "g:h".to_string();
        let expected_edges: Vec<String> = Vec::from([edge1, edge2]);
        let expected = (expected_nodes, expected_edges);
        assert_eq!(res, expected);

        Ok(())
    }

    #[test]
    fn test_inverted_edges() -> Result<()> {
        let ts = TableSearch::new(
            &[
                Table::new("A".to_string(), "B".to_string(), Vec::new(), Vec::new()),
                Table::new("C".to_string(), "D".to_string(), Vec::new(), Vec::new()),
                Table::new("AA".to_string(), "BB".to_string(), Vec::new(), Vec::new()),
                // Table::new("CA".to_string(),"DA".to_string(),Vec::new(),
                // Vec::new())
            ],
            &[
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
            ],
        );
        let res = ts.path_to("A.B".to_string(), "AA.BB".to_string())?;
        let node1 = "A.B".to_string();
        let node2 = "C.D".to_string();
        let node3 = "AA.BB".to_string();
        let expected_nodes: Vec<String> = Vec::from([node1, node2, node3]);
        let edge1 = "e:f".to_string();
        let edge2 = "g:h".to_string();
        let expected_edges: Vec<String> = Vec::from([edge1, edge2]);
        let expected = (expected_nodes, expected_edges);
        assert_eq!(res, expected);

        Ok(())
    }

    #[test]
    fn test_all_joins() -> Result<()> {
        let ts = TableSearch::new(
            &[
                Table::new("A".to_string(), "B".to_string(), Vec::new(), Vec::new()),
                Table::new("C".to_string(), "D".to_string(), Vec::new(), Vec::new()),
                Table::new("AA".to_string(), "BB".to_string(), Vec::new(), Vec::new()),
                Table::new("CC".to_string(), "DD".to_string(), Vec::new(), Vec::new()),
            ],
            &[
                ForeignKey::new(
                    "A".to_string(),
                    "B".to_string(),
                    "e".to_string(),
                    "C".to_string(),
                    "D".to_string(),
                    "f".to_string(),
                ),
                ForeignKey::new(
                    "A".to_string(),
                    "B".to_string(),
                    "g".to_string(),
                    "AA".to_string(),
                    "BB".to_string(),
                    "h".to_string(),
                ),
            ],
        );
        let res = ts.joinable_tables("A.B".to_string())?;
        let node1 = "A.B".to_string();
        let node2 = "AA.BB".to_string();
        let node3 = "C.D".to_string();
        let expected_nodes: Vec<String> = Vec::from([node1, node2, node3]);
        let edge1 = "e:f".to_string();
        let edge2 = "g:h".to_string();
        let expected_edges: Vec<String> = Vec::from([edge1, edge2]);
        assert!(
            res.0.iter().all(|item| expected_nodes.contains(item))
                && res.1.iter().all(|item| expected_edges.contains(item))
        );

        Ok(())
    }

    #[test]
    fn test_no_joins() -> Result<()> {
        let ts = TableSearch::new(
            &[
                Table::new("A".to_string(), "B".to_string(), Vec::new(), Vec::new()),
                Table::new("C".to_string(), "D".to_string(), Vec::new(), Vec::new()),
                Table::new("AA".to_string(), "BB".to_string(), Vec::new(), Vec::new()),
                Table::new("CC".to_string(), "DD".to_string(), Vec::new(), Vec::new()),
            ],
            &[
                ForeignKey::new(
                    "A".to_string(),
                    "B".to_string(),
                    "e".to_string(),
                    "C".to_string(),
                    "D".to_string(),
                    "f".to_string(),
                ),
                ForeignKey::new(
                    "A".to_string(),
                    "B".to_string(),
                    "g".to_string(),
                    "AA".to_string(),
                    "BB".to_string(),
                    "h".to_string(),
                ),
            ],
        );
        let res = ts.joinable_tables("CC.DD".to_string())?;
        let node = "CC.DD".to_string();
        let expected_nodes: Vec<String> = Vec::from([node]);
        let expected_edges: Vec<String> = Vec::from([]);
        let expected = (expected_nodes, expected_edges);
        assert_eq!(res, expected);

        Ok(())
    }
}