// Cargo.toml dependencies:
//
// [dependencies]
// wasm-bindgen = "0.2"
// serde = { version = "1.0", features = ["derive"] }
// serde_json = "1.0"
// quick-xml = "0.30"
//
// [lib]
// crate-type = ["cdylib", "rlib"]

use wasm_bindgen::prelude::*;
use serde::Serialize;
use quick_xml::Reader;
use quick_xml::events::Event;

/// An intermediate representation of an XML element.
#[derive(Debug)]
struct XmlNode {
    name: String,
    /// For leaf nodes, this stores the text value (if any)
    value: Option<String>,
    children: Vec<XmlNode>,
}

/// Graph visualization types.
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

#[derive(Serialize)]
struct GraphLink {
    source: String,
    target: String,
}

#[derive(Serialize)]
struct Graph {
    nodes: Vec<GraphNode>,
    links: Vec<GraphLink>,
}

/// Tree visualization types.
#[derive(Serialize)]
struct TreeNode {
    name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    value: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    children: Option<Vec<TreeNode>>,
}

/// Recursively parses an XML element after a Start event is encountered.
/// It accumulates text and child nodes until the corresponding End event.
fn parse_element(
    reader: &mut Reader<&[u8]>,
    start: &quick_xml::events::BytesStart,
) -> Result<XmlNode, String> {
    // Convert the tag name (BytesStart) into &str using as_ref()
    let name = std::str::from_utf8(start.name().as_ref())
        .map_err(|_| "Invalid UTF-8 in tag name".to_string())?
        .to_string();

    let mut node = XmlNode {
        name,
        value: None,
        children: Vec::new(),
    };

    let mut text_content = String::new();

    // In quick-xml v0.30 the read_event method takes no extra buffer.
    loop {
        match reader.read_event() {
            Ok(Event::Start(e)) => {
                let child = parse_element(reader, &e)?;
                node.children.push(child);
            }
            Ok(Event::Text(e)) => {
                // Use new unescape() method and then convert the result into an owned String.
                let txt = e
                    .unescape()
                    .map_err(|e| e.to_string())?
                    .into_owned();
                text_content.push_str(&txt);
            }
            Ok(Event::Empty(e)) => {
                let tag_name = std::str::from_utf8(e.name().as_ref())
                    .map_err(|_| "Invalid UTF-8 in empty tag".to_string())?
                    .to_string();
                node.children.push(XmlNode {
                    name: tag_name,
                    value: None,
                    children: Vec::new(),
                });
            }
            Ok(Event::End(e)) => {
                // Verify matching end tag. Use as_ref() on QName.
                if e.name().as_ref() != start.name().as_ref() {
                    return Err("Mismatched closing tag".to_string());
                }
                break;
            }
            Ok(Event::Eof) => return Err("Unexpected end of file".to_string()),
            Err(e) => return Err(format!("Error parsing XML: {}", e)),
            _ => {} // skip comments, declarations, etc.
        }
    }

    if !text_content.trim().is_empty() {
        node.value = Some(text_content.trim().to_string());
    }
    Ok(node)
}

/// Parses the entire XML document into our intermediate tree representation.
fn parse_xml_to_tree(xml: &str) -> Result<XmlNode, String> {
    let mut reader = Reader::from_str(xml);
    reader.trim_text(true);

    loop {
        match reader.read_event() {
            Ok(Event::Start(e)) => {
                return parse_element(&mut reader, &e);
            }
            Ok(Event::Empty(e)) => {
                let name = std::str::from_utf8(e.name().as_ref())
                    .map_err(|_| "Invalid UTF-8 in tag".to_string())?
                    .to_string();
                return Ok(XmlNode {
                    name,
                    value: None,
                    children: Vec::new(),
                });
            }
            Ok(Event::Eof) => break,
            Err(e) => return Err(format!("XML parsing error: {}", e)),
            _ => {}
        }
    }
    Err("No root element found in XML".to_string())
}

/// Recursively builds the graph representation from the XmlNode tree.
/// A mutable counter assigns a unique ID to each node.
fn build_graph(
    node: &XmlNode,
    parent_id: Option<String>,
    depth: usize,
    counter: &mut usize,
    nodes: &mut Vec<GraphNode>,
    links: &mut Vec<GraphLink>,
) {
    let current_id = counter.to_string();
    *counter += 1;

    let is_leaf = node.children.is_empty();
    let value = if is_leaf {
        node.value.clone().unwrap_or_default()
    } else {
        format!("{} items", node.children.len())
    };

    let label = if depth == 0 {
        "Object".to_string()
    } else {
        node.name.clone()
    };

    nodes.push(GraphNode {
        id: current_id.clone(),
        label,
        value,
        depth,
        parent: parent_id.clone(),
        is_leaf,
    });

    if let Some(pid) = parent_id {
        links.push(GraphLink {
            source: pid,
            target: current_id.clone(),
        });
    }

    for child in &node.children {
        build_graph(child, Some(current_id.clone()), depth + 1, counter, nodes, links);
    }
}

/// Recursively builds the tree visualization from the XmlNode tree.
fn build_tree(node: &XmlNode) -> TreeNode {
    let children = if node.children.is_empty() {
        None
    } else {
        Some(node.children.iter().map(build_tree).collect())
    };

    // Only show the value for leaf nodes.
    let value = if node.children.is_empty() {
        node.value.clone()
    } else {
        None
    };

    TreeNode {
        name: node.name.clone(),
        value,
        children,
    }
}

/// Exports a function to process XML into the graph format.
/// Returns a JSON string representing the graph or an error if the XML is invalid.
#[wasm_bindgen]
pub fn process_xml_graph(xml: &str) -> Result<String, JsValue> {
    let root = parse_xml_to_tree(xml)
        .map_err(|e| JsValue::from_str(&format!("Parse error: {}", e)))?;

    let mut nodes = Vec::new();
    let mut links = Vec::new();
    let mut counter = 1_usize;
    build_graph(&root, None, 0, &mut counter, &mut nodes, &mut links);

    let graph = Graph { nodes, links };
    serde_json::to_string(&graph)
        .map_err(|e| JsValue::from_str(&format!("Serialization error: {}", e)))
}

/// Exports a function to process XML into the tree format.
/// Returns a JSON string representing the tree or an error if the XML is invalid.
#[wasm_bindgen]
pub fn process_xml_tree(xml: &str) -> Result<String, JsValue> {
    let root = parse_xml_to_tree(xml)
        .map_err(|e| JsValue::from_str(&format!("Parse error: {}", e)))?;
    let tree = build_tree(&root);
    serde_json::to_string(&tree)
        .map_err(|e| JsValue::from_str(&format!("Serialization error: {}", e)))
}
