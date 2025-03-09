use wasm_bindgen::prelude::*;
use serde_json::Value;
use serde_yaml;
use quick_xml::se::to_string as to_xml;
use csv:: Writer;
use std::collections::HashMap;
use std::collections::HashSet;

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

