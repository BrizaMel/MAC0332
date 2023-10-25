#[derive(PartialEq, Debug)]
pub struct SimpleCommand {
    pub attribute: String,
    pub operator: Operator,
    pub value: Value,
}

#[derive(PartialEq, Debug)]
pub struct Value {
    pub value:String,
    pub data_type:DataType,
}


#[derive(PartialEq, Debug)]
pub enum Operator {
    Equal,
    GreaterThan,
}

#[derive(PartialEq, Debug)]
pub enum DataType {
    Integer,
    String,
}

impl SimpleCommand {
    pub fn new(attribute:String,operator:Operator,value:Value) -> Self {
        Self {attribute,operator,value}
    }
}
impl Value {
    pub fn new(value:String,data_type:DataType) -> Self {
        Self {value,data_type}
    }
}

