use serde_json::{Map, Value};
use serde_yaml;

pub fn convert_to_yaml(value: Value) -> serde_yaml::Result<String> {
    let map = value.as_object().unwrap();
    let mut result: serde_json::Map<String, Value> = serde_json::Map::new(); // Use serde_json::Map here
    let mut fields: Vec<Value> = Vec::new(); // We store fields as Vec<Value>

    // Add project_name and name to result
    result.insert(
        "project_name".to_string(),
        Value::String("MyGoProject".to_string()),
    );
    result.insert(
        "name".to_string(),
        Value::String(generate_struct_name("Root")),
    );

    for (key, val) in map.iter() {
        fields.push(Value::Object(process_field(key, val))); // Wrap the returned Map in a Value::Object
    }

    result.insert("fields".to_string(), Value::Array(fields));
    serde_yaml::to_string(&Value::Object(result)) // Wrap the result in a Value::Object for serde_yaml::to_string
}

fn process_field(key: &str, val: &Value) -> Map<String, Value> {
    let mut field_map = Map::new();
    field_map.insert("name".to_string(), Value::String(to_pascal_case(key)));
    field_map.insert("json_name".to_string(), Value::String(key.to_string()));
    field_map.insert(
        "dynamo_name".to_string(),
        Value::String(to_dynamo_name(key)),
    );

    match val {
        Value::String(_) => {
            field_map.insert("type".to_string(), Value::String("string".to_string()));
        }
        Value::Number(_) => {
            field_map.insert("type".to_string(), Value::String("int".to_string()));
        }
        Value::Array(arr) => {
            if arr[0].is_string() {
                field_map.insert("type".to_string(), Value::String("string".to_string()));
            } else {
                field_map.insert("type".to_string(), Value::String("int".to_string()));
            }
            field_map.insert("is_slice".to_string(), Value::Bool(true));
        }
        Value::Object(ref obj) => {
            let nested = convert_to_yaml_nested(obj, key);
            field_map.insert("nested_struct".to_string(), nested);
        }
        _ => {}
    }

    field_map
}

pub fn convert_to_yaml_nested(obj: &Map<String, Value>, parent_name: &str) -> Value {
    let mut nested_map = Map::new();
    let mut fields: Vec<Value> = Vec::new();

    for (key, val) in obj.iter() {
        fields.push(Value::Object(process_field(key, val)));
    }

    nested_map.insert(
        "name".to_string(),
        Value::String(generate_struct_name(parent_name)),
    );
    nested_map.insert("fields".to_string(), Value::Array(fields));

    Value::Object(nested_map)
}

fn generate_struct_name(base_name: &str) -> String {
    format!("{}Details", to_pascal_case(base_name))
}

fn to_pascal_case(s: &str) -> String {
    s.split('_')
        .map(|word| {
            let mut w = word.chars();
            w.next()
                .unwrap()
                .to_uppercase()
                .chain(w)
                .collect::<String>()
        })
        .collect()
}

fn to_dynamo_name(s: &str) -> String {
    s.to_uppercase().replace("-", "_")
}
