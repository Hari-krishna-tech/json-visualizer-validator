use wasm_bindgen::prelude::*;
use serde_json::{Value, Error as JsonError, Map};
use std::collections::HashMap;
use std::collections::HashSet;
use serde_yaml;

// Helper function to parse JSON
fn parse_json(json_str: &str) -> Result<Value, String> {
    serde_json::from_str::<Value>(json_str)
        .map_err(|e| format!("JSON parsing error: {}", e))
}

#[wasm_bindgen]
pub fn json_to_typescript(json_str: &str) -> Result<String, JsValue> {
    let parsed = parse_json(json_str)
        .map_err(|e| JsValue::from_str(&e))?;
    
    let ts_type = generate_typescript_type(&parsed, "RootType")
        .map_err(|e| JsValue::from_str(&e))?;
    
    Ok(ts_type)
}

#[wasm_bindgen]
pub fn json_to_java(json_str: &str) -> Result<String, JsValue> {
    let parsed = parse_json(json_str)
        .map_err(|e| JsValue::from_str(&e))?;
    
    let java_interface = generate_java_interface(&parsed, "RootClass")
        .map_err(|e| JsValue::from_str(&e))?;
    
    Ok(java_interface)
}

#[wasm_bindgen]
pub fn json_to_golang(json_str: &str) -> Result<String, JsValue> {
    let parsed = parse_json(json_str)
        .map_err(|e| JsValue::from_str(&e))?;
    
    let golang_struct = generate_golang_struct(&parsed, "RootType")
        .map_err(|e| JsValue::from_str(&e))?;
    
    Ok(golang_struct)
}

#[wasm_bindgen]
pub fn json_to_python(json_str: &str) -> Result<String, JsValue> {
    let parsed = parse_json(json_str)
        .map_err(|e| JsValue::from_str(&e))?;
    
    let python_class = generate_python_class(&parsed, "RootClass")
        .map_err(|e| JsValue::from_str(&e))?;
    
    Ok(python_class)
}

// Helper functions for type conversion

fn generate_typescript_type(value: &Value, type_name: &str) -> Result<String, String> {
    match value {
        Value::Object(map) => {
            let mut fields = Vec::new();
            let mut nested_types = Vec::new();
            
            for (key, val) in map {
                match val {
                    Value::Object(_) => {
                        let nested_type_name = format!("{}_{}", type_name, to_pascal_case(key));
                        let nested_type = generate_typescript_type(val, &nested_type_name)?;
                        nested_types.push(nested_type);
                        fields.push(format!("  {}: {};", key, nested_type_name));
                    },
                    Value::Array(arr) => {
                        if let Some(first) = arr.first() {
                            if let Value::Object(_) = first {
                                let nested_type_name = format!("{}_{}", type_name, to_pascal_case(key));
                                let nested_type = generate_typescript_type(first, &nested_type_name)?;
                                nested_types.push(nested_type);
                                fields.push(format!("  {}: {}[];", key, nested_type_name));
                            } else {
                                let ts_type = json_value_to_ts_type(first);
                                fields.push(format!("  {}: {}[];", key, ts_type));
                            }
                        } else {
                            fields.push(format!("  {}: any[];", key));
                        }
                    },
                    _ => {
                        let ts_type = json_value_to_ts_type(val);
                        fields.push(format!("  {}: {};", key, ts_type));
                    }
                }
            }
            
            let interface = format!("interface {} {{\n{}\n}}", type_name, fields.join("\n"));
            
            if nested_types.is_empty() {
                Ok(interface)
            } else {
                Ok(format!("{}\n\n{}", nested_types.join("\n\n"), interface))
            }
        },
        _ => Err("Root JSON value must be an object".to_string())
    }
}

fn generate_java_interface(value: &Value, class_name: &str) -> Result<String, String> {
    match value {
        Value::Object(map) => {
            let mut fields = Vec::new();
            let mut getters = Vec::new();
            let mut nested_classes = Vec::new();
            
            for (key, val) in map {
                match val {
                    Value::Object(_) => {
                        let nested_class_name = format!("{}", to_pascal_case(key));
                        let nested_class = generate_java_interface(val, &nested_class_name)?;
                        nested_classes.push(nested_class);
                        
                        let field_type = nested_class_name;
                        let field_name = to_camel_case(key);
                        fields.push(format!("    private {} {};", field_type, field_name));
                        
                        let getter = format!(
                            "    public {} get{}() {{\n        return this.{};\n    }}",
                            field_type, to_pascal_case(key), field_name
                        );
                        getters.push(getter);
                    },
                    Value::Array(arr) => {
                        if let Some(first) = arr.first() {
                            if let Value::Object(_) = first {
                                let nested_class_name = format!("{}", to_pascal_case(key));
                                let nested_class = generate_java_interface(first, &nested_class_name)?;
                                nested_classes.push(nested_class);
                                
                                let field_type = format!("List<{}>", nested_class_name);
                                let field_name = to_camel_case(key);
                                fields.push(format!("    private {} {};", field_type, field_name));
                                
                                let getter = format!(
                                    "    public {} get{}() {{\n        return this.{};\n    }}",
                                    field_type, to_pascal_case(key), field_name
                                );
                                getters.push(getter);
                            } else {
                                let java_type = json_value_to_java_type(first);
                                let field_type = format!("List<{}>", java_type);
                                let field_name = to_camel_case(key);
                                fields.push(format!("    private {} {};", field_type, field_name));
                                
                                let getter = format!(
                                    "    public {} get{}() {{\n        return this.{};\n    }}",
                                    field_type, to_pascal_case(key), field_name
                                );
                                getters.push(getter);
                            }
                        } else {
                            let field_type = "List<Object>";
                            let field_name = to_camel_case(key);
                            fields.push(format!("    private {} {};", field_type, field_name));
                            
                            let getter = format!(
                                "    public {} get{}() {{\n        return this.{};\n    }}",
                                field_type, to_pascal_case(key), field_name
                            );
                            getters.push(getter);
                        }
                    },
                    _ => {
                        let java_type = json_value_to_java_type(val);
                        let field_name = to_camel_case(key);
                        fields.push(format!("    private {} {};", java_type, field_name));
                        
                        let getter = format!(
                            "    public {} get{}() {{\n        return this.{};\n    }}",
                            java_type, to_pascal_case(key), field_name
                        );
                        getters.push(getter);
                    }
                }
            }
            
            let class_content = format!(
                "import java.util.List;\nimport java.util.Map;\n\npublic class {} {{\n{}\n\n{}\n}}",
                class_name, fields.join("\n"), getters.join("\n\n")
            );
            
            if nested_classes.is_empty() {
                Ok(class_content)
            } else {
                Ok(format!("{}\n\n{}", class_content, nested_classes.join("\n\n")))
            }
        },
        _ => Err("Root JSON value must be an object".to_string())
    }
}

