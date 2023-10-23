// Module responsible for hosting the TableSearch struct
// The struct is responsible for dealing with the feasibility
// of operations involving fields from different tables of a
// given relational database.
pub mod entities;

use anyhow::{anyhow, Result};
use petgraph::{
    algo::dijkstra,
    dot::Dot,
    graph::{Graph, NodeIndex},
    Undirected,
};
use std::collections::HashMap;

use crate::relational::entities::ForeignKey;

use self::entities::TableSearchInfo;

pub struct TableSearch {
    // maps table identifiers (in the format schema_name.table_name) to their corresponding node indices in the graph
    table_identifier_to_node_index: HashMap<String, NodeIndex>,
    // undirected graph: nodes represent tables and edges represent foreign keys
    table_search_graph: Graph<String, String, Undirected>,
}

impl TableSearch {
    pub fn new(tables: &[TableSearchInfo], foreign_keys: &[ForeignKey]) -> Self {
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
    fn should_create_tables_and_foreign_keys() {
        TableSearch::new(
            &[
                TableSearchInfo::new("A".to_string(), "B".to_string()),
                TableSearchInfo::new("C".to_string(), "D".to_string()),
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
    fn should_find_no_path() -> Result<()> {
        let ts = TableSearch::new(
            &[
                TableSearchInfo::new("A".to_string(), "B".to_string()),
                TableSearchInfo::new("C".to_string(), "D".to_string()),
                TableSearchInfo::new("AA".to_string(), "BB".to_string()),
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
        let path = ts.path_to("A.B".to_string(), "AA.BB".to_string())?;

        let expected_nodes: Vec<String> = vec![];
        let expected_edges: Vec<String> = vec![];
        assert_eq!(path, (expected_nodes, expected_edges));

        Ok(())
    }

    #[test]
    fn should_find_path() -> Result<()> {
        let ts = TableSearch::new(
            &[
                TableSearchInfo::new("A".to_string(), "B".to_string()),
                TableSearchInfo::new("C".to_string(), "D".to_string()),
                TableSearchInfo::new("AA".to_string(), "BB".to_string()),
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
        let path = ts.path_to("A.B".to_string(), "C.D".to_string())?;

        let expected_nodes = vec!["A.B".to_string(), "C.D".to_string()];
        let expected_edges = vec!["e:f".to_string()];
        assert_eq!(path, (expected_nodes, expected_edges));

        Ok(())
    }

    #[test]
    fn should_find_path_2() -> Result<()> {
        let ts = TableSearch::new(
            &[
                TableSearchInfo::new("A".to_string(), "B".to_string()),
                TableSearchInfo::new("C".to_string(), "D".to_string()),
                TableSearchInfo::new("AA".to_string(), "BB".to_string()),
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

        let path = ts.path_to("A.B".to_string(), "AA.BB".to_string())?;

        let expected_nodes = vec!["A.B".to_string(), "C.D".to_string(), "AA.BB".to_string()];
        let expected_edges = vec!["e:f".to_string(), "g:h".to_string()];
        assert_eq!(path, (expected_nodes, expected_edges));

        Ok(())
    }

    #[test]
    fn should_find_path_when_edges_are_inverted() -> Result<()> {
        let ts = TableSearch::new(
            &[
                TableSearchInfo::new("A".to_string(), "B".to_string()),
                TableSearchInfo::new("C".to_string(), "D".to_string()),
                TableSearchInfo::new("AA".to_string(), "BB".to_string()),
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
        let path = ts.path_to("A.B".to_string(), "AA.BB".to_string())?;

        let expected_nodes = vec!["A.B".to_string(), "C.D".to_string(), "AA.BB".to_string()];
        let expected_edges = vec!["e:f".to_string(), "g:h".to_string()];
        assert_eq!(path, (expected_nodes, expected_edges));

        Ok(())
    }

    #[test]
    fn should_find_all_joinable_tables() -> Result<()> {
        let ts = TableSearch::new(
            &[
                TableSearchInfo::new("A".to_string(), "B".to_string()),
                TableSearchInfo::new("C".to_string(), "D".to_string()),
                TableSearchInfo::new("AA".to_string(), "BB".to_string()),
                TableSearchInfo::new("CC".to_string(), "DD".to_string()),
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

        let (nodes, edges) = ts.joinable_tables("A.B".to_string())?;

        let expected_nodes = vec!["A.B".to_string(), "AA.BB".to_string(), "C.D".to_string()];
        let expected_edges = vec!["e:f".to_string(), "g:h".to_string()];

        assert!(
            nodes.iter().all(|node| expected_nodes.contains(node))
                && edges.iter().all(|edge| expected_edges.contains(edge))
        );

        Ok(())
    }

    #[test]
    fn should_find_all_joinable_tables_2() -> Result<()> {
        let ts = TableSearch::new(
            &[
                TableSearchInfo::new("A".to_string(), "B".to_string()),
                TableSearchInfo::new("C".to_string(), "D".to_string()),
                TableSearchInfo::new("AA".to_string(), "BB".to_string()),
                TableSearchInfo::new("CC".to_string(), "DD".to_string()),
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

        let expected_nodes = vec!["CC.DD".to_string()];
        let expected_edges = vec![];

        assert_eq!(res, (expected_nodes, expected_edges));

        Ok(())
    }
}
