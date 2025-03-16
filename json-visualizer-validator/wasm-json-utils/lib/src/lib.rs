use wasm_bindgen::prelude::*;
use serde_json::Value;
use serde_yaml;
use quick_xml::se::to_string as to_xml;
use csv:: Writer;
use std::collections::HashMap;
use std::collections::HashSet;
mod types;
mod visualization;

// export to js
#[wasm_bindgen]
pub fn json_to_yaml(json_str: &str) -> Result<String, JsValue> {
    let json_value: Value = serde_json::from_str(json_str).map_err(|e| JsValue::from_str(&e.to_string()))?;
    let yaml_string = serde_yaml::to_string(&json_value).map_err(|e| JsValue::from_str(&e.to_string()))?;
    Ok(yaml_string)
}


// json to xml

#[wasm_bindgen]
pub fn json_to_xml(json_str: &str) -> Result<String, JsValue> {
    // Parse the JSON string
    let json_value: Value = serde_json::from_str(json_str)
        .map_err(|e| JsValue::from_str(&format!("JSON parsing error: {}", e)))?;
    
    // Start building the XML with proper declaration and root element
    let mut xml_output = String::from("<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n");
    
    // Add root element
    match &json_value {
        Value::Object(_) => {
            xml_output.push_str("<root>\n");
            serialize_json_to_xml(&mut xml_output, &json_value, 2);
            xml_output.push_str("</root>");
        },
        // If the JSON is an array, we still need a root element
        Value::Array(_) => {
            xml_output.push_str("<root>\n");
            serialize_json_to_xml(&mut xml_output, &json_value, 2);
            xml_output.push_str("</root>");
        },
        // Handle primitive values (unlikely as root but supported)
        _ => {
            xml_output.push_str("<root>");
            serialize_primitive_to_xml(&mut xml_output, &json_value);
            xml_output.push_str("</root>");
        }
    }
    
    Ok(xml_output)
}

// Main serialization function that handles different JSON types
fn serialize_json_to_xml(output: &mut String, value: &Value, indent: usize) {
    let indent_str = " ".repeat(indent);
    
    match value {
        Value::Object(map) => {
            // For objects, create an element for each key-value pair
            for (key, val) in map {
                match val {
                    Value::Object(_) => {
                        // For nested objects, create a new element with nested content
                        output.push_str(&format!("{}<{}>\n", indent_str, escape_xml_tag(key)));
                        serialize_json_to_xml(output, val, indent + 2);
                        output.push_str(&format!("{}</{}>\n", indent_str, escape_xml_tag(key)));
                    },
                    Value::Array(_) => {
                        // For arrays, create an element that contains array items
                        output.push_str(&format!("{}<{}>\n", indent_str, escape_xml_tag(key)));
                        serialize_json_to_xml(output, val, indent + 2);
                        output.push_str(&format!("{}</{}>\n", indent_str, escape_xml_tag(key)));
                    },
                    // For primitive values, create a simple element
                    _ => {
                        output.push_str(&format!("{}<{}>", indent_str, escape_xml_tag(key)));
                        serialize_primitive_to_xml(output, val);
                        output.push_str(&format!("</{}>\n", escape_xml_tag(key)));
                    }
                }
            }
        },
        Value::Array(arr) => {
            // For arrays, create an item element for each array entry
            for (i, item) in arr.iter().enumerate() {
                match item {
                    Value::Object(_) => {
                        // Use "item" as the element name for objects in arrays
                        output.push_str(&format!("{}<item>\n", indent_str));
                        serialize_json_to_xml(output, item, indent + 2);
                        output.push_str(&format!("{}</item>\n", indent_str));
                    },
                    Value::Array(_) => {
                        // Use "item" as the element name for arrays in arrays
                        output.push_str(&format!("{}<item>\n", indent_str));
                        serialize_json_to_xml(output, item, indent + 2);
                        output.push_str(&format!("{}</item>\n", indent_str));
                    },
                    // For primitive values, create a simple item element
                    _ => {
                        output.push_str(&format!("{}<item>", indent_str));
                        serialize_primitive_to_xml(output, item);
                        output.push_str("</item>\n");
                    }
                }
            }
        },
        // This case should not happen as primitive values are handled in the caller
        _ => serialize_primitive_to_xml(output, value)
    }
}

// Helper function to serialize primitive JSON values
fn serialize_primitive_to_xml(output: &mut String, value: &Value) {
    match value {
        Value::Null => output.push_str("null"),
        Value::Bool(b) => output.push_str(&b.to_string()),
        Value::Number(n) => output.push_str(&n.to_string()),
        Value::String(s) => output.push_str(&escape_xml_content(s)),
        // These cases are handled in the main function and shouldn't reach here
        Value::Object(_) | Value::Array(_) => {}
    }
}

