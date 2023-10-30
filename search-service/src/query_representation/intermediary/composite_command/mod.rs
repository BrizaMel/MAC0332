use strum_macros::{Display, EnumString};

use crate::query_representation::intermediary::Command;

#[derive(PartialEq, Debug)]
pub struct CompositeCommand {
    pub logical_operator: LogicalOperator,
    pub commands: Vec<Command>,
}

#[derive(PartialEq, Debug, Display, EnumString)]
#[strum(serialize_all = "UPPERCASE")]
pub enum LogicalOperator {
    And,
    Or,
}

impl CompositeCommand {
    pub fn new(logical_operator: LogicalOperator, commands: Vec<Command>) -> Self {
        Self {
            logical_operator,
            commands,
        }
    }
}
