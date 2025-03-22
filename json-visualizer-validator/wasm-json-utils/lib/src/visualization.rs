

use serde_wasm_bindgen;
use wasm_bindgen::prelude::*;
use serde::{Serialize, Deserialize};
use serde_json::{Value, json};
use js_sys::JSON;
use std::collections::HashMap;

#[wasm_bindgen]
pub fn process_json_tree(json_str: &str) -> Result<String, JsValue> {
    // Parse the JSON
    let parsed = match serde_json::from_str::<Value>(json_str) {
        Ok(v) => v,
        Err(e) => return Err(JsValue::from_str(&format!("Failed to parse JSON: {}", e))),
    };
    
    // Convert to D3.js friendly format
    let tree = convert_to_d3_format(&parsed, "root");
    
    // Serialize back to JSON string
    match serde_json::to_string(&tree) {
        Ok(s) => Ok(s),
        Err(e) => Err(JsValue::from_str(&format!("Failed to serialize result: {}", e))),
    }
}

fn convert_to_d3_format(value: &Value, name: &str) -> Value {
    match value {
        Value::Object(obj) => {
            let mut result = json!({
                "name": name,
                "children": []
            });
            
            let children = obj.iter()
                .map(|(k, v)| convert_to_d3_format(v, k))
                .collect::<Vec<Value>>();
            
            result["children"] = json!(children);
            result
        },
        Value::Array(arr) => {
            let mut result = json!({
                "name": name,
                "children": []
            });
            
            let children = arr.iter()
                .enumerate()
                .map(|(i, v)| convert_to_d3_format(v, &format!("{}[{}]", name, i)))
                .collect::<Vec<Value>>();
            
            result["children"] = json!(children);
            result
        },
        _ => {
            json!({
                "name": name,
                "value": value.to_string()
            })
        }
    }
}




// Define the data structures
#[derive(Serialize, Deserialize, Clone)]
pub struct JsonNode {
    pub id: String,
    pub label: String,
    pub value: String,
    pub depth: usize,
    pub parent: Option<String>,
    pub is_leaf: bool,
}

#[derive(Serialize, Deserialize)]
pub struct JsonLink {
    pub source: String,
    pub target: String,
}

#[derive(Serialize, Deserialize)]
pub struct ProcessedData {
    nodes: Vec<JsonNode>,
    links: Vec<JsonLink>,
}

#[wasm_bindgen]
pub fn process_json(json_str: &str) -> Result<JsValue, JsError> {
    // Parse the JSON string
    let json_value: Value = serde_json::from_str(json_str)
        .map_err(|e| JsError::new(&format!("Failed to parse JSON: {}", e)))?;
    
    let mut nodes = Vec::new();
    let mut links = Vec::new();
    let mut next_id = 1;
    
    // Process the root element
    process_value(
        &json_value,
        None,
        0,
        &mut nodes,
        &mut links,
        &mut next_id,
        None,
    );
    
    // Create the processed data structure
    let processed_data = ProcessedData { nodes, links };
    
    // Serialize to JsValue
    Ok(serde_wasm_bindgen::to_value(&processed_data)
        .map_err(|e| JsError::new(&format!("Failed to serialize: {}", e)))?)
}

fn process_value(
    value: &Value,
    parent_id: Option<String>,
    depth: usize,
    nodes: &mut Vec<JsonNode>,
    links: &mut Vec<JsonLink>,
    next_id: &mut usize,
    key: Option<&str>,
) -> String {
    let id = next_id.to_string();
    *next_id += 1;
    
    match value {
        Value::Object(map) => {
            // Calculate number of items
            let item_count = map.len();
            let label = key.unwrap_or("Object").to_string();
            let value_text = format!("{} items", item_count);
            
            // Create node for this object
            let node = JsonNode {
                id: id.clone(),
                label,
                value: value_text,
                depth,
                parent: parent_id.clone(),
                is_leaf: false,
            };
            nodes.push(node);
            
            // Create link to parent if exists
            if let Some(parent) = &parent_id {
                links.push(JsonLink {
                    source: parent.clone(),
                    target: id.clone(),
                });
            }
            
            // Process each property in the object
            for (prop_key, prop_value) in map {
                let child_id = process_value(
                    prop_value,
                    Some(id.clone()),
                    depth + 1,
                    nodes,
                    links,
                    next_id,
                    Some(prop_key),
                );
            }
            
            id
        },
        Value::Array(arr) => {
            // Calculate number of items
            let item_count = arr.len();
            let label = key.unwrap_or("Array").to_string();
            let value_text = format!("{}", item_count);
            
            // Create node for this array
            let node = JsonNode {
                id: id.clone(),
                label,
                value: value_text,
                depth,
                parent: parent_id.clone(),
                is_leaf: false,
            };
            nodes.push(node);
            
            // Create link to parent if exists
            if let Some(parent) = &parent_id {
                links.push(JsonLink {
                    source: parent.clone(),
                    target: id.clone(),
                });
            }
            
            // Process each element in the array
            for (i, item) in arr.iter().enumerate() {
                let child_id = process_value(
                    item,
                    Some(id.clone()),
                    depth + 1,
                    nodes,
                    links,
                    next_id,
                    Some(&i.to_string()),
                );
            }
            
            id
        },
        // Handle primitive values (string, number, boolean, null)
        _ => {
            let (label, value_text) = format_primitive(key, value);
            
            // Create node for this primitive value
            let node = JsonNode {
                id: id.clone(),
                label,
                value: value_text,
                depth,
                parent: parent_id.clone(),
                is_leaf: true,
            };
            nodes.push(node);
            
            // Create link to parent if exists
            if let Some(parent) = &parent_id {
                links.push(JsonLink {
                    source: parent.clone(),
                    target: id.clone(),
                });
            }
            
            id
        }
    }
}

fn format_primitive(key: Option<&str>, value: &Value) -> (String, String) {
    let label = key.unwrap_or("Value").to_string();
    
    let value_text = match value {
        Value::String(s) => format!("{}", s),
        Value::Number(n) => {
            if n.is_i64() {
                format!("{}", n.as_i64().unwrap())
            } else if n.is_u64() {
                format!("{}", n.as_u64().unwrap())
            } else {
                format!("{}", n.as_f64().unwrap())
            }
        },
        Value::Bool(b) => format!("{}", b),
        Value::Null => "null".to_string(),
        _ => "".to_string(), // This shouldn't happen
    };
    
    (label, value_text)
}
