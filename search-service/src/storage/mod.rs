use crate::relational::table_search::TableSearch;
use crate::traits::Visitor;

use crate::query_representation::intermediary::Command;

use crate::query_representation::ultimate::command_to_query;

use anyhow::Error;

pub mod mysql;
pub mod postgres;

#[derive(Clone)]
pub struct DatabaseVisitor {
    //TableSearch struct with information on the db's tables
    pub table_search: TableSearch,
}

impl DatabaseVisitor {
    pub fn new(table_search: TableSearch) -> Self {
        Self { table_search }
    }
}

impl Visitor for DatabaseVisitor {
    fn visit_command(&self, projection: Vec<String>, command: &Command) -> Result<String, Error> {
        let query = command_to_query(projection, command, &self.table_search)?;

        Ok(query)
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use crate::query_representation::intermediary::Command;

    use crate::query_representation::intermediary::tests::tests::{
        create_composite_command, create_simple_command,
    };

    use crate::relational::entities::ForeignKey;
    use crate::relational::table_search::entities::TableSearchInfo;
    use crate::relational::table_search::TableSearch;
    use crate::storage::DatabaseVisitor;
    use crate::traits::Component;

    use anyhow::Error;

    #[test]
    fn test_visitor_architecture() -> Result<(), Error> {
        let simple_command = create_simple_command()?;
        let composite_command = create_composite_command()?;

        //TODO: Pass correct lists of Tables and ForeignKeys to visitor
        let tables = vec![TableSearchInfo::new(
            "movies".to_string(),
            "movie".to_string(),
        )];
        let fks: Vec<ForeignKey> = vec![];

        let table_search = TableSearch::new(tables, fks);
        let postgres_visitor = DatabaseVisitor::new(table_search);

        let sc_return = Command::SingleCommand(simple_command).accept(
            vec![
                "movies.movie.runtime".to_string(),
                "movies.movie.revenue".to_string(),
            ],
            Arc::new(postgres_visitor.clone()),
        )?;

        let cc_return = Command::CompositeCommand(composite_command).accept(
            vec![
                "movies.movie.runtime".to_string(),
                "movies.movie.revenue".to_string(),
            ],
            Arc::new(postgres_visitor),
        )?;

        assert_eq!(sc_return, "SELECT movies.movie.runtime::TEXT, movies.movie.revenue::TEXT\nFROM movies.movie\nWHERE (movies.movie.runtime > 200);".to_string());
        assert_eq!(cc_return, "SELECT movies.movie.runtime::TEXT, movies.movie.revenue::TEXT\nFROM movies.movie\nWHERE ((movies.movie.runtime > 200) AND (movies.movie.revenue > 1000000));".to_string());

        Ok(())
    }
}
