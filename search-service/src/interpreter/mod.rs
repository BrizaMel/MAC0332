mod simple_command;
mod composite_command;
mod postgres_visitor;
mod tests;

pub use simple_command::SimpleCommand;
pub use composite_command::CompositeCommand;
use anyhow::Error;

pub trait Component {
    fn accept(&self, projection:Vec<String>, v: &'static dyn Visitor) -> Result<String, Error>;
}

pub trait Visitor {
    fn visit_simple_command(&self, projection:Vec<String>, command: &SimpleCommand) -> Result<String, Error>;
    fn visit_composite_command(&self, projection:Vec<String>, command: &CompositeCommand) -> Result<String, Error>;
}