fn generate_golang_struct(value: &Value, struct_name: &str) -> Result<String, String> {
    match value {
        Value::Object(map) => {
            let mut fields = Vec::new();
            let mut nested_structs = Vec::new();
            
            for (key, val) in map {
                let field_name = to_pascal_case(key);
                
                match val {
                    Value::Object(_) => {
                        let nested_struct_name = format!("{}", field_name);
                        let nested_struct = generate_golang_struct(val, &nested_struct_name)?;
                        nested_structs.push(nested_struct);
                        
                        fields.push(format!("\t{} {} `json:\"{}\"`", field_name, nested_struct_name, key));
                    },
                    Value::Array(arr) => {
                        if let Some(first) = arr.first() {
                            if let Value::Object(_) = first {
                                let nested_struct_name = format!("{}", field_name);
                                let nested_struct = generate_golang_struct(first, &nested_struct_name)?;
                                nested_structs.push(nested_struct);
                                
                                fields.push(format!("\t{} []{} `json:\"{}\"`", field_name, nested_struct_name, key));
                            } else {
                                let go_type = json_value_to_go_type(first);
                                fields.push(format!("\t{} []{}  `json:\"{}\"`", field_name, go_type, key));
                            }
                        } else {
                            fields.push(format!("\t{} []interface{{}}  `json:\"{}\"`", field_name, key));
                        }
                    },
                    _ => {
                        let go_type = json_value_to_go_type(val);
                        fields.push(format!("\t{} {}  `json:\"{}\"`", field_name, go_type, key));
                    }
                }
            }
            
            let struct_def = format!("type {} struct {{\n{}\n}}", struct_name, fields.join("\n"));
            
            if nested_structs.is_empty() {
                Ok(struct_def)
            } else {
                Ok(format!("{}\n\n{}", nested_structs.join("\n\n"), struct_def))
            }
        },
        _ => Err("Root JSON value must be an object".to_string())
    }
}

fn generate_python_class(value: &Value, class_name: &str) -> Result<String, String> {
    match value {
        Value::Object(map) => {
            let mut fields = Vec::new();
            let mut init_fields = Vec::new();
            let mut nested_classes = Vec::new();
            let mut imports = vec!["from dataclasses import dataclass", "from typing import List, Dict, Optional, Any"];
            
            for (key, val) in map {
                let field_name = key;
                
                match val {
                    Value::Object(_) => {
                        let nested_class_name = to_pascal_case(key);
                        let nested_class = generate_python_class(val, &nested_class_name)?;
                        nested_classes.push(nested_class);
                        
                        fields.push(format!("    {}: '{}'", field_name, nested_class_name));
                        init_fields.push(format!("        self.{} = {}", field_name, field_name));
                    },
                    Value::Array(arr) => {
                        if let Some(first) = arr.first() {
                            if let Value::Object(_) = first {
                                let nested_class_name = to_pascal_case(key);
                                let nested_class = generate_python_class(first, &nested_class_name)?;
                                nested_classes.push(nested_class);
                                
                                fields.push(format!("    {}: List['{}']", field_name, nested_class_name));
                                init_fields.push(format!("        self.{} = {}", field_name, field_name));
                            } else {
                                let py_type = json_value_to_python_type(first);
                                fields.push(format!("    {}: List[{}]", field_name, py_type));
                                init_fields.push(format!("        self.{} = {}", field_name, field_name));
                            }
                        } else {
                            fields.push(format!("    {}: List[Any]", field_name));
                            init_fields.push(format!("        self.{} = {}", field_name, field_name));
                        }
                    },
                    _ => {
                        let py_type = json_value_to_python_type(val);
                        fields.push(format!("    {}: {}", field_name, py_type));
                        init_fields.push(format!("        self.{} = {}", field_name, field_name));
                    }
                }
            }
            
            let class_def = format!(
                "@dataclass\nclass {}:\n{}\n\n    def __init__(self, {}):\n{}",
                class_name,
                fields.join("\n"),
                map.keys().map(|k| format!("{}", k)).collect::<Vec<_>>().join(", "),
                init_fields.join("\n")
            );
            
            let imports_str = imports.join("\n");
            
            if nested_classes.is_empty() {
                Ok(format!("{}\n\n{}", imports_str, class_def))
            } else {
                Ok(format!("{}\n\n{}\n\n{}", imports_str, nested_classes.join("\n\n"), class_def))
            }
        },
        _ => Err("Root JSON value must be an object".to_string())
    }
}

// Helper functions for type conversion

fn json_value_to_ts_type(value: &Value) -> &str {
    match value {
        Value::Null => "null",
        Value::Bool(_) => "boolean",
        Value::Number(n) => {
            if n.is_i64() || n.is_u64() {
                "number"
            } else {
                "number"
            }
        },
        Value::String(_) => "string",
        Value::Array(_) => "any[]",
        Value::Object(_) => "object",
    }
}

fn json_value_to_java_type(value: &Value) -> &str {
    match value {
        Value::Null => "Object",
        Value::Bool(_) => "Boolean",
        Value::Number(n) => {
            if n.is_i64() || n.is_u64() {
                "Long"
            }
            else {
                "Double"
            }
        },
        Value::String(_) => "String",
        Value::Array(_) => "List<Object>",
        Value::Object(_) => "Object",
    }
}

fn json_value_to_go_type(value: &Value) -> &str {
    match value {
        Value::Null => "interface{}",
        Value::Bool(_) => "bool",
        Value::Number(n) => {
            if n.is_i64() {
                "int64"
            } else if n.is_u64() {
                "uint64"
            } else {
                "float64"
            }
        },
        Value::String(_) => "string",
        Value::Array(_) => "[]interface{}",
        Value::Object(_) => "map[string]interface{}",
    }
}

fn json_value_to_python_type(value: &Value) -> &str {
    match value {
        Value::Null => "None",
        Value::Bool(_) => "bool",
        Value::Number(n) => {
            if n.is_i64() || n.is_u64() {
                "int"
            } else {
                "float"
            }
        },
        Value::String(_) => "str",
        Value::Array(_) => "List[Any]",
        Value::Object(_) => "Dict[str, Any]",
    }
}

// Helper functions for string casing

fn to_pascal_case(s: &str) -> String {
    let mut result = String::new();
    let mut capitalize_next = true;
    
    for c in s.chars() {
        if c == '_' || c == '-' || c == ' ' {
            capitalize_next = true;
        } else if capitalize_next {
            result.push(c.to_ascii_uppercase());
            capitalize_next = false;
        } else {
            result.push(c);
        }
    }
    
    result
}

fn to_camel_case(s: &str) -> String {
    let pascal = to_pascal_case(s);
    if pascal.is_empty() {
        return pascal;
    }
    
    let mut result = String::new();
    result.push(pascal.chars().next().unwrap().to_ascii_lowercase());
    result.push_str(&pascal[1..]);
    
    result
}



// Main YAML parsing function
fn parse_yaml(yaml_str: &str) -> Result<Value, String> {
    serde_yaml::from_str(yaml_str)
        .map_err(|e| format!("Invalid YAML: {}", e))
}

// TypeScript conversion
#[wasm_bindgen]
pub fn yaml_to_typescript(yaml_str: &str) -> Result<String, JsValue> {
    let parsed = parse_yaml(yaml_str)
        .map_err(|e| JsValue::from_str(&e))?;
    
    let ts_type = generate_typescript_type_yaml(&parsed, "RootType")
        .map_err(|e| JsValue::from_str(&e))?;
    
    Ok(ts_type)
}