// Helper function to escape XML content
fn escape_xml_content(content: &str) -> String {
    content
        .replace("&", "&amp;")
        .replace("<", "&lt;")
        .replace(">", "&gt;")
        .replace("\"", "&quot;")
        .replace("'", "&apos;")
}

// Helper function to escape XML tag names
fn escape_xml_tag(tag: &str) -> String {
    // XML tags can't contain spaces or special characters
    // For simplicity, we'll replace problematic characters with underscores
    let mut result = String::new();
    
    // Ensure the tag starts with a letter or underscore (XML requirement)
    let first_char = tag.chars().next().unwrap_or('_');
    if !first_char.is_alphabetic() && first_char != '_' {
        result.push('_');
    }
    
    // Replace invalid characters with underscores
    for c in tag.chars() {
        if c.is_alphanumeric() || c == '_' || c == '-' || c == '.' {
            result.push(c);
        } else {
            result.push('_');
        }
    }
    
    result
}


// Json to CSV 

#[wasm_bindgen]
pub fn json_to_csv(json_str: &str) -> Result<String, JsValue> {
    // Parse the JSON string
    let json_value: Value = serde_json::from_str(json_str)
        .map_err(|e| JsValue::from_str(&format!("JSON parsing error: {}", e)))?;
    
    // CSV conversion only makes sense for arrays of objects
    match &json_value {
        Value::Array(arr) => {
            if arr.is_empty() {
                return Ok(String::from(""));
            }
            
            // Check if the array contains objects
            let contains_objects = arr.iter().any(|item| item.is_object());
            
            if contains_objects {
                convert_array_of_objects_to_csv(arr)
            } else {
                // For simple arrays, we'll create a single column CSV
                convert_simple_array_to_csv(arr)
            }
        },
        Value::Object(obj) => {
            // Single object - treat as a one-row table
            let mut headers = Vec::new();
            let mut values = Vec::new();
            
            for (key, value) in obj {
                headers.push(escape_csv_field(key));
                values.push(json_value_to_csv_field(value));
            }
            
            let mut csv = String::new();
            csv.push_str(&headers.join(","));
            csv.push_str("\n");
            csv.push_str(&values.join(","));
            
            Ok(csv)
        },
        // For primitive values, just return the value as a single cell
        _ => Ok(json_value_to_csv_field(&json_value))
    }
}

fn convert_array_of_objects_to_csv(arr: &Vec<Value>) -> Result<String, JsValue> {
    // Collect all possible field names across all objects
    let mut all_fields = HashSet::new();
    
    for item in arr {
        if let Value::Object(obj) = item {
            for key in obj.keys() {
                all_fields.insert(key.clone());
            }
        }
    }
    
    // No fields found - return error
    if all_fields.is_empty() {
        return Err(JsValue::from_str("No object fields found in array"));
    }
    
    // Sort fields for consistent output
    let mut field_list: Vec<String> = all_fields.into_iter().collect();
    field_list.sort();
    
    // Create CSV content
    let mut csv = String::new();
    
    // Add CSV header
    let headers: Vec<String> = field_list.iter()
        .map(|field| escape_csv_field(field))
        .collect();
    csv.push_str(&headers.join(","));
    
    // Add rows
    for item in arr {
        csv.push_str("\n");
        
        let row_values: Vec<String> = field_list.iter()
            .map(|field| {
                if let Value::Object(obj) = item {
                    if let Some(value) = obj.get(field) {
                        json_value_to_csv_field(value)
                    } else {
                        String::new() // Empty field if not present
                    }
                } else {
                    String::new() // Should not happen given our checks
                }
            })
            .collect();
        
        csv.push_str(&row_values.join(","));
    }
    
    Ok(csv)
}

fn convert_simple_array_to_csv(arr: &Vec<Value>) -> Result<String, JsValue> {
    // For simple arrays (not containing objects), create a single column
    let header = "value";
    let mut csv = String::from(header);
    
    for item in arr {
        csv.push_str("\n");
        csv.push_str(&json_value_to_csv_field(item));
    }
    
    Ok(csv)
}

fn json_value_to_csv_field(value: &Value) -> String {
    match value {
        Value::Null => String::new(),
        Value::Bool(b) => escape_csv_field(&b.to_string()),
        Value::Number(n) => escape_csv_field(&n.to_string()),
        Value::String(s) => escape_csv_field(s),
        Value::Array(arr) => {
            // For arrays inside objects, convert to string representation
            let items: Vec<String> = arr.iter()
                .map(|item| match item {
                    Value::String(s) => s.clone(),
                    _ => item.to_string()
                })
                .collect();
            escape_csv_field(&format!("[{}]", items.join(", ")))
        },
        Value::Object(obj) => {
            // For objects inside objects, convert to string representation
            let entries: Vec<String> = obj.iter()
                .map(|(k, v)| format!("{}:{}", k, match v {
                    Value::String(s) => s.clone(),
                    _ => v.to_string()
                }))
                .collect();
            escape_csv_field(&format!("{{{}}}", entries.join(", ")))
        }
    }
}

