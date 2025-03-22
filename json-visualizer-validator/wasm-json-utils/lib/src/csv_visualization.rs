use wasm_bindgen::prelude::*;
use serde::{Serialize};
use serde_json::{self, Value};
use csv;

// Define the node and link structure for the graph.
#[derive(Serialize)]
struct Node {
    id: String,
    label: String,
    value: String,
    depth: usize,
    #[serde(skip_serializing_if = "Option::is_none")]
    parent: Option<String>,
    is_leaf: bool,
}

#[derive(Serialize)]
struct Link {
    source: String,
    target: String,
}

#[derive(Serialize)]
struct Graph {
    nodes: Vec<Node>,
    links: Vec<Link>,
}

// A helper structure to progressively build the graph.
struct GraphBuilder {
    nodes: Vec<Node>,
    links: Vec<Link>,
    counter: usize,
}

impl GraphBuilder {
    fn new() -> Self {
        GraphBuilder {
            nodes: Vec::new(),
            links: Vec::new(),
            counter: 1,
        }
    }

    fn next_id(&mut self) -> String {
        let id = self.counter;
        self.counter += 1;
        id.to_string()
    }
}

/// Recursively process a JSON value and push nodes and links into the builder.
/// Each node holds its label, string-converted value (or a string like "6 items")
/// plus its depth and (optionally) parent id.
fn process_node(
    builder: &mut GraphBuilder,
    value: &Value,
    label: &str,
    depth: usize,
    parent: Option<String>,
) -> String {
    let id = builder.next_id();
    let is_leaf;
    let node_value = match value {
        Value::Object(map) => {
            is_leaf = false;
            format!("{} items", map.len())
        }
        Value::Array(arr) => {
            is_leaf = false;
            format!("{} items", arr.len())
        }
        _ => {
            // For primitives, just use the string representation.
            is_leaf = true;
            value.to_string()
        }
    };

    let node = Node {
        id: id.clone(),
        label: label.to_string(),
        value: node_value,
        depth,
        parent: parent.clone(),
        is_leaf,
    };

    builder.nodes.push(node);

    // If a parent exists, add a link from the parent to this node.
    if let Some(p) = parent {
        builder.links.push(Link {
            source: p,
            target: id.clone(),
        });
    }

    // If this node is not a leaf, recursively process its children.
    if !is_leaf {
        match value {
            Value::Object(map) => {
                for (k, v) in map.iter() {
                    process_node(builder, v, k, depth + 1, Some(id.clone()));
                }
            }
            Value::Array(arr) => {
                // Process each array element. Here we label each child with its index.
                for (i, item) in arr.iter().enumerate() {
                    let item_label = i.to_string();
                    process_node(builder, item, &item_label, depth + 1, Some(id.clone()));
                }
            }
            _ => {}
        }
    }

    id
}

/// Build a graph representation by starting with the JSON value at the root.
fn build_graph_from_json(value: &Value) -> Graph {
    let mut builder = GraphBuilder::new();
    let label = match value {
        Value::Object(_) => "Object",
        Value::Array(_) => "Array",
        _ => "Value",
    };
    process_node(&mut builder, value, label, 0, None);
    Graph {
        nodes: builder.nodes,
        links: builder.links,
    }
}

// Define the tree structure. If a node is not a leaf, it has a children list;
// otherwise it has a value.
#[derive(Serialize)]
struct Tree {
    name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    value: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    children: Option<Vec<Tree>>,
}

/// Recursively build the tree structure.
/// For objects/arrays the "children" field is recursively filled in.
fn build_tree(name: &str, value: &Value) -> Tree {
    match value {
        Value::Object(map) => {
            let children: Vec<Tree> = map
                .iter()
                .map(|(k, v)| build_tree(k, v))
                .collect();
            Tree {
                name: name.to_string(),
                value: None,
                children: Some(children),
            }
        }
        Value::Array(arr) => {
            let children: Vec<Tree> = arr
                .iter()
                .enumerate()
                .map(|(i, v)| build_tree(&i.to_string(), v))
                .collect();
            Tree {
                name: name.to_string(),
                value: None,
                children: Some(children),
            }
        }
        // For primitives, simply set the string version of the value.
        _ => Tree {
            name: name.to_string(),
            value: Some(value.to_string()),
            children: None,
        },
    }
}

/// Process the CSV string and return a JSON string for graph visualization.
///
/// This function:
/// 1. Parses the CSV (using the first record).
/// 2. Converts the record into a JSON object (attempting a JSON parse on each field).
/// 3. Recursively builds the graph representation.
#[wasm_bindgen]
pub fn process_csv_graph(csv: &str) -> Result<String, JsValue> {
    // Create CSV reader from the input string.
    let mut rdr = csv::Reader::from_reader(csv.as_bytes());
    let headers = rdr
        .headers()
        .map_err(|e| JsValue::from_str(&format!("CSV header error: {}", e)))?
        .clone();

    // Read the first record.
    let record = rdr
        .records()
        .next()
        .ok_or_else(|| JsValue::from_str("No CSV record found"))?
        .map_err(|e| JsValue::from_str(&format!("CSV record error: {}", e)))?;

    // Build a JSON object from the CSV record.
    let mut map = serde_json::Map::new();
    for (header, field) in headers.iter().zip(record.iter()) {
        // Try to parse the field as JSON. If that fails, leave it as a string.
        let parsed: Result<Value, _> = serde_json::from_str(field);
        let value = parsed.unwrap_or(Value::String(field.to_string()));
        map.insert(header.to_string(), value);
    }
    let json_value = Value::Object(map);

    // Build and serialize the graph from the JSON object.
    let graph = build_graph_from_json(&json_value);
    serde_json::to_string(&graph)
        .map_err(|e| JsValue::from_str(&format!("Serialization error: {}", e)))
}

/// Process the CSV string and return a JSON string for tree visualization.
///
/// This function behaves like `process_csv_graph` except that
/// it builds a nested tree structure instead.
#[wasm_bindgen]
pub fn process_csv_tree(csv: &str) -> Result<String, JsValue> {
    let mut rdr = csv::Reader::from_reader(csv.as_bytes());
    let headers = rdr
        .headers()
        .map_err(|e| JsValue::from_str(&format!("CSV header error: {}", e)))?
        .clone();

    let record = rdr
        .records()
        .next()
        .ok_or_else(|| JsValue::from_str("No CSV record found"))?
        .map_err(|e| JsValue::from_str(&format!("CSV record error: {}", e)))?;

    let mut map = serde_json::Map::new();
    for (header, field) in headers.iter().zip(record.iter()) {
        let parsed: Result<Value, _> = serde_json::from_str(field);
        let value = parsed.unwrap_or(Value::String(field.to_string()));
        map.insert(header.to_string(), value);
    }
    let json_value = Value::Object(map);

    // The root of the tree is fixed as "root".
    let mut children = Vec::new();
    if let Value::Object(map) = &json_value {
        for (k, v) in map.iter() {
            children.push(build_tree(k, v));
        }
    }
    let root = Tree {
        name: "root".to_string(),
        value: None,
        children: Some(children),
    };

    serde_json::to_string(&root)
        .map_err(|e| JsValue::from_str(&format!("Serialization error: {}", e)))
}