fn generate_typescript_type_yaml(value: &Value, type_name: &str) -> Result<String, String> {
    match value {
        Value::Object(map) => {
            let mut fields = Vec::new();
            for (key, val) in map {
                let field_type = match val {
                    Value::Null => "null | undefined".to_string(),
                    Value::Bool(_) => "boolean".to_string(),
                    Value::Number(n) => {
                        if n.is_i64() || n.is_u64() {
                            "number".to_string()
                        } else {
                            "number".to_string()
                        }
                    },
                    Value::String(_) => "string".to_string(),
                    Value::Array(items) => {
                        if items.is_empty() {
                            "any[]".to_string()
                        } else {
                            let item_type = generate_typescript_type(&items[0], &format!("{}Item", key))?;
                            if item_type.contains("interface") || item_type.contains("type") {
                                let nested_type_name = format!("{}Item", key);
                                fields.push(item_type);
                                format!("{}[]", nested_type_name)
                            } else {
                                format!("{}[]", item_type.trim())
                            }
                        }
                    },
                    Value::Object(_) => {
                        let nested_type_name = format!("{}{}", capitalize(key), "Type");
                        let nested_type = generate_typescript_type(val, &nested_type_name)?;
                        fields.push(nested_type);
                        nested_type_name
                    },
                };
                fields.push(format!("  {}: {};", key, field_type));
            }
            
            // Render the interface
            let mut result = format!("interface {} {{\n", type_name);
            for field in fields {
                if !field.starts_with("interface") && !field.starts_with("type") {
                    result.push_str(&field);
                    result.push('\n');
                } else {
                    // Add nested types before the main interface
                    result = format!("{}\n{}", field, result);
                }
            }
            result.push_str("}\n");
            Ok(result)
        },
        Value::Array(items) => {
            if items.is_empty() {
                Ok(format!("type {} = any[];\n", type_name))
            } else {
                let item_type = generate_typescript_type(&items[0], &format!("{}Item", type_name))?;
                if item_type.contains("interface") || item_type.contains("type") {
                    let array_type = format!("type {} = {}Item[];\n", type_name, type_name);
                    Ok(format!("{}\n{}", item_type, array_type))
                } else {
                    Ok(format!("type {} = {}[];\n", type_name, item_type.trim()))
                }
            }
        },
        Value::String(_) => Ok("string".to_string()),
        Value::Number(_) => Ok("number".to_string()),
        Value::Bool(_) => Ok("boolean".to_string()),
        Value::Null => Ok("null | undefined".to_string()),
    }
} 

// Java conversion
#[wasm_bindgen]
pub fn yaml_to_java(yaml_str: &str) -> Result<String, JsValue> {
    let parsed = parse_yaml(yaml_str)
        .map_err(|e| JsValue::from_str(&e))?;
    
    let java_code = generate_java_interface_yaml(&parsed, "RootType")
        .map_err(|e| JsValue::from_str(&e))?;
    
    Ok(java_code)
}

fn generate_java_interface_yaml(value: &Value, type_name: &str) -> Result<String, String> {
    match value {
        Value::Object(map) => {
            let mut fields = Vec::new();
            let mut nested_classes = Vec::new();
            
            for (key, val) in map {
                let (field_type, nested) = match val {
                    Value::Null => ("Object".to_string(), None),
                    Value::Bool(_) => ("Boolean".to_string(), None),
                    Value::Number(n) => {
                        if n.is_i64() || n.is_u64() {
                            ("Long".to_string(), None)
                        } else {
                            ("Double".to_string(), None)
                        }
                    },
                    Value::String(_) => ("String".to_string(), None),
                    Value::Array(items) => {
                        if items.is_empty() {
                            ("List<Object>".to_string(), None)
                        } else {
                            let nested_type_name = format!("{}{}", capitalize(key), "Type");
                            let (item_type, nested_class) = match &items[0] {
                                Value::Object(_) => {
                                    let nested = generate_java_interface(&items[0], &nested_type_name)?;
                                    (nested_type_name.clone(), Some(nested))
                                },
                                _ => {
                                    let simple_type = java_type_for_value(&items[0])?;
                                    (simple_type, None)
                                }
                            };
                            
                            if let Some(class_def) = nested_class {
                                nested_classes.push(class_def);
                            }
                            
                            (format!("List<{}>", item_type), None)
                        }
                    },
                    Value::Object(_) => {
                        let nested_type_name = format!("{}{}", capitalize(key), "Type");
                        let nested_class = generate_java_interface(val, &nested_type_name)?;
                        nested_classes.push(nested_class);
                        (nested_type_name.clone(), None)
                    },
                };
                
                if let Some(nested) = nested {
                    nested_classes.push(nested);
                }
                
                // Create getter method name using Java naming conventions
                let getter_prefix = if field_type == "Boolean" { "is" } else { "get" };
                let getter_name = format!("{}{}", getter_prefix, capitalize(key));
                
                fields.push(format!("    {} {}();\n", field_type, getter_name));
            }
            
            // Render the interface
            let mut result = String::new();
            
            // Add nested classes first
            for nested in nested_classes {
                result.push_str(&nested);
                result.push_str("\n");
            }
            
            // Add the main interface
            result.push_str(&format!("public interface {} {{\n", type_name));
            for field in fields {
                result.push_str(&field);
            }
            result.push_str("}\n");
            
            Ok(result)
        },
        Value::Array(items) => {
            if items.is_empty() {
                Ok(format!("public interface {} {{\n    List<Object> getItems();\n}}\n", type_name))
            } else {
                let item_type_name = format!("{}Item", type_name);
                let nested_class = generate_java_interface(&items[0], &item_type_name)?;
                let mut result = nested_class;
                result.push_str(&format!("\npublic interface {} {{\n    List<{}> getItems();\n}}\n", type_name, item_type_name));
                Ok(result)
            }
        },
        _ => Err("Cannot generate Java interface from non-object YAML".to_string()),
    }
}

fn java_type_for_value(value: &Value) -> Result<String, String> {
    match value {
        Value::Null => Ok("Object".to_string()),
        Value::Bool(_) => Ok("Boolean".to_string()),
        Value::Number(n) => {
            if n.is_i64() || n.is_u64() {
                Ok("Long".to_string())
            } else {
                Ok("Double".to_string())
            }
        },
        Value::String(_) => Ok("String".to_string()),
        Value::Array(_) => Ok("List<Object>".to_string()),
        Value::Object(_) => Err("Need to generate a nested class for object types".to_string()),
    }
}

// Python conversion
#[wasm_bindgen]
pub fn yaml_to_python(yaml_str: &str) -> Result<String, JsValue> {
    let parsed = parse_yaml(yaml_str)
        .map_err(|e| JsValue::from_str(&e))?;
    
    let python_code = generate_python_class_yaml(&parsed, "RootType")
        .map_err(|e| JsValue::from_str(&e))?;
    
    Ok(python_code)
}