fn escape_csv_field(field: &str) -> String {
    // Check if we need to escape the field
    let needs_escaping = field.contains(',') || field.contains('"') || field.contains('\n') || field.contains('\r');
    
    if needs_escaping {
        // Replace double quotes with two double quotes and wrap in quotes
        let escaped = field.replace("\"", "\"\"");
        format!("\"{}\"", escaped)
    } else {
        field.to_string()
    }
}



// YAML to JSON conversion
#[wasm_bindgen]
pub fn yaml_to_json(yaml_str: &str) -> Result<String, JsValue> {
    let yaml_value: Value = serde_yaml::from_str(yaml_str)
        .map_err(|e| JsValue::from_str(&e.to_string()))?;
    
    let json_string = serde_json::to_string_pretty(&yaml_value)
        .map_err(|e| JsValue::from_str(&e.to_string()))?;
    
    Ok(json_string)
}

// YAML to XML conversion
#[wasm_bindgen]
pub fn yaml_to_xml(yaml_str: &str) -> Result<String, JsValue> {
    let yaml_value: Value = serde_yaml::from_str(yaml_str)
        .map_err(|e| JsValue::from_str(&e.to_string()))?;
    
    let xml_string = value_to_xml(&yaml_value, "root")
        .map_err(|e| JsValue::from_str(&e.to_string()))?;
    
    Ok(xml_string)
}

// YAML to CSV conversion with improved handling
#[wasm_bindgen]
pub fn yaml_to_csv(yaml_str: &str) -> Result<String, JsValue> {
    let yaml_value: Value = serde_yaml::from_str(yaml_str)
        .map_err(|e| JsValue::from_str(&e.to_string()))?;
    
    match &yaml_value {
        Value::Array(arr) => {
            // Check if it's an array of objects, which is ideal for CSV
            if arr.iter().all(|item| item.is_object()) && !arr.is_empty() {
                // Process array of objects (ideal case)
                value_to_csv(&yaml_value)
                    .map_err(|e| JsValue::from_str(&e.to_string()))
            } else if arr.iter().all(|item| item.is_array()) && !arr.is_empty() {
                // Handle array of arrays as rows and columns
                array_of_arrays_to_csv(arr)
                    .map_err(|e| JsValue::from_str(&e.to_string()))
            } else {
                // Simple array - convert to single column
                simple_array_to_csv(arr)
                    .map_err(|e| JsValue::from_str(&e.to_string()))
            }
        },
        Value::Object(obj) => {
            // Handle single object - convert to key-value pairs
            single_object_to_csv(obj)
                .map_err(|e| JsValue::from_str(&e.to_string()))
        },
        _ => {
            // Handle scalar values or other simple types
            Ok(format!("value\n{}", scalar_to_string(&yaml_value)))
        }
    }
}

// Helper function to convert Value to XML
fn value_to_xml(value: &Value, tag_name: &str) -> Result<String, String> {
    match value {
        Value::Null => Ok(format!("<{tag_name}></{tag_name}>")),
        Value::Bool(b) => Ok(format!("<{tag_name}>{}</{}>"
, if *b { "true" } else { "false" }, tag_name)),
        Value::Number(n) => Ok(format!("<{tag_name}>{}</{}>"
, n, tag_name)),
        Value::String(s) => {
            // Escape XML special characters
            let escaped = s.replace("&", "&amp;")
                          .replace("<", "&lt;")
                          .replace(">", "&gt;")
                          .replace("\"", "&quot;")
                          .replace("'", "&apos;");
            Ok(format!("<{tag_name}>{}</{}>"
, escaped, tag_name))
        },
        Value::Array(arr) => {
            let mut result = String::new();
            
            for (i, item) in arr.iter().enumerate() {
                let item_tag = format!("{}Item", tag_name);
                match value_to_xml(item, &item_tag) {
                    Ok(xml) => result.push_str(&xml),
                    Err(e) => return Err(e),
                }
            }
            
            Ok(format!("<{tag_name}>{}</{}>"
, result, tag_name))
        },
        Value::Object(map) => {
            let mut result = String::new();
            
            for (key, val) in map {
                match value_to_xml(val, key) {
                    Ok(xml) => result.push_str(&xml),
                    Err(e) => return Err(e),
                }
            }
            
            Ok(format!("<{tag_name}>{}</{}>"
, result, tag_name))
        }
    }
}

