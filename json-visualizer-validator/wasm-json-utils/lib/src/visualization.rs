

use serde_wasm_bindgen;
use wasm_bindgen::prelude::*;
use serde::{Serialize, Deserialize};
use serde_json::Value;
use js_sys::JSON;
use std::collections::HashMap;

/// A simple node structure which represents a tree suitable for many D3.js layouts.
#[derive(Serialize, Deserialize)]
pub struct Node {
    /// The name or key of this JSON node.
    pub name: String,
    /// The child nodes; used if the JSON element is an object or an array.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub children: Option<Vec<Node>>,
    /// The actual JSON value for leaf nodes (if any).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<Value>,
}

/// Recursively converts a serde_json::Value into a Node tree.
/// The optional `key` helps preserve the original keys. If `None` it falls back to a default.
fn json_to_tree(key: Option<String>, data: &Value) -> Node {
    match data {
        // When the JSON data is an object, iterate over its keys and values.
        Value::Object(map) => {
            let children = map
                .iter()
                .map(|(k, v)| json_to_tree(Some(k.clone()), v))
                .collect();
            Node {
                name: key.unwrap_or_else(|| "root".into()),
                children: Some(children),
                value: None,
            }
        }

        // When the data is an array, iterate over its elements.
        Value::Array(arr) => {
            let children = arr
                .iter()
                .enumerate()
                .map(|(i, v)| json_to_tree(Some(i.to_string()), v))
                .collect();
            Node {
                name: key.unwrap_or_else(|| "array".into()),
                children: Some(children),
                value: None,
            }
        }

        // For primitives (string, number, bool, or null), store the value.
        _ => Node {
            name: key.unwrap_or_else(|| "value".into()),
            children: None,
            value: Some(data.clone()),
        },
    }
}

/// The exported wasm function that processes JSON.
/// It accepts a JSON string and returns a JsValue containing the tree structure.
/// This tree structure should be simple enough to pass directly to d3.js for creating visualizations.
#[wasm_bindgen]
pub fn process_json_tree(json_data: &str) -> Result<JsValue, JsValue> {
    // Parse the input JSON into a serde_json::Value.
    let parsed: Value = serde_json::from_str(json_data)
        .map_err(|e| JsValue::from_str(&e.to_string()))?;
    
    // Convert the parsed JSON value into our tree structure.
    let tree = json_to_tree(None, &parsed);
    
    // Convert the tree into a JsValue so we can send it back to JavaScript.
    serde_wasm_bindgen::to_value(&tree)
        .map_err(|e| JsValue::from_str(&e.to_string()))
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
