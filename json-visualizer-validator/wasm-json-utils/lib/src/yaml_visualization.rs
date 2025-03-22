use wasm_bindgen::prelude::*;
use serde::{Serialize, Deserialize};
use serde_json;
use serde_yaml;

/// A node in the graph view.
#[derive(Serialize)]
struct GraphNode {
    id: String,
    label: String,
    value: String,
    depth: usize,
    #[serde(skip_serializing_if = "Option::is_none")]
    parent: Option<String>,
    is_leaf: bool,
}

/// A link (edge) in the graph view.
#[derive(Serialize)]
struct GraphLink {
    source: String,
    target: String,
}

/// Output format for the graph: a set of nodes and links.
#[derive(Serialize)]
struct GraphOutput {
    nodes: Vec<GraphNode>,
    links: Vec<GraphLink>,
}

/// A node in the tree view.
#[derive(Serialize)]
struct TreeNode {
    name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    value: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    children: Option<Vec<TreeNode>>,
}

/// Converts a scalar serde_yaml::Value (or unsupported type) to a string.
/// For strings, we use serde_json::to_string so that quotes are added.
fn scalar_to_string(value: &serde_yaml::Value) -> String {
    match value {
        serde_yaml::Value::Null => "null".to_string(),
        serde_yaml::Value::Bool(b) => b.to_string(),
        serde_yaml::Value::Number(n) => n.to_string(),
        serde_yaml::Value::String(s) => {
            // This will add extra quotes (e.g., "Home Goods" becomes "\"Home Goods\"")
            serde_json::to_string(s).unwrap_or_else(|_| s.clone())
        }
        _ => format!("{:?}", value),
    }
}

/// Recursively traverse the YAML value to build nodes and links for the
/// graph format. Returns the id of the node that was just created.
fn build_graph(
    value: &serde_yaml::Value,
    label: Option<&str>,
    parent: Option<&str>,
    depth: usize,
    counter: &mut usize,
    nodes: &mut Vec<GraphNode>,
    links: &mut Vec<GraphLink>,
) -> String {
    // Create a unique id for the current node.
    let current_id = counter.to_string();
    *counter += 1;

    // Determine the node’s label, value (as shown) and if it’s a leaf.
    let (node_label, node_value, is_leaf) = match value {
        serde_yaml::Value::Mapping(map) => {
            let count = map.len();
            let lbl = label.unwrap_or("Object").to_string();
            (lbl, format!("{} items", count), count == 0)
        }
        serde_yaml::Value::Sequence(seq) => {
            let count = seq.len();
            let lbl = label.unwrap_or("Array").to_string();
            (lbl, format!("{} items", count), count == 0)
        }
        _ => {
            let s = scalar_to_string(value);
            let lbl = label.unwrap_or(&s).to_string();
            (lbl, s, true)
        }
    };

    // Create the current node.
    nodes.push(GraphNode {
        id: current_id.clone(),
        label: node_label,
        value: node_value,
        depth,
        parent: parent.map(|s| s.to_string()),
        is_leaf,
    });

    // If this node has a parent, record the edge.
    if let Some(p) = parent {
        links.push(GraphLink {
            source: p.to_string(),
            target: current_id.clone(),
        });
    }

    // If the current value is composite, process its children.
    match value {
        serde_yaml::Value::Mapping(map) => {
            for (k, v) in map.iter() {
                // Convert the key to a string for labeling.
                let key_str = match k {
                    serde_yaml::Value::String(s) => s.clone(),
                    _ => format!("{:?}", k),
                };
                build_graph(v, Some(&key_str), Some(&current_id), depth + 1, counter, nodes, links);
            }
        }
        serde_yaml::Value::Sequence(seq) => {
            for (i, elem) in seq.iter().enumerate() {
                build_graph(
                    elem,
                    Some(&format!("[{}]", i)),
                    Some(&current_id),
                    depth + 1,
                    counter,
                    nodes,
                    links,
                );
            }
        }
        _ => {}
    }
    current_id
}

