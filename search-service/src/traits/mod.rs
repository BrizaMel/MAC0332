use std::sync::Arc;

use crate::query_representation::intermediary::Command;

use anyhow::Error;

pub trait Component {
    fn accept(&self, projection: Vec<String>, v: Arc<dyn Visitor>) -> Result<String, Error>;
}

pub trait Visitor {
    fn visit_command(&self, projection: Vec<String>, command: &Command) -> Result<String, Error>;
}

pub trait Expression {
    fn interpret(&self) -> Result<Command, Error>;
}