// Helper function to convert array of objects to CSV
fn value_to_csv(value: &Value) -> Result<String, String> {
    match value {
        Value::Array(arr) => {
            if arr.is_empty() {
                return Ok(String::new());
            }
            
            // Extract headers from all objects to ensure we capture all fields
            let mut all_headers = Vec::new();
            for item in arr {
                if let Value::Object(map) = item {
                    for key in map.keys() {
                        if !all_headers.contains(key) {
                            all_headers.push(key.clone());
                        }
                    }
                } else {
                    return Err("CSV conversion only supports arrays of objects".to_string());
                }
            }
            
            all_headers.sort(); // Sort headers for consistent output
            
            // Create header row
            let headers = all_headers.join(",");
            let mut result = headers + "\n";
            
            // Create data rows
            for item in arr {
                if let Value::Object(map) = item {
                    let row: Vec<String> = all_headers.iter().map(|header| {
                        match map.get(header) {
                            Some(Value::String(s)) => escape_csv_field_yml(s),
                            Some(Value::Number(n)) => n.to_string(),
                            Some(Value::Bool(b)) => b.to_string(),
                            Some(Value::Null) => String::new(),
                            Some(Value::Array(_)) => "[array]".to_string(),
                            Some(Value::Object(_)) => "[object]".to_string(),
                            None => String::new(),
                        }
                    }).collect();
                    
                    result.push_str(&row.join(","));
                    result.push('\n');
                }
            }
            
            Ok(result)
        },
        _ => Err("CSV conversion only supports arrays of objects".to_string()),
    }
}

// Convert array of arrays to CSV
fn array_of_arrays_to_csv(arr: &Vec<Value>) -> Result<String, String> {
    let mut result = String::new();
    
    for row_val in arr {
        if let Value::Array(row) = row_val {
            let csv_row: Vec<String> = row.iter()
                .map(|cell| escape_csv_field_yml(&scalar_to_string(cell)))
                .collect();
            
            result.push_str(&csv_row.join(","));
            result.push('\n');
        } else {
            return Err("Expected array of arrays".to_string());
        }
    }
    
    Ok(result)
}

// Convert simple array to CSV (one column)
fn simple_array_to_csv(arr: &Vec<Value>) -> Result<String, String> {
    let mut result = String::from("value\n");
    
    for item in arr {
        result.push_str(&escape_csv_field_yml(&scalar_to_string(item)));
        result.push('\n');
    }
    
    Ok(result)
}

// Convert a single object to CSV
fn single_object_to_csv(obj: &serde_json::Map<String, Value>) -> Result<String, String> {
    let mut result = String::from("key,value\n");
    
    let mut keys: Vec<&String> = obj.keys().collect();
    keys.sort(); // Sort keys for consistent output
    
    for key in keys {
        let value = obj.get(key).unwrap();
        result.push_str(&format!("{},{}\n", 
            escape_csv_field(key),
            escape_csv_field(&scalar_to_string(value))
        ));
    }
    
    Ok(result)
}

// Helper function to convert any scalar value to string representation
fn scalar_to_string(value: &Value) -> String {
    match value {
        Value::Null => String::new(),
        Value::Bool(b) => b.to_string(),
        Value::Number(n) => n.to_string(),
        Value::String(s) => s.clone(),
        Value::Array(_) => "[array]".to_string(),
        Value::Object(_) => "[object]".to_string(),
    }
}


// Helper function to escape CSV fields
fn escape_csv_field_yml(field: &str) -> String {
    if field.contains(',') || field.contains('\"') || field.contains('\n') {
        let escaped = field.replace("\"", "\"\"");
        format!("\"{}\"", escaped)
    } else {
        field.to_string()
    }
}

// XML to others below 


// Simple XML Node representation for our parser
struct XmlNode {
    name: String,
    attributes: HashMap<String, String>,
    text: String,
    children: Vec<XmlNode>,
}

// XML to JSON conversion
#[wasm_bindgen]
pub fn xml_to_json(xml_str: &str) -> Result<String, JsValue> {
    let parsed = parse_xml(xml_str)
        .map_err(|e| JsValue::from_str(&e))?;
    
    let json_string = xml_node_to_json(&parsed)
        .map_err(|e| JsValue::from_str(&e))?;
    
    Ok(json_string)
}

// XML to YAML conversion
#[wasm_bindgen]
pub fn xml_to_yaml(xml_str: &str) -> Result<String, JsValue> {
    let parsed = parse_xml(xml_str)
        .map_err(|e| JsValue::from_str(&e))?;
    
    let yaml_string = xml_node_to_yaml(&parsed)
        .map_err(|e| JsValue::from_str(&e))?;
    
    Ok(yaml_string)
}

// XML to CSV conversion
#[wasm_bindgen]
pub fn xml_to_csv(xml_str: &str) -> Result<String, JsValue> {
    let parsed = parse_xml(xml_str)
        .map_err(|e| JsValue::from_str(&e))?;
    
    let csv_string = xml_node_to_csv(&parsed)
        .map_err(|e| JsValue::from_str(&e))?;
    
    Ok(csv_string)
}

// Custom XML parser implementation
fn parse_xml(xml_str: &str) -> Result<XmlNode, String> {
    // This is a simplified XML parser for illustration
    // In a real implementation, you would need more robust parsing logic
    
    // Remove XML declaration if present
    let mut content = xml_str.trim();
    if content.starts_with("<?xml") {
        if let Some(pos) = content.find("?>") {
            content = &content[pos + 2..].trim();
        }
    }
    
    // Parse the root element
    parse_element(content)
}

fn parse_element(s: &str) -> Result<XmlNode, String> {
    // Check for empty or invalid input
    if s.is_empty() {
        return Err("Empty XML string".to_string());
    }
    
    // Start tag must begin with '<'
    if !s.starts_with('<') {
        return Err("Invalid XML: expected '<'".to_string());
    }
    
    // Find the end of the opening tag
    let mut i = 1;
    while i < s.len() && !s.chars().nth(i).unwrap_or(' ').is_whitespace() && s.chars().nth(i).unwrap_or(' ') != '>' {
        i += 1;
    }
    
    // Extract tag name
    let tag_name = &s[1..i];
    
    // Find where the opening tag ends
    let mut j = i;
    let mut in_quote = false;
    let mut quote_char = ' ';
    
    while j < s.len() {
        let c = s.chars().nth(j).unwrap_or(' ');
        
        if c == '"' || c == '\'' {
            if !in_quote {
                in_quote = true;
                quote_char = c;
            } else if c == quote_char {
                in_quote = false;
            }
        } else if c == '>' && !in_quote {
            break;
        }
        
        j += 1;
    }
    
    if j >= s.len() {
        return Err("Invalid XML: opening tag not closed".to_string());
    }
    
    // Parse attributes
    let attr_str = &s[i..j];
    let attributes = parse_attributes(attr_str);
    
    // Check if it's a self-closing tag
    let is_self_closing = s.chars().nth(j - 1).unwrap_or(' ') == '/';
    
    let mut node = XmlNode {
        name: tag_name.to_string(),
        attributes,
        text: String::new(),
        children: Vec::new(),
    };
    
    if is_self_closing {
        return Ok(node);
    }
    
    // Find the matching closing tag
    let content_start = j + 1;
    let closing_tag = format!("</{}>", tag_name);
    
    if let Some(content_end) = s[content_start..].find(&closing_tag) {
        let content = &s[content_start..content_start + content_end];
        
        // Parse child elements
        let mut remaining = content;
        
        while let Some(child_start) = remaining.find('<') {
            // Check if it's a comment, CDATA, or processing instruction
            if remaining[child_start..].starts_with("<!--") 
                || remaining[child_start..].starts_with("<![CDATA[")
                || remaining[child_start..].starts_with("<?") {
                // Skip these special elements for now
                let skip_marker = if remaining[child_start..].starts_with("<!--") {
                    "-->"
                } else if remaining[child_start..].starts_with("<![CDATA[") {
                    "]]>"
                } else {
                    "?>"
                };
                
                if let Some(end_pos) = remaining[child_start..].find(skip_marker) {
                    let end = child_start + end_pos + skip_marker.len();
                    remaining = &remaining[end..];
                    continue;
                } else {
                    return Err("Unclosed special element".to_string());
                }
            }
            
            // Extract text content before this child
            let text_content = remaining[..child_start].trim();
            if !text_content.is_empty() {
                node.text.push_str(text_content);
            }
            
            // Check if this is a closing tag
            if remaining[child_start + 1..].starts_with('/') {
                // This is a closing tag, which should be handled by the parent call
                break;
            }
            
            // Find where this child element ends
            let mut child_xml = &remaining[child_start..];
            let mut depth = 1;
            let mut pos = 1;
            
            while depth > 0 && pos < child_xml.len() {
                if child_xml[pos..].starts_with('<') {
                    if child_xml[pos + 1..].starts_with('/') {

                        if let Some(close_end) = child_xml[pos..].find('>') {
                        depth -= 1;
                        // skip past this entire closing tag
                        pos += close_end + 1;
                        continue;
                        }
                    } else if !child_xml[pos..].starts_with("<!--") 
                        && !child_xml[pos..].starts_with("<![CDATA[")
                        && !child_xml[pos..].starts_with("<?") {
                        // Regular opening tag
                        depth += 1;
                    }
                }
                pos += 1;
            }
            
            // Parse the child element
            let child_element = parse_element(&child_xml[..pos])?;
            node.children.push(child_element);
            
            // Move past this child element
            remaining = &remaining[child_start + pos..];
        }
        
        // Add any remaining text
        let final_text = remaining.trim();
        if !final_text.is_empty() {
            node.text.push_str(final_text);
        }
        
        Ok(node)
    } else {
        Err(format!("Closing tag not found for {}", tag_name))
    }
}

