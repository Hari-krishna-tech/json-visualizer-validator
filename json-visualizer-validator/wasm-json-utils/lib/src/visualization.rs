use wasm_bindgen::prelude::*;
use serde::{Serialize, Deserialize};
use serde_json::Value;
use js_sys::JSON;

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

// Generate a simple ID without uuid dependency
fn generate_id() -> String {
    let timestamp = js_sys::Date::now() as u64;
    let random = (js_sys::Math::random() * 10000.0) as u64;
    format!("id-{}-{}", timestamp, random)
}

#[wasm_bindgen]
pub fn process_json(json_str: &str) -> Result<String, JsValue> {
    // Set up panic hook for better error messages
    console_error_panic_hook::set_once();
    
    // Parse the JSON string
    let value: Value = match serde_json::from_str(json_str) {
        Ok(v) => v,
        Err(e) => return Err(JsValue::from_str(&format!("Parse error: {}", e))),
    };
    
    // Process the JSON into nodes and links
    let mut nodes = Vec::new();
    let mut links = Vec::new();
    
    if !value.is_null() {
        let root_id = generate_id();
        process_value(&value, None, 0, &root_id, &mut nodes, &mut links);
    }
    
    // Create the output structure
    let processed_data = ProcessedData { nodes, links };
    
    // Serialize to JSON string
    match serde_json::to_string(&processed_data) {
        Ok(json) => Ok(json),
        Err(e) => Err(JsValue::from_str(&format!("Serialization error: {}", e))),
    }
}

// Process a JSON value and add nodes/links to our collections
fn process_value(
    value: &Value,
    parent_id: Option<&str>,
    depth: usize,
    current_id: &str,
    nodes: &mut Vec<JsonNode>,
    links: &mut Vec<JsonLink>,
) {
    match value {
        Value::Object(obj) => {
            // Add node for this object
            nodes.push(JsonNode {
                id: current_id.to_string(),
                label: "Object".to_string(),
                value: format!("{} items", obj.len()),
                depth,
                parent: parent_id.map(String::from),
                is_leaf: false,
            });
            
            // Process each key-value pair
            for (key, val) in obj {
                let child_id = generate_id();
                
                // Add a key node
                let key_id = generate_id();
                nodes.push(JsonNode {
                    id: key_id.clone(),
                    label: "Key".to_string(),
                    value: key.to_string(),
                    depth: depth + 1,
                    parent: Some(current_id.to_string()),
                    is_leaf: true,
                });
                
                links.push(JsonLink {
                    source: current_id.to_string(),
                    target: key_id.clone(),
                });
                
                // Process the value
                process_value(val, Some(current_id), depth + 1, &child_id, nodes, links);
                
                // Link from key to value
                links.push(JsonLink {
                    source: key_id,
                    target: child_id,
                });
            }
        },
        Value::Array(arr) => {
            // Add node for this array
            nodes.push(JsonNode {
                id: current_id.to_string(),
                label: "Array".to_string(),
                value: format!("{} items", arr.len()),
                depth,
                parent: parent_id.map(String::from),
                is_leaf: arr.is_empty(),
            });
            
            // Process each array element
            for (i, val) in arr.iter().enumerate() {
                let child_id = generate_id();
                
                // Add index node (optional - can be removed if not needed)
                let index_id = generate_id();
                nodes.push(JsonNode {
                    id: index_id.clone(),
                    label: "Index".to_string(),
                    value: i.to_string(),
                    depth: depth + 1,
                    parent: Some(current_id.to_string()),
                    is_leaf: true,
                });
                
                links.push(JsonLink {
                    source: current_id.to_string(),
                    target: index_id.clone(),
                });
                
                // Process the value
                process_value(val, Some(current_id), depth + 1, &child_id, nodes, links);
                
                // Link from index to value
                links.push(JsonLink {
                    source: index_id,
                    target: child_id,
                });
            }
        },
        Value::String(s) => {
            nodes.push(JsonNode {
                id: current_id.to_string(),
                label: "String".to_string(),
                value: s.clone(),
                depth,
                parent: parent_id.map(String::from),
                is_leaf: true,
            });
        },
        Value::Number(n) => {
            nodes.push(JsonNode {
                id: current_id.to_string(),
                label: "Number".to_string(),
                value: n.to_string(),
                depth,
                parent: parent_id.map(String::from),
                is_leaf: true,
            });
        },
        Value::Bool(b) => {
            nodes.push(JsonNode {
                id: current_id.to_string(),
                label: "Boolean".to_string(),
                value: b.to_string(),
                depth,
                parent: parent_id.map(String::from),
                is_leaf: true,
            });
        },
        Value::Null => {
            nodes.push(JsonNode {
                id: current_id.to_string(),
                label: "Null".to_string(),
                value: "null".to_string(),
                depth,
                parent: parent_id.map(String::from),
                is_leaf: true,
            });
        },
    }
    
    // Link parent to current if parent exists
    if let Some(parent) = parent_id {
        links.push(JsonLink {
            source: parent.to_string(),
            target: current_id.to_string(),
        });
    }
}