fn generate_python_class_yaml(value: &Value, class_name: &str) -> Result<String, String> {
   match value {
        Value::Object(map) => {
            let mut fields = Vec::new();
            let mut init_params = Vec::new();
            let mut init_body = Vec::new();
            let mut nested_classes = Vec::new();
            
            for (key, val) in map {
                let (field_type, nested) = match val {
                    Value::Null => ("None".to_string(), None),
                    Value::Bool(_) => ("bool".to_string(), None),
                    Value::Number(n) => {
                        if n.is_i64() || n.is_u64() {
                            ("int".to_string(), None)
                        } else {
                            ("float".to_string(), None)
                        }
                    },
                    Value::String(_) => ("str".to_string(), None),
                    Value::Array(items) => {
                        if items.is_empty() {
                            ("list".to_string(), None)
                        } else {
                            let nested_class_name = format!("{}{}", capitalize(key), "Type");
                            match &items[0] {
                                Value::Object(_) => {
                                    let nested_class = generate_python_class(&items[0], &nested_class_name)?;
                                    (format!("List[{}]", nested_class_name), Some(nested_class))
                                },
                                _ => {
                                    let simple_type = python_type_for_value(&items[0])?;
                                    (format!("List[{}]", simple_type), None)
                                }
                            }
                        }
                    },
                    Value::Object(_) => {
                        let nested_class_name = format!("{}{}", capitalize(key), "Type");
                        let nested_class = generate_python_class(val, &nested_class_name)?;
                        (nested_class_name.clone(), Some(nested_class))
                    },
                };
                
                if let Some(nested) = nested {
                    nested_classes.push(nested);
                }
                
                // Add type annotation
                fields.push(format!("    {}: {}", key, field_type));
                
                // Add parameter to __init__
                init_params.push(format!("{} = None", key));
                
                // Add assignment in __init__
                init_body.push(format!("        self.{} = {}", key, key));
            }
            
            // Render the class
            let mut result = String::new();
            
            // Import statements
            result.push_str("from typing import List, Optional, Dict, Any\n\n");
            
            // Add nested classes first
            for nested in nested_classes {
                result.push_str(&nested);
                result.push_str("\n\n");
            }
            
            // Add class definition
            result.push_str(&format!("class {}:\n", class_name));
            
            // Add type annotations
            for field in fields {
                result.push_str(&field);
                result.push_str("\n");
            }
            result.push_str("\n");
            
            // Add __init__ method
            result.push_str(&format!("    def __init__(self, {}):\n", init_params.join(", ")));
            for init_line in init_body {
                result.push_str(&init_line);
                result.push_str("\n");
            }
            
            // Add fromdict method
            result.push_str("\n    @classmethod\n");
            result.push_str(&format!("    def from_dict(cls, data: Dict[str, Any]) -> '{}':\n", class_name));
            result.push_str("        if data is None:\n");
            result.push_str("            return None\n");
            result.push_str("        return cls(\n");
            
            // Generate constructor arguments
            let mut from_dict_args = Vec::new();
            for (key, val) in map {
                match val {
                    Value::Object(_) => {
                        let class_name = format!("{}{}", capitalize(key), "Type");
                        from_dict_args.push(format!("            {} = {}.from_dict(data.get('{}'))", key, class_name, key));
                    },
                    Value::Array(items) if !items.is_empty() => {
                        match &items[0] {
                            Value::Object(_) => {
                                let class_name = format!("{}{}", capitalize(key), "Type");
                                // Fixed string concatenation issue
                                from_dict_args.push(format!("            {} = [{}.from_dict(item) for item in data.get('{}', [])]", key, class_name, key));
                            },
                            _ => {
                                from_dict_args.push(format!("            {} = data.get('{}')", key, key));
                            }
                        }
                    },
                    _ => {
                        from_dict_args.push(format!("            {} = data.get('{}')", key, key));
                    }
                }
            }
            
            result.push_str(&from_dict_args.join(",\n"));
            result.push_str("\n        )\n");
            
            Ok(result)
        },
        Value::Array(items) => {
            if items.is_empty() {
                let mut result = String::new();
                result.push_str("from typing import List, Any\n\n");
                result.push_str(&format!("class {}:\n", class_name));
                result.push_str("    items: List[Any]\n\n");
                result.push_str("    def __init__(self, items = None):\n");
                result.push_str("        self.items = items if items is not None else []\n\n");
                result.push_str("    @classmethod\n");
                result.push_str(&format!("    def from_dict(cls, data: List[Any]) -> '{}':\n", class_name));
                result.push_str("        if data is None:\n");
                result.push_str("            return None\n");
                result.push_str("        return cls(items=data)\n");
                Ok(result)
            } else {
                let item_class_name = format!("{}Item", class_name);
                let item_class = generate_python_class(&items[0], &item_class_name)?;
                
                let mut result = String::new();
                result.push_str("from typing import List, Any, Optional\n\n");
                result.push_str(&item_class);
                result.push_str("\n\n");
                result.push_str(&format!("class {}:\n", class_name));
                result.push_str(&format!("    items: List[{}]\n\n", item_class_name));
                result.push_str("    def __init__(self, items = None):\n");
                result.push_str("        self.items = items if items is not None else []\n\n");
                result.push_str("    @classmethod\n");
                result.push_str(&format!("    def from_dict(cls, data: List[Any]) -> '{}':\n", class_name));
                result.push_str("        if data is None:\n");
                result.push_str("            return None\n");
                result.push_str(&format!("        return cls(items=[{}.from_dict(item) for item in data])\n", item_class_name));
                Ok(result)
            }
        },
        _ => Err("Cannot generate Python class from non-object/non-array YAML".to_string()),
    }
}

fn python_type_for_value(value: &Value) -> Result<String, String> {
    match value {
        Value::Null => Ok("None".to_string()),
        Value::Bool(_) => Ok("bool".to_string()),
        Value::Number(n) => {
            if n.is_i64() || n.is_u64() {
                Ok("int".to_string())
            } else {
                Ok("float".to_string())
            }
        },
        Value::String(_) => Ok("str".to_string()),
        Value::Array(_) => Ok("list".to_string()),
        Value::Object(_) => Err("Need to generate a nested class for object types".to_string()),
    }
}

// Golang conversion
#[wasm_bindgen]
pub fn yaml_to_golang(yaml_str: &str) -> Result<String, JsValue> {
    let parsed = parse_yaml(yaml_str)
        .map_err(|e| JsValue::from_str(&e))?;
    
    let go_code = generate_golang_struct_yaml(&parsed, "RootType")
        .map_err(|e| JsValue::from_str(&e))?;
    
    Ok(go_code)
}