/// Recursively convert the YAML value into a tree node.
fn build_tree_node(value: &serde_yaml::Value, key: &str) -> TreeNode {
    match value {
        serde_yaml::Value::Mapping(map) => {
            let mut children = Vec::new();
            for (k, v) in map.iter() {
                let key_str = match k {
                    serde_yaml::Value::String(s) => s.clone(),
                    _ => format!("{:?}", k),
                };
                children.push(build_tree_node(v, &key_str));
            }
            TreeNode {
                name: key.to_string(),
                value: None,
                children: Some(children),
            }
        }
        serde_yaml::Value::Sequence(seq) => {
            let mut children = Vec::new();
            for (i, elem) in seq.iter().enumerate() {
                children.push(build_tree_node(elem, &format!("[{}]", i)));
            }
            TreeNode {
                name: key.to_string(),
                value: None,
                children: Some(children),
            }
        }
        _ => TreeNode {
            name: key.to_string(),
            value: Some(scalar_to_string(value)),
            children: None,
        },
    }
}

/// Exposed WebAssembly function for processing YAML into a graph format.
///
/// It parses the YAML string and then recursively builds a list of nodes and
/// links. The returned JSON follows the format:
///
/// {
///    "nodes": [ { "id": "...", "label": "...", "value": "...", "depth": ..., "parent": "..."?, "is_leaf": ... }, ... ],
///    "links": [ { "source": "...", "target": "..." }, ... ]
/// }
#[wasm_bindgen]
pub fn process_yaml_graph(yaml_str: &str) -> Result<String, JsValue> {
    // Parse the YAML string.
    let value: serde_yaml::Value = serde_yaml::from_str(yaml_str)
        .map_err(|e| JsValue::from_str(&format!("Error parsing YAML: {}", e)))?;
    let mut nodes: Vec<GraphNode> = Vec::new();
    let mut links: Vec<GraphLink> = Vec::new();
    let mut counter: usize = 1;
    // Use "root" as the label for the top node.
    build_graph(&value, Some("root"), None, 0, &mut counter, &mut nodes, &mut links);
    let output = GraphOutput { nodes, links };
    serde_json::to_string(&output)
        .map_err(|e| JsValue::from_str(&format!("Error building JSON: {}", e)))
}

/// Exposed WebAssembly function for processing YAML into a tree format.
///
/// It parses the YAML string and converts it into a hierarchical tree where
/// each node has a "name" and, if a leaf, a "value". Composite values get a
/// "children" array. The returned JSON for an object looks like:
///
/// {
///    "name": "root",
///    "children": [
///       {"name": "category", "value": "\"Home Goods\""},
///       ...,
///       {"name": "specifications", "children": [ ... ] }
///    ]
/// }
#[wasm_bindgen]
pub fn process_yaml_tree(yaml_str: &str) -> Result<String, JsValue> {
    // Parse the YAML string.
    let value: serde_yaml::Value = serde_yaml::from_str(yaml_str)
        .map_err(|e| JsValue::from_str(&format!("Error parsing YAML: {}", e)))?;
    // For tree output, if the root is a mapping, list its keys as children.
    let tree = match value {
        serde_yaml::Value::Mapping(ref map) => {
            let mut children = Vec::new();
            for (k, v) in map.iter() {
                let key_str = match k {
                    serde_yaml::Value::String(s) => s.clone(),
                    _ => format!("{:?}", k), 
                };
                children.push(build_tree_node(v, &key_str));
            }
            TreeNode {
                name: "root".to_string(),
                value: None,
                children: Some(children),
            }
        }
        _ => TreeNode {
            name: "root".to_string(),
            value: Some(scalar_to_string(&value)),
            children: None,
        },
    };
    serde_json::to_string(&tree)
        .map_err(|e| JsValue::from_str(&format!("Error building JSON: {}", e)))
}
