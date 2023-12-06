// Module responsible for hosting the TableSearch struct
// The struct is responsible for dealing with the feasibility
// of operations involving fields from different tables of a
// given relational database.
pub mod entities;
pub mod errors;

use anyhow::Result;
use petgraph::{
    algo::astar,
    graph::{Graph, NodeIndex},
    unionfind::UnionFind,
    Undirected,
};

use std::collections::{HashMap, HashSet};

use std::cmp::{min,max};

use crate::relational::entities::ForeignKey;

use self::{entities::TableSearchInfo, errors::TableSearchError};

#[derive(Clone)]
pub struct TableSearch {
    // maps table identifiers (in the format schema_name.table_name) to their corresponding node indices in the graph
    table_identifier_to_node_index: HashMap<String, NodeIndex>,
    // undirected graph: nodes represent tables and edges represent foreign keys
    table_search_graph: Graph<String, String, Undirected>,
}

impl TableSearch {
    pub fn new(tables: Vec<TableSearchInfo>, foreign_keys: Vec<ForeignKey>) -> Self {
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

        Self {
            table_identifier_to_node_index,
            table_search_graph,
        }
    }

    pub fn get_join_requirements(&self, atrs: &Vec<String>) -> (Vec<String>, Vec<String>) {
        println!("{:?}", atrs);
        let mut tables_needed: HashSet<String> = HashSet::from([]);
        let mut attributes_needed: HashSet<String> = HashSet::from([]);

        let mut tables_uf: UnionFind<usize> = UnionFind::new(atrs.len());

        if atrs.len() > 1 {
            for i in 0..atrs.len() {
                for j in i + 1..atrs.len() {
                    let (new_tables, new_attrs) =
                        &self.get_attibute_pair_requirements(&atrs[i], &atrs[j]);

                    if new_tables.len() > 0 {
                        tables_uf.union(i, j);
                    }

                    tables_needed.extend(new_tables.to_owned());
                    attributes_needed.extend(new_attrs.to_owned());

                }
            }
        } else if atrs.len() == 1 {
            let (table_str, atr_str) = &self.get_atr_info(&atrs[0]);

            tables_needed.insert(table_str.to_owned());
            attributes_needed.insert(atr_str.to_owned());
        }

        let table_sets = tables_uf.into_labeling();
        for i in 0..atrs.len() - 1 {
            if table_sets[i] != table_sets[i + 1] {
                panic!("Atributes can't be joined")
                // TODO: actually throw the error here. I couldn't manage to do it myself :(
                // TableSearchError::AtributesCantBeJoined()?
            }
        }

        let mut tables_needed_as_vec: Vec<String> = tables_needed.into_iter().collect();
        tables_needed_as_vec.sort();

        let mut attributes_needed_as_vec: Vec<String> = attributes_needed.into_iter().collect();
        attributes_needed_as_vec.sort();

        (
            tables_needed_as_vec,
            attributes_needed_as_vec,
        )
    }

    fn get_attibute_pair_requirements(
        &self,
        atr1: &String,
        atr2: &String,
    ) -> (HashSet<String>, HashSet<String>) {
        let (table_str1, _atr_str1) = &self.get_atr_info(&atr1);
        let (table_str2, _atr_str2) = &self.get_atr_info(&atr2);

        let mut attributes_needed: HashSet<String> = HashSet::from([]);

        // I chose to use unwrap here as this should be receiving a list of proper attributes (from tables
        // which exist in the given DB). If that is  not the case, this part of the system should be
        // refactored.
        let (tables_needed, fks) = self
            .path_to(table_str1.to_string(), table_str2.to_string())
            .unwrap();

        for i in 0..fks.len() {
            let atributes: Vec<&str> = fks[i].split(":").collect();

            let atribute1 = format!("{}.{}", tables_needed[i], atributes[0]).to_string();
            let atribute2 = format!("{}.{}", tables_needed[i + 1], atributes[1]).to_string();

            attributes_needed.insert(format!("{}:{}", min(atribute1.to_owned(),atribute2.to_owned()), max(atribute1,atribute2)));

        }

        let tables_needed_set: HashSet<String> = HashSet::from_iter(tables_needed.into_iter());

        (tables_needed_set, attributes_needed)
    }

    fn get_atr_info(&self, atr: &String) -> (String, String) {
        let words_vec: Vec<&str> = atr.split(".").collect();

        (
            format!("{}.{}", words_vec[0], words_vec[1]).to_string(),
            words_vec[2].to_string(),
        )
    }

    pub fn path_to(
        &self,
        origin: String,
        destiny: String,
    ) -> Result<(Vec<String>, Vec<String>), TableSearchError> {
        let origin_index = self
            .table_identifier_to_node_index
            .get(&origin)
            .ok_or_else(|| TableSearchError::TableNotFoundInGraph(origin))?;
        let destiny_index = self
            .table_identifier_to_node_index
            .get(&destiny)
            .ok_or_else(|| TableSearchError::TableNotFoundInGraph(destiny.clone()))?;

        let (mut tables, mut ordered_edges) =
            self.get_paths(*origin_index, Some(*destiny_index))?;

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
            .ok_or_else(|| TableSearchError::TableNotFoundInGraph(origin))?;
        let (tables, ordered_edges) = self.get_paths(*origin_index, None)?;
        Ok((tables, ordered_edges))
    }

    fn get_paths(
        &self,
        origin_index: NodeIndex,
        destiny_index: Option<NodeIndex>,
    ) -> Result<(Vec<String>, Vec<String>), TableSearchError> {

        let path = astar(
            &self.table_search_graph,
            origin_index,               // start
            |n| if destiny_index.is_none() {
                    false
                } else{
                    n == destiny_index.unwrap()     // is_goal
                },
            |_| 1, // edge_cost
            |_| 0,           // estimate_cost
        );

        let mut tables = vec![];
        let mut ordered_edges = vec![];
        let mut ordered_nodes : Vec<NodeIndex> = vec![origin_index];
        
        if !path.is_none() {
            ordered_nodes = path.unwrap().1;
        }
        
        let num_of_nodes = match destiny_index {
            None => 0,
            Some(_) => ordered_nodes.len(),
        };


        for i in 0..ordered_nodes.len() {
            let node_index = ordered_nodes[i];
            let table_identifier = self.table_search_graph.node_weight(node_index).unwrap();
            tables.push(table_identifier.clone());

            if i > 0 && i < num_of_nodes {
                let previous_node_index  = ordered_nodes[i - 1];
                
                let edge_wrapped = self
                    .table_search_graph
                    .find_edge(previous_node_index, node_index);

                if edge_wrapped.is_none() {
                    continue;
                }

                let edge = edge_wrapped.unwrap();

                let edge_weight = self
                    .table_search_graph
                    .edge_weight(edge)
                    .ok_or_else(|| TableSearchError::EdgeNotFoundInGraph)?;

                ordered_edges.push(edge_weight.into());

            }
        }
    
        Ok((tables, ordered_edges))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_create_tables_and_foreign_keys() {
        TableSearch::new(
            vec![
                TableSearchInfo::new("A".to_string(), "B".to_string()),
                TableSearchInfo::new("C".to_string(), "D".to_string()),
            ],
            vec![ForeignKey::new(
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
            vec![
                TableSearchInfo::new("A".to_string(), "B".to_string()),
                TableSearchInfo::new("C".to_string(), "D".to_string()),
                TableSearchInfo::new("AA".to_string(), "BB".to_string()),
            ],
            vec![ForeignKey::new(
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
            vec![
                TableSearchInfo::new("A".to_string(), "B".to_string()),
                TableSearchInfo::new("C".to_string(), "D".to_string()),
                TableSearchInfo::new("AA".to_string(), "BB".to_string()),
            ],
            vec![ForeignKey::new(
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
            vec![
                TableSearchInfo::new("A".to_string(), "B".to_string()),
                TableSearchInfo::new("C".to_string(), "D".to_string()),
                TableSearchInfo::new("AA".to_string(), "BB".to_string()),
            ],
            vec![
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
            vec![
                TableSearchInfo::new("A".to_string(), "B".to_string()),
                TableSearchInfo::new("C".to_string(), "D".to_string()),
                TableSearchInfo::new("AA".to_string(), "BB".to_string()),
            ],
            vec![
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
            vec![
                TableSearchInfo::new("A".to_string(), "B".to_string()),
                TableSearchInfo::new("C".to_string(), "D".to_string()),
                TableSearchInfo::new("AA".to_string(), "BB".to_string()),
                TableSearchInfo::new("CC".to_string(), "DD".to_string()),
            ],
            vec![
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
            vec![
                TableSearchInfo::new("A".to_string(), "B".to_string()),
                TableSearchInfo::new("C".to_string(), "D".to_string()),
                TableSearchInfo::new("AA".to_string(), "BB".to_string()),
                TableSearchInfo::new("CC".to_string(), "DD".to_string()),
            ],
            vec![
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
