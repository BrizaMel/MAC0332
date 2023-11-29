use std::sync::Arc;

use crate::query_representation::intermediary::Command;
use crate::relational::entities::DbSchema;
use anyhow::Error;

use async_trait::async_trait;

use crate::query_representation::intermediary::single_command::DataType;

pub trait Component {
    fn accept(&self, projection: Vec<String>, v: Arc<dyn Visitor>) -> Result<String, Error>;
}

pub trait Visitor {
    fn visit_command(&self, projection: Vec<String>, command: &Command) -> Result<String, Error>;
}

pub trait Expression {
    fn interpret(&self) -> Result<Command, Error>;
}

#[async_trait]
pub trait DatabaseOperations {
    async fn get_db_schema_info(&self) -> Result<DbSchema,Error>;
    fn translate_native_type(&self, mysql_type: &str) -> Result<DataType,Error>;
}