fn parse_attributes(s: &str) -> HashMap<String, String> {
    let mut attributes = HashMap::new();
    let mut i = 0;
    
    while i < s.len() {
        // Skip whitespace
        while i < s.len() && s.chars().nth(i).unwrap_or(' ').is_whitespace() {
            i += 1;
        }
        
        if i >= s.len() {
            break;
        }
        
        // Find attribute name
        let name_start = i;
        while i < s.len() && s.chars().nth(i).unwrap_or(' ') != '=' {
            i += 1;
        }
        
        if i >= s.len() {
            break;
        }
        
        let name = s[name_start..i].trim();
        
        // Skip the equals sign and find the quote
        i += 1;
        while i < s.len() && s.chars().nth(i).unwrap_or(' ') != '"' && s.chars().nth(i).unwrap_or(' ') != '\'' {
            i += 1;
        }
        
        if i >= s.len() {
            break;
        }
        
        let quote = s.chars().nth(i).unwrap_or('"');
        i += 1;
        
        // Find the end of the attribute value
        let value_start = i;
        while i < s.len() && s.chars().nth(i).unwrap_or(' ') != quote {
            i += 1;
        }
        
        if i >= s.len() {
            break;
        }
        
        let value = s[value_start..i].to_string();
        attributes.insert(name.to_string(), value);
        
        i += 1;
    }
    
    attributes
}

// Convert XML node to JSON string
fn xml_node_to_json(node: &XmlNode) -> Result<String, String> {
    let mut result = String::from("{");
    
    // Add tag name
    result.push_str(&format!("\"_name\": \"{}\"", node.name));
    
    // Add attributes
    if !node.attributes.is_empty() {
        result.push_str(", \"_attributes\": {");
        let mut first = true;
        for (key, value) in &node.attributes {
            if !first {
                result.push_str(", ");
            }
            result.push_str(&format!("\"{}\": \"{}\"", key, escape_json_string(value)));
            first = false;
        }
        result.push_str("}");
    }
    
    // Add text content if any
    if !node.text.is_empty() {
        result.push_str(&format!(", \"_text\": \"{}\"", escape_json_string(&node.text)));
    }
    
    // Add children
    if !node.children.is_empty() {
        // Group children by tag name
        let mut child_map: HashMap<String, Vec<&XmlNode>> = HashMap::new();
        for child in &node.children {
            child_map.entry(child.name.clone())
                .or_insert_with(Vec::new)
                .push(child);
        }
        
        for (child_name, children) in child_map {
            if children.len() == 1 {
                // Single child
                result.push_str(&format!(", \"{}\": ", child_name));
                let child_json = xml_node_to_json(children[0])?;
                result.push_str(&child_json);
            } else {
                // Multiple children with same name become an array
                result.push_str(&format!(", \"{}\": [", child_name));
                for (i, child) in children.iter().enumerate() {
                    if i > 0 {
                        result.push_str(", ");
                    }
                    let child_json = xml_node_to_json(child)?;
                    result.push_str(&child_json);
                }
                result.push_str("]");
            }
        }
    }
    
    result.push_str("}");
    Ok(result)
}

// Convert XML node to YAML string
fn xml_node_to_yaml(node: &XmlNode) -> Result<String, String> {
    // We'll implement a simple YAML serializer
    let mut result = String::new();
    yaml_serialize_node(node, &mut result, 0)?;
    Ok(result)
}

fn yaml_serialize_node(node: &XmlNode, result: &mut String, indent: usize) -> Result<(), String> {
    let indent_str = " ".repeat(indent);
    
    // Add tag name
    result.push_str(&format!("{}{}:\n", indent_str, node.name));
    
    let child_indent = indent + 2;
    let child_indent_str = " ".repeat(child_indent);
    
    // Add attributes
    if !node.attributes.is_empty() {
        result.push_str(&format!("{}attributes:\n", child_indent_str));
        let attr_indent = child_indent + 2;
        let attr_indent_str = " ".repeat(attr_indent);
        
        for (key, value) in &node.attributes {
            result.push_str(&format!("{}{}: {}\n", attr_indent_str, key, escape_yaml_string(value)));
        }
    }
    
    // Add text content if any
    if !node.text.is_empty() {
        result.push_str(&format!("{}text: {}\n", child_indent_str, escape_yaml_string(&node.text)));
    }
    
    // Add children
    if !node.children.is_empty() {
        result.push_str(&format!("{}children:\n", child_indent_str));
        
        for child in &node.children {
            yaml_serialize_node(child, result, child_indent + 2)?;
        }
    }
    
    Ok(())
}

// Convert XML node to CSV string
fn xml_node_to_csv(node: &XmlNode) -> Result<String, String> {
    // For CSV conversion, we need to extract tabular data
    // This is a very simplified approach that works best for XML representing tables
    
    let mut headers = Vec::new();
    let mut records = Vec::new();
    
    // Try to extract table-like data
    extract_csv_data(node, &mut headers, &mut records)?;
    
    if headers.is_empty() {
        return Err("Could not extract tabular data from XML".to_string());
    }
    
    // Generate CSV string
    let mut result = String::new();
    
    // Add headers
    for (i, header) in headers.iter().enumerate() {
        if i > 0 {
            result.push(',');
        }
        result.push_str(&escape_csv_field(header));
    }
    result.push('\n');
    
    // Add records
    for record in records {
        for (i, field) in record.iter().enumerate() {
            if i > 0 {
                result.push(',');
            }
            result.push_str(&escape_csv_field(field));
        }
        result.push('\n');
    }
    
    Ok(result)
}

fn extract_csv_data(node: &XmlNode, headers: &mut Vec<String>, records: &mut Vec<Vec<String>>) -> Result<(), String> {
    // This is a simplified approach - we assume:
    // 1. The node contains child elements that represent rows
    // 2. Each row contains child elements that represent cells
    // 3. The cell element names will be used as column headers
    
    // Try to identify row elements - assume they are immediate children with the same name
    let mut child_counts: HashMap<String, usize> = HashMap::new();
    
    for child in &node.children {
        *child_counts.entry(child.name.clone()).or_insert(0) += 1;
    }
    
    // Find the child element name that appears most frequently - likely the row elements
    let mut max_count = 0;
    let mut row_element_name = String::new();
    
    for (name, count) in child_counts {
        if count > max_count {
            max_count = count;
            row_element_name = name;
        }
    }
    
    if max_count <= 1 {
        // No repeating elements found, try a different approach
        // If node has text directly, treat it as a single record
        if !node.text.trim().is_empty() {
            headers.push("value".to_string());
            records.push(vec![node.text.trim().to_string()]);
            return Ok(());
        }
        
        // Otherwise, treat each child as a column
        for child in &node.children {
            headers.push(child.name.clone());
            if records.is_empty() {
                records.push(Vec::new());
            }
            records[0].push(child.text.trim().to_string());
        }
        
        return Ok(());
    }
    
    // Process row elements
    let row_elements: Vec<&XmlNode> = node.children.iter()
        .filter(|child| child.name == row_element_name)
        .collect();
    
    // Extract column headers from the first row
    if !row_elements.is_empty() {
        let first_row = &row_elements[0];
        
        // Use child element names as headers
        for cell in &first_row.children {
            headers.push(cell.name.clone());
        }
        
        // If no headers found, try attributes
        if headers.is_empty() {
            for (attr_name, _) in &first_row.attributes {
                headers.push(attr_name.clone());
            }
        }
        
        // Process each row
        for row in &row_elements {
            let mut record = Vec::new();
            
            // Handle cell elements
            if !headers.is_empty() && headers[0] != "value" {
                // Use indices instead of consuming iterator
                for i in 0..headers.len() {
                    let header = &headers[i];
                    // Find the cell with matching tag name
                    let cell_value = row.children.iter()
                        .find(|child| child.name == *header)
                        .map(|child| child.text.trim().to_string())
                        .unwrap_or_default();
                    
                    record.push(cell_value);
                }
            } else {
                // Handle attribute-based rows
                // Use indices instead of consuming iterator
                for i in 0..headers.len() {
                    let header = &headers[i];
                    let attr_value = row.attributes.get(header)
                        .cloned()
                        .unwrap_or_default();
                    
                    record.push(attr_value);
                }
            }
            
            if !record.is_empty() {
                records.push(record);
            }
        }
    }
    
    Ok(())
}

// Helper functions for escaping special characters in different formats
fn escape_json_string(s: &str) -> String {
    let mut result = String::with_capacity(s.len() + 2);
    
    for c in s.chars() {
        match c {
            '"' => result.push_str("\\\""),
            '\\' => result.push_str("\\\\"),
            '\n' => result.push_str("\\n"),
            '\r' => result.push_str("\\r"),
            '\t' => result.push_str("\\t"),
            '\u{0008}' => result.push_str("\\b"),
            '\u{000C}' => result.push_str("\\f"),
            _ => result.push(c),
        }
    }
    
    result
}

