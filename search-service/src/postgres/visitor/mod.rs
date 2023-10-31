use crate::relational::entities::ForeignKey;
use crate::relational::table_search::entities::TableSearchInfo;
use crate::relational::table_search::TableSearch;
use crate::traits::Visitor;

use crate::query_representation::intermediary::Command;

use crate::query_representation::ultimate::command_to_query;

use anyhow::Error;

#[derive(Clone)]
pub struct PostgresVisitor {
    //TableSearch struct with information on the db's tables
    pub table_search: TableSearch,
}

impl PostgresVisitor {
    pub fn new(tables: Vec<TableSearchInfo>, foreign_keys: Vec<ForeignKey>) -> Self {
        Self {
            table_search: TableSearch::new(tables, foreign_keys),
        }
    }
}

impl Visitor for PostgresVisitor {
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

    use crate::postgres::visitor::PostgresVisitor;

    use crate::relational::entities::ForeignKey;
    use crate::relational::table_search::entities::TableSearchInfo;
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

        let postgres_visitor = PostgresVisitor::new(tables, fks);

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

        assert_eq!(sc_return, "SELECT movies.movie.runtime, movies.movie.revenue\nFROM movies.movie\nWHERE movies.movie.runtime > 200;".to_string());
        assert_eq!(cc_return, "SELECT movies.movie.runtime, movies.movie.revenue\nFROM movies.movie\nWHERE movies.movie.runtime > 200ANDmovies.movie.revenue > 1000000;".to_string());

        Ok(())
    }
}
