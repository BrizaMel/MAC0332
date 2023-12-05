use serde_json::{Map, Value};
use tokio_postgres::Row;

pub fn row_to_json(row: Row) -> anyhow::Result<Value> {
    let mut object: Map<String, Value> = Map::new();

    for column in row.columns().iter() {
        let field_name = column.name();
        let field_value: String = row.try_get(field_name)?;
        object.insert(field_name.to_string(), serde_json::to_value(field_value)?);
    }

    Ok(Value::Object(object))
}
