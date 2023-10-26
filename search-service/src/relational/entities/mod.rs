pub mod mapping;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct Attribute {
    pub name: String,
    pub data_type: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Table {
    pub schema: String,
    pub name: String,
    pub attributes: Vec<Attribute>,
    pub primary_keys: Vec<PrimaryKey>,
}

#[derive(Serialize, Deserialize)]
pub struct ForeignKey {
    pub schema_name: String,
    pub table_name: String,
    pub attribute_name: String,
    pub schema_name_foreign: String,
    pub table_name_foreign: String,
    pub attribute_name_foreign: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct PrimaryKey {
    pub schema_name: String,
    pub table_name: String,
    pub attribute_name: String,
}

#[derive(Serialize, Deserialize)]
pub struct DbSchema {
    pub tables: Vec<Table>,
    pub foreing_keys: Vec<ForeignKey>,
}

impl Attribute {
    pub fn new(arg_name: String, arg_type: String) -> Self {
        let name = arg_name;
        let data_type = arg_type;
        Self { name, data_type }
    }
}

impl Table {
    pub fn new(
        schema: String,
        name: String,
        attributes: Vec<Attribute>,
        primary_keys: Vec<PrimaryKey>,
    ) -> Self {
        Self {
            schema,
            name,
            attributes,
            primary_keys,
        }
    }
}

impl ForeignKey {
    pub fn new(
        schema_name: String,
        table_name: String,
        attribute_name: String,
        schema_name_foreign: String,
        table_name_foreign: String,
        attribute_name_foreign: String,
    ) -> Self {
        Self {
            schema_name,
            table_name,
            attribute_name,
            schema_name_foreign,
            table_name_foreign,
            attribute_name_foreign,
        }
    }
}

impl PrimaryKey {
    pub fn new(schema_name: String, table_name: String, attribute_name: String) -> Self {
        Self {
            schema_name,
            table_name,
            attribute_name,
        }
    }
}

impl DbSchema {
    pub fn new(tables: Vec<Table>, foreing_keys: Vec<ForeignKey>) -> Self {
        Self {
            tables,
            foreing_keys,
        }
    }
}