fn generate_golang_struct_yaml(value: &Value, struct_name: &str) -> Result<String, String> {
    match value {
        Value::Object(map) => {
            let mut fields = Vec::new();
            let mut nested_structs = Vec::new();
            
            for (key, val) in map {
                let field_name = capitalize(key);
                let (field_type, json_tag, nested) = match val {
                    Value::Null => ("interface{}".to_string(), format!("`json:\"{}\" yaml:\"{}\"`", key, key), None),
                    Value::Bool(_) => ("bool".to_string(), format!("`json:\"{}\" yaml:\"{}\"`", key, key), None),
                    Value::Number(n) => {
                        if n.is_i64() || n.is_u64() {
                            ("int64".to_string(), format!("`json:\"{}\" yaml:\"{}\"`", key, key), None)
                        } else {
                            ("float64".to_string(), format!("`json:\"{}\" yaml:\"{}\"`", key, key), None)
                        }
                    },
                    Value::String(_) => ("string".to_string(), format!("`json:\"{}\" yaml:\"{}\"`", key, key), None),
                    Value::Array(items) => {
                        if items.is_empty() {
                            ("[]interface{}".to_string(), format!("`json:\"{}\" yaml:\"{}\"`", key, key), None)
                        } else {
                            let nested_struct_name = format!("{}{}", capitalize(key), "Item");
                            match &items[0] {
                                Value::Object(_) => {
                                    let nested_struct = generate_golang_struct(&items[0], &nested_struct_name)?;
                                    (format!("[]{}", nested_struct_name), format!("`json:\"{}\" yaml:\"{}\"`", key, key), Some(nested_struct))
                                },
                                _ => {
                                    let simple_type = golang_type_for_value(&items[0])?;
                                    (format!("[]{}", simple_type), format!("`json:\"{}\" yaml:\"{}\"`", key, key), None)
                                }
                            }
                        }
                    },
                    Value::Object(_) => {
                        let nested_struct_name = format!("{}{}", capitalize(key), "Type");
                        let nested_struct = generate_golang_struct(val, &nested_struct_name)?;
                        (nested_struct_name.clone(), format!("`json:\"{}\" yaml:\"{}\"`", key, key), Some(nested_struct))
                    },
                };
                
                if let Some(nested) = nested {
                    nested_structs.push(nested);
                }
                
                fields.push(format!("\t{} {} {}", field_name, field_type, json_tag));
            }
            
            // Render the struct
            let mut result = String::new();
            
            // Add package declaration
            result.push_str("package main\n\n");
            
            // Add imports
            result.push_str("import (\n\t\"encoding/json\"\n\t\"gopkg.in/yaml.v2\"\n)\n\n");
            
            // Add nested structs first
            for nested in nested_structs {
                result.push_str(&nested);
                result.push_str("\n\n");
            }
            
            // Add the main struct
            result.push_str(&format!("type {} struct {{\n", struct_name));
            for field in fields {
                result.push_str(&field);
                result.push_str("\n");
            }
            result.push_str("}\n");
            
            Ok(result)
        },
        Value::Array(items) => {
            if items.is_empty() {
                let mut result = String::new();
                result.push_str("package main\n\n");
                result.push_str("import (\n\t\"encoding/json\"\n\t\"gopkg.in/yaml.v2\"\n)\n\n");
                // Fixed formatting issue
                result.push_str(&format!("type {} struct {{\n\tItems []interface{{}} `json:\"items\" yaml:\"items\"`\n}}\n", struct_name));
                Ok(result)
            } else {
                let item_struct_name = format!("{}Item", struct_name);
                let item_struct = generate_golang_struct(&items[0], &item_struct_name)?;
                
                let mut result = String::new();
                result.push_str("package main\n\n");
                result.push_str("import (\n\t\"encoding/json\"\n\t\"gopkg.in/yaml.v2\"\n)\n\n");
                result.push_str(&item_struct);
                result.push_str("\n\n");
                result.push_str(&format!("type {} struct {{\n\tItems []{} `json:\"items\" yaml:\"items\"`\n}}\n", struct_name, item_struct_name));
                Ok(result)
            }
        },
        _ => Err("Cannot generate Golang struct from non-object/non-array YAML".to_string()),
    }
}

fn golang_type_for_value(value: &Value) -> Result<String, String> {
    match value {
        Value::Null => Ok("interface{}".to_string()),
        Value::Bool(_) => Ok("bool".to_string()),
        Value::Number(n) => {
            if n.is_i64() || n.is_u64() {
                Ok("int64".to_string())
            } else {
                Ok("float64".to_string())
            }
        },
        Value::String(_) => Ok("string".to_string()),
        Value::Array(_) => Ok("[]interface{}".to_string()),
        Value::Object(_) => Err("Need to generate a nested struct for object types".to_string()),
    }
}

// Helper function to capitalize first letter of a string
fn capitalize(s: &str) -> String {
    let mut c = s.chars();
    match c.next() {
        None => String::new(),
        Some(first) => first.to_uppercase().collect::<String>() + c.as_str(),
    }
}


// XML to JAVA 
// Define a struct to represent an XML node


#[derive(Debug, Clone)] // Added Clone trait
struct XmlElement {
    name: String,
    attributes: HashMap<String, String>,
    children: Vec<XmlElement>,
}


#[wasm_bindgen]
pub fn xml_to_typescript(xml: &str) -> Result<String, JsValue> {
    let root = parse_xml(xml)?;
    Ok(generate_typescript(&root))
}

#[wasm_bindgen]
pub fn xml_to_java(xml: &str) -> Result<String, JsValue> {
    let root = parse_xml(xml)?;
    Ok(generate_java(&root))
}

#[wasm_bindgen]
pub fn xml_to_golang(xml: &str) -> Result<String, JsValue> {
    let root = parse_xml(xml)?;
    Ok(generate_golang(&root))
}

#[wasm_bindgen]
pub fn xml_to_python(xml: &str) -> Result<String, JsValue> {
    let root = parse_xml(xml)?;
    Ok(generate_python(&root))
}

fn parse_xml(xml: &str) -> Result<XmlElement, JsValue> {
    let mut chars = xml.trim().chars().peekable();
    let mut stack: Vec<XmlElement> = Vec::new();

    while let Some(c) = chars.next() {
        if c == '<' {
            let closing = chars.next_if_eq(&'/').is_some();
            let mut tag = String::new();
            
            // Parse tag name
            while let Some(&c) = chars.peek() {
                if c == '>' || c == ' ' { break; }
                tag.push(chars.next().unwrap());
            }

            if closing {
                // Handle closing tag
                let element = stack.pop().ok_or_else(|| {
                    JsValue::from_str("Closing tag without matching open tag")
                })?;

                if element.name != tag {
                    return Err(JsValue::from_str(
                        &format!("Mismatched tag: </{}> when expecting </{}>", tag, element.name)
                    ));
                }

                // Update root if we've closed the last element
                if stack.is_empty() {
                    return Ok(element);
                }
            } else {
                // Handle opening tag
                let mut attributes = HashMap::new();
                // Skip attributes parsing for brevity
                while chars.next_if_eq(&'>').is_none() {
                    chars.next();
                }

                let element = XmlElement {
                    name: tag.clone(),
                    attributes,
                    children: Vec::new(),
                };

                // Clone element for parent relationship
                if let Some(parent) = stack.last_mut() {
                    parent.children.push(element.clone());
                }

                stack.push(element);
            }
        }
    }

    Err(JsValue::from_str("Invalid XML structure"))
}
// TypeScript generation
fn generate_typescript(element: &XmlElement) -> String {
    let mut output = format!("interface {} {{\n", element.name);
    for (attr, _) in &element.attributes {
        output.push_str(&format!("  {}: string;\n", attr));
    }
    for child in &element.children {
        output.push_str(&format!("  {}: {};\n", child.name, child.name));
    }
    output.push_str("}\n");
    
    for child in &element.children {
        output.push_str(&generate_typescript(child));
    }
    output
}

