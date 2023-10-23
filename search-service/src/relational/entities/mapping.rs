use crate::relational::table_search::entities::TableSearchInfo;

use super::Table;

impl From<Table> for TableSearchInfo {
    fn from(table: Table) -> Self {
        TableSearchInfo {
            schema: table.schema,
            name: table.name,
        }
    }
}
