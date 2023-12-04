use std::sync::Arc;

use crate::query_representation::intermediary::Command;
use crate::relational::entities::DbSchema;
use anyhow::Error;

use async_trait::async_trait;

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
pub trait SearchServiceStorage: Sync + Send {
    async fn get_db_schema_info(&self) -> Result<DbSchema, Error>;
    async fn execute(&self, query: String) -> Result<Vec<serde_json::Value>, Error>;
}
