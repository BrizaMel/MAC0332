use serde_json::{Map, Value};
use mysql::Row;

pub fn row_to_json(row: Row) -> anyhow::Result<Value> {
    let mut object: Map<String, Value> = Map::new();

    for (idx,column) in row.clone().columns().into_iter().enumerate(){
        let field_name = column.name_str();
        let field_value: String = row.clone().get(idx).expect("Error getting row element");
        object.insert(field_name.to_string(), serde_json::to_value(field_value)?);
    }

    Ok(Value::Object(object))
}