fn escape_yaml_string(s: &str) -> String {
    // Simple YAML escaping, add quotes if needed
    if s.contains('\n') || s.contains(':') || s.contains('"') {
        format!("\"{}\"", s.replace('"', "\\\""))
    } else if s.is_empty() {
        "\"\"".to_string()
    } else {
        s.to_string()
    }
}


// CSV to other 

#[wasm_bindgen]
pub fn csv_to_json(csv_str: &str) -> Result<String, JsValue> {
    // Parse CSV
    let mut reader = csv::ReaderBuilder::new()
        .has_headers(true)
        .from_reader(csv_str.as_bytes());
    
    // Get headers
    let headers = match reader.headers() {
        Ok(headers) => headers.clone(),
        Err(e) => return Err(JsValue::from_str(&e.to_string())),
    };
    
    // Create a Vec to store our rows
    let mut rows = Vec::new();
    
    // Iterate through records
    for result in reader.records() {
        let record = match result {
            Ok(record) => record,
            Err(e) => return Err(JsValue::from_str(&e.to_string())),
        };
        
        // Create a map for this row
        let mut row_map = std::collections::BTreeMap::new();
        
        // Add fields to map
        for (i, field) in record.iter().enumerate() {
            if i < headers.len() {
                row_map.insert(headers[i].to_string(), field.to_string());
            }
        }
        
        // Add row to rows
        rows.push(row_map);
    }
    
    // Convert to JSON
    match serde_json::to_string_pretty(&rows) {
        Ok(json) => Ok(json),
        Err(e) => Err(JsValue::from_str(&e.to_string())),
    }
}

#[wasm_bindgen]
pub fn csv_to_yaml(csv_str: &str) -> Result<String, JsValue> {
    // First convert CSV to a data structure
    let mut reader = csv::ReaderBuilder::new()
        .has_headers(true)
        .from_reader(csv_str.as_bytes());
    
    // Get headers
    let headers = match reader.headers() {
        Ok(headers) => headers.clone(),
        Err(e) => return Err(JsValue::from_str(&e.to_string())),
    };
    
    // Create a Vec to store our rows
    let mut rows = Vec::new();
    
    // Iterate through records
    for result in reader.records() {
        let record = match result {
            Ok(record) => record,
            Err(e) => return Err(JsValue::from_str(&e.to_string())),
        };
        
        // Create a map for this row
        let mut row_map = std::collections::BTreeMap::new();
        
        // Add fields to map
        for (i, field) in record.iter().enumerate() {
            if i < headers.len() {
                row_map.insert(headers[i].to_string(), field.to_string());
            }
        }
        
        // Add row to rows
        rows.push(row_map);
    }
    
    // Convert to YAML
    match serde_yaml::to_string(&rows) {
        Ok(yaml) => Ok(yaml),
        Err(e) => Err(JsValue::from_str(&e.to_string())),
    }
}

#[wasm_bindgen]
pub fn csv_to_xml(csv_str: &str) -> Result<String, JsValue> {
    // Parse CSV
    let mut reader = csv::ReaderBuilder::new()
        .has_headers(true)
        .from_reader(csv_str.as_bytes());
    
    // Get headers
    let headers = match reader.headers() {
        Ok(headers) => headers.clone(),
        Err(e) => return Err(JsValue::from_str(&e.to_string())),
    };
    
    // Start building XML output
    let mut xml_output = String::from("<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n<root>\n");
    
    // Iterate through records
    for result in reader.records() {
        let record = match result {
            Ok(record) => record,
            Err(e) => return Err(JsValue::from_str(&e.to_string())),
        };
        
        // Add row element
        xml_output.push_str("  <row>\n");
        
        // Add fields to row
        for (i, field) in record.iter().enumerate() {
            if i < headers.len() {
                let field_name = escape_xml(&headers[i]);
                let field_value = escape_xml(field);
                xml_output.push_str(&format!("    <{}>{}</{}>\n", field_name, field_value, field_name));
            }
        }
        
        // Close row element
        xml_output.push_str("  </row>\n");
    }
    
    // Finish XML document
    xml_output.push_str("</root>");
    
    Ok(xml_output)
}

// Helper function to escape XML special characters
fn escape_xml(s: &str) -> String {
    s.replace("&", "&amp;")
     .replace("<", "&lt;")
     .replace(">", "&gt;")
     .replace("\"", "&quot;")
     .replace("'", "&apos;")
}