// Java generation
fn generate_java(element: &XmlElement) -> String {
    let mut output = format!("public interface {} {{\n", element.name);
    for (attr, _) in &element.attributes {
        output.push_str(&format!("    String get{}();\n", capitalize(attr)));
    }
    for child in &element.children {
        output.push_str(&format!("    {} get{}();\n", child.name, capitalize(&child.name)));
    }
    output.push_str("}\n");
    
    for child in &element.children {
        output.push_str(&generate_java(child));
    }
    output
}

// Go generation
fn generate_golang(element: &XmlElement) -> String {
    let mut output = format!("type {} struct {{\n", element.name);
    for (attr, _) in &element.attributes {
        output.push_str(&format!("    {} string `xml:\"{},attr\"`\n", attr, attr));
    }
    for child in &element.children {
        output.push_str(&format!("    {} {} `xml:\"{}\"`\n", capitalize(&child.name), child.name, child.name));
    }
    output.push_str("}\n");
    
    for child in &element.children {
        output.push_str(&generate_golang(child));
    }
    output
}

// Python generation
fn generate_python(element: &XmlElement) -> String {
    let mut output = format!("@dataclass\nclass {}:\n", element.name);
    for (attr, _) in &element.attributes {
        output.push_str(&format!("    {}: str\n", attr));
    }
    for child in &element.children {
        output.push_str(&format!("    {}: {}\n", child.name, child.name));
    }
    output.push('\n');
    
    for child in &element.children {
        output.push_str(&generate_python(child));
    }
    output
}

// fn capitalize(s: &str) -> String {
//     let mut c = s.chars();
//     match c.next() {
//         None => String::new(),
//         Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
//     }
// }


// CSV parsing and type generation functions
#[wasm_bindgen]
pub fn csv_to_typescript(csv: &str) -> Result<String, JsValue> {
    let (headers, rows) = parse_csv(csv)
        .map_err(|e| JsValue::from_str(&format!("CSV parsing error: {}", e)))?;
    
    let type_info = infer_types(&headers, &rows)
        .map_err(|e| JsValue::from_str(&format!("Type inference error: {}", e)))?;
    
    generate_typescript_type_csv(&headers, &type_info)
        .map_err(|e| JsValue::from_str(&format!("TypeScript generation error: {}", e)))
}

#[wasm_bindgen]
pub fn csv_to_java(csv: &str) -> Result<String, JsValue> {
    let (headers, rows) = parse_csv(csv)
        .map_err(|e| JsValue::from_str(&format!("CSV parsing error: {}", e)))?;
    
    let type_info = infer_types(&headers, &rows)
        .map_err(|e| JsValue::from_str(&format!("Type inference error: {}", e)))?;
    
    generate_java_interface_csv(&headers, &type_info)
        .map_err(|e| JsValue::from_str(&format!("Java generation error: {}", e)))
}

#[wasm_bindgen]
pub fn csv_to_golang(csv: &str) -> Result<String, JsValue> {
    let (headers, rows) = parse_csv(csv)
        .map_err(|e| JsValue::from_str(&format!("CSV parsing error: {}", e)))?;
    
    let type_info = infer_types(&headers, &rows)
        .map_err(|e| JsValue::from_str(&format!("Type inference error: {}", e)))?;
    
    generate_golang_struct_csv(&headers, &type_info)
        .map_err(|e| JsValue::from_str(&format!("Golang generation error: {}", e)))
}

#[wasm_bindgen]
pub fn csv_to_python(csv: &str) -> Result<String, JsValue> {
    let (headers, rows) = parse_csv(csv)
        .map_err(|e| JsValue::from_str(&format!("CSV parsing error: {}", e)))?;
    
    let type_info = infer_types(&headers, &rows)
        .map_err(|e| JsValue::from_str(&format!("Type inference error: {}", e)))?;
    
    generate_python_class_csv(&headers, &type_info)
        .map_err(|e| JsValue::from_str(&format!("Python generation error: {}", e)))
}

// Helper functions

// Represents the detected type for each column
#[derive(Debug, Clone)]
enum FieldType {
    String,
    Number,
    Boolean,
    Date,
    Null,
    Mixed(Vec<FieldType>),
}

// Parse CSV into headers and rows
fn parse_csv(csv: &str) -> Result<(Vec<String>, Vec<Vec<String>>), String> {
    let mut lines = csv.lines().filter(|line| !line.trim().is_empty());
    
    // Parse headers
    let header_line = lines.next().ok_or("CSV is empty")?;
    let headers = parse_csv_row(header_line)?;
    
    if headers.is_empty() {
        return Err("No headers found in CSV".to_string());
    }
    
    // Parse data rows
    let mut rows = Vec::new();
    for (i, line) in lines.enumerate() {
        let row = parse_csv_row(line)?;
        
        // Check if row has same number of columns as headers
        if row.len() != headers.len() {
            return Err(format!("Row {} has {} columns, but headers have {} columns", 
                              i + 1, row.len(), headers.len()));
        }
        
        rows.push(row);
    }
    
    Ok((headers, rows))
}

// Parse a single CSV row, handling quoted fields
fn parse_csv_row(line: &str) -> Result<Vec<String>, String> {
    let mut result = Vec::new();
    let mut current_field = String::new();
    let mut chars = line.chars().peekable();
    let mut in_quotes = false;
    
    while let Some(c) = chars.next() {
        match c {
            '"' => {
                if in_quotes && chars.peek() == Some(&'"') {
                    // Escaped quote within quoted field
                    chars.next();
                    current_field.push('"');
                } else {
                    // Toggle quote state
                    in_quotes = !in_quotes;
                }
            },
            ',' if !in_quotes => {
                // End of field
                result.push(current_field);
                current_field = String::new();
            },
            _ => {
                current_field.push(c);
            }
        }
    }
    
    // Add the last field
    result.push(current_field);
    
    if in_quotes {
        return Err("Unclosed quotes in CSV".to_string());
    }
    
    Ok(result)
}

// Infer types from CSV data
fn infer_types(headers: &[String], rows: &[Vec<String>]) -> Result<HashMap<String, FieldType>, String> {
    let mut type_info = HashMap::new();
    
    // No data rows, assume all fields are strings
    if rows.is_empty() {
        for header in headers {
            type_info.insert(header.clone(), FieldType::String);
        }
        return Ok(type_info);
    }
    
    // Initialize type info with null for all fields
    for header in headers {
        type_info.insert(header.clone(), FieldType::Null);
    }
    
    // Infer types from each row
    for row in rows {
        for (i, value) in row.iter().enumerate() {
            if i >= headers.len() {
                continue;
            }
            
            let header = &headers[i];
            let inferred_type = infer_value_type(value);
            
            // Update the type info for this field
            let current_type = type_info.get(header).unwrap();
            let new_type = merge_types(current_type, &inferred_type);
            type_info.insert(header.clone(), new_type);
        }
    }
    
    Ok(type_info)
}

// Infer type from a single value
fn infer_value_type(value: &str) -> FieldType {
    let trimmed = value.trim();
    
    if trimmed.is_empty() {
        return FieldType::Null;
    }
    
    // Check for boolean
    if trimmed.eq_ignore_ascii_case("true") || trimmed.eq_ignore_ascii_case("false") {
        return FieldType::Boolean;
    }
    
    // Check for number
    if trimmed.parse::<f64>().is_ok() {
        return FieldType::Number;
    }
    
    // Check for date (simple ISO format check)
    if (trimmed.len() == 10 && trimmed.chars().filter(|&c| c == '-').count() == 2) ||
       (trimmed.len() >= 19 && trimmed.contains('T') && trimmed.contains(':')) {
        return FieldType::Date;
    }
    
    // Default to string
    FieldType::String
}

// Merge two field types
fn merge_types(current: &FieldType, new: &FieldType) -> FieldType {
    match (current, new) {
        (FieldType::Null, _) => new.clone(),
        (_, FieldType::Null) => current.clone(),
        (a, b) if std::mem::discriminant(a) == std::mem::discriminant(b) => a.clone(),
        (FieldType::Mixed(types), new_type) => {
            let mut updated_types = types.clone();
            if !types.iter().any(|t| std::mem::discriminant(t) == std::mem::discriminant(new_type)) {
                updated_types.push(new_type.clone());
            }
            FieldType::Mixed(updated_types)
        },
        (current_type, new_type) => {
            let mut types = vec![current_type.clone()];
            if !types.iter().any(|t| std::mem::discriminant(t) == std::mem::discriminant(new_type)) {
                types.push(new_type.clone());
            }
            FieldType::Mixed(types)
        }
    }
}

// Generate TypeScript type from type info
fn generate_typescript_type_csv(headers: &[String], type_info: &HashMap<String, FieldType>) -> Result<String, String> {
    let mut output = String::from("export interface CsvData {\n");
    
    for header in headers {
        let field_type = type_info.get(header).unwrap_or(&FieldType::String);
        let ts_type = field_type_to_typescript_csv(field_type);
        
        // Sanitize field name
        let field_name = sanitize_field_name(header);
        
        // Add optional marker if field might be null
        let optional = matches!(field_type, FieldType::Null | FieldType::Mixed(_));
        let optional_marker = if optional { "?" } else { "" };
        
        output.push_str(&format!("  {}{}: {};\n", field_name, optional_marker, ts_type));
    }
    
    output.push_str("}\n");
    Ok(output)
}

// Convert FieldType to TypeScript type
fn field_type_to_typescript_csv(field_type: &FieldType) -> String {
    match field_type {
        FieldType::String => "string".to_string(),
        FieldType::Number => "number".to_string(),
        FieldType::Boolean => "boolean".to_string(),
        FieldType::Date => "Date".to_string(),
        FieldType::Null => "null".to_string(),
        FieldType::Mixed(types) => {
            let mut unique_types = Vec::new();
            for t in types {
                let type_str = field_type_to_typescript_csv(t);
                if !unique_types.contains(&type_str) {
                    unique_types.push(type_str);
                }
            }
            unique_types.join(" | ")
        }
    }
}

// Generate Java interface from type info
fn generate_java_interface_csv(headers: &[String], type_info: &HashMap<String, FieldType>) -> Result<String, String> {
    let mut output = String::from("public interface CsvData {\n");
    
    // Generate getters for each field
    for header in headers {
        let field_type = type_info.get(header).unwrap_or(&FieldType::String);
        let java_type = field_type_to_java(field_type);
        
        // Sanitize field name
        let field_name = sanitize_field_name(header);
        let getter_name = format!("get{}", capitalize_first(field_name.as_str()));
        
        output.push_str(&format!("    {} {}();\n", java_type, getter_name));
    }
    
    output.push_str("}\n");
    Ok(output)
}

// Convert FieldType to Java type
fn field_type_to_java(field_type: &FieldType) -> String {
    match field_type {
        FieldType::String => "String".to_string(),
        FieldType::Number => "Double".to_string(),
        FieldType::Boolean => "Boolean".to_string(),
        FieldType::Date => "java.util.Date".to_string(),
        FieldType::Null => "Object".to_string(),
        FieldType::Mixed(_) => "Object".to_string(),
    }
}

// Generate Golang struct from type info
fn generate_golang_struct_csv(headers: &[String], type_info: &HashMap<String, FieldType>) -> Result<String, String> {
    let mut output = String::from("type CsvData struct {\n");
    
    for header in headers {
        let field_type = type_info.get(header).unwrap_or(&FieldType::String);
        let go_type = field_type_to_golang(field_type);
        
        // Sanitize and capitalize field name (Go public fields start with capitals)
        let field_name = capitalize_first(&sanitize_field_name(header));
        
        // Add JSON tag
        let json_tag = header.replace("\"", "\\\"");
        output.push_str(&format!("\t{} {} `json:\"{}\"` \n", field_name, go_type, json_tag));
    }
    
    output.push_str("}\n");
    Ok(output)
}

// Convert FieldType to Golang type
fn field_type_to_golang(field_type: &FieldType) -> String {
    match field_type {
        FieldType::String => "string".to_string(),
        FieldType::Number => "float64".to_string(),
        FieldType::Boolean => "bool".to_string(),
        FieldType::Date => "time.Time".to_string(),
        FieldType::Null => "interface{}".to_string(),
        FieldType::Mixed(_) => "interface{}".to_string(),
    }
}

// Generate Python class from type info
fn generate_python_class_csv(headers: &[String], type_info: &HashMap<String, FieldType>) -> Result<String, String> {
    let mut output = String::from("from dataclasses import dataclass\nfrom typing import Optional, Union, Any\nfrom datetime import datetime\n\n");
    output.push_str("@dataclass\nclass CsvData:\n");
    
    if headers.is_empty() {
        output.push_str("    pass\n");
        return Ok(output);
    }
    
    for header in headers {
        let field_type = type_info.get(header).unwrap_or(&FieldType::String);
        let python_type = field_type_to_python(field_type);
        
        // Sanitize field name
        let field_name = sanitize_field_name(header);
        
        // Add type annotation
        let optional = matches!(field_type, FieldType::Null | FieldType::Mixed(_));
        let type_annotation = if optional {
            format!("Optional[{}]", python_type)
        } else {
            python_type
        };
        
        output.push_str(&format!("    {}: {} = None\n", field_name, type_annotation));
    }
    
    Ok(output)
}

// Convert FieldType to Python type
fn field_type_to_python(field_type: &FieldType) -> String {
    match field_type {
        FieldType::String => "str".to_string(),
        FieldType::Number => "float".to_string(),
        FieldType::Boolean => "bool".to_string(),
        FieldType::Date => "datetime".to_string(),
        FieldType::Null => "None".to_string(),
        FieldType::Mixed(types) => {
            let mut unique_types = Vec::new();
            for t in types {
                let type_str = field_type_to_python(t);
                if !unique_types.contains(&type_str) && type_str != "None" {
                    unique_types.push(type_str);
                }
            }
            if unique_types.is_empty() {
                "Any".to_string()
            } else if unique_types.len() == 1 {
                unique_types[0].clone()
            } else {
                format!("Union[{}]", unique_types.join(", "))
            }
        }
    }
}

// Helper function to sanitize field names
fn sanitize_field_name(name: &str) -> String {
    let mut result = String::new();
    let mut chars = name.chars().peekable();
    let mut is_first = true;
    
    while let Some(c) = chars.next() {
        if is_first {
            // First character must be a letter or underscore for most languages
            if c.is_alphabetic() || c == '_' {
                result.push(c);
            } else {
                result.push('_');
            }
            is_first = false;
        } else {
            // Subsequent characters can be alphanumeric or underscore
            if c.is_alphanumeric() || c == '_' {
                result.push(c);
            } else {
                result.push('_');
            }
        }
    }
    
    // Handle reserved keywords
    match result.as_str() {
        "class" | "interface" | "type" | "struct" | "enum" | "import" | "export" |
        "function" | "if" | "else" | "for" | "while" | "return" | "break" | "continue" |
        "switch" | "case" | "default" | "try" | "catch" | "finally" | "throw" | "new" |
        "this" | "super" | "extends" | "implements" | "static" | "public" | "private" |
        "protected" | "final" | "abstract" | "const" | "let" | "var" | "void" => {
            format!("{}_", result)
        },
        _ => result
    }
}

// Helper function to capitalize the first letter of a string
fn capitalize_first(s: &str) -> String {
    let mut chars = s.chars();
    match chars.next() {
        None => String::new(),
        Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
    }
}



#[wasm_bindgen]
pub fn json_to_json_schema(json_str: &str) -> Result<String, JsValue> {
    // First, parse the JSON string
    let parsed = parse_json_json(json_str)
        .map_err(|e| JsValue::from_str(&format!("JSON parsing error: {}", e)))?;
    
    // Convert parsed JSON to JSON Schema
    let schema = generate_json_schema(&parsed)
        .map_err(|e| JsValue::from_str(&format!("Schema generation error: {}", e)))?;
    
    // Serialize schema back to string
    serde_json::to_string_pretty(&schema)
        .map_err(|e| JsValue::from_str(&format!("Schema serialization error: {}", e)))
}

// Parse JSON string into Value
fn parse_json_json(json_str: &str) -> Result<Value, String> {
    serde_json::from_str::<Value>(json_str)
        .map_err(|e| format!("Invalid JSON: {}", e))
}

// Generate JSON Schema from JSON Value
fn generate_json_schema(value: &Value) -> Result<Value, String> {
    let schema = detect_type_and_generate_schema(value, true)?;
    Ok(schema)
}

// Recursively determine types and generate schema
fn detect_type_and_generate_schema(value: &Value, is_root: bool) -> Result<Value, String> {
    match value {
        Value::Null => {
            let mut schema = Map::new();
            schema.insert("type".into(), Value::String("null".into()));
            Ok(Value::Object(schema))
        },
        Value::Bool(_) => {
            let mut schema = Map::new();
            schema.insert("type".into(), Value::String("boolean".into()));
            Ok(Value::Object(schema))
        },
        Value::Number(n) => {
            let mut schema = Map::new();
            // Determine if integer or number
            if n.is_i64() || n.is_u64() {
                schema.insert("type".into(), Value::String("integer".into()));
            } else {
                schema.insert("type".into(), Value::String("number".into()));
            }
            Ok(Value::Object(schema))
        },
        Value::String(_) => {
            let mut schema = Map::new();
            schema.insert("type".into(), Value::String("string".into()));
            Ok(Value::Object(schema))
        },
        Value::Array(arr) => {
            let mut schema = Map::new();
            schema.insert("type".into(), Value::String("array".into()));
            
            if arr.is_empty() {
                schema.insert("items".into(), Value::Object(Map::new()));
            } else {
                // Check if all items have the same type
                let sample_schemas: Vec<Value> = arr.iter()
                    .map(|item| detect_type_and_generate_schema(item, false))
                    .collect::<Result<Vec<_>, _>>()?;
                
                if all_same_type(&sample_schemas) {
                    // Use the first item's schema as the general schema
                    schema.insert("items".into(), sample_schemas[0].clone());
                } else {
                    // Create oneOf with all possible types
                    let mut unique_schemas = Vec::new();
                    let mut schema_signatures = HashSet::new();
                    
                    for schema in sample_schemas {
                        let signature = get_schema_signature(&schema);
                        if !schema_signatures.contains(&signature) {
                            schema_signatures.insert(signature);
                            unique_schemas.push(schema);
                        }
                    }
                    
                    schema.insert("items".into(), Value::Object({
                        let mut oneOf_map = Map::new();
                        oneOf_map.insert("oneOf".into(), Value::Array(unique_schemas));
                        oneOf_map
                    }));
                }
            }
            
            Ok(Value::Object(schema))
        },
        Value::Object(obj) => {
            let mut schema = Map::new();
            schema.insert("type".into(), Value::String("object".into()));
            
            let mut properties = Map::new();
            let mut required = Vec::new();
            
            for (key, val) in obj {
                properties.insert(key.clone(), detect_type_and_generate_schema(val, false)?);
                required.push(Value::String(key.clone()));
            }
            
            schema.insert("properties".into(), Value::Object(properties));
            if !required.is_empty() {
                schema.insert("required".into(), Value::Array(required));
            }
            
            if is_root {
                // Add schema metadata at root level
                schema.insert("$schema".into(), Value::String("http://json-schema.org/draft-07/schema#".into()));
            }
            
            Ok(Value::Object(schema))
        }
    }
}

// Helper function to check if all schemas are of the same type
fn all_same_type(schemas: &[Value]) -> bool {
    if schemas.is_empty() {
        return true;
    }
    
    let first_type = get_type(&schemas[0]);
    schemas.iter().all(|schema| get_type(schema) == first_type)
}

// Extract the type from a schema
fn get_type(schema: &Value) -> String {
    if let Value::Object(obj) = schema {
        if let Some(Value::String(type_str)) = obj.get("type") {
            return type_str.clone();
        }
    }
    "unknown".to_string()
}

// Create a simple signature for schema comparison
fn get_schema_signature(schema: &Value) -> String {
    if let Value::Object(obj) = schema {
        if let Some(Value::String(type_str)) = obj.get("type") {
            return if type_str == "object" {
                if let Some(Value::Object(props)) = obj.get("properties") {
                    let keys: Vec<String> = props.keys().cloned().collect();
                    return format!("object:{}", keys.join(","));
                }
                "object:{}".to_string()
            } else if type_str == "array" {
                if let Some(items) = obj.get("items") {
                    return format!("array:{}", get_schema_signature(items));
                }
                "array:any".to_string()
            } else {
                type_str.clone()
            };
        }
    }
    "unknown".to_string()
}