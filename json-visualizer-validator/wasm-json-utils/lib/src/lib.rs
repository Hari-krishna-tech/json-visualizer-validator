use wasm_bindgen::prelude::*;
use serde_json::Value;
use serde_yaml;
use quick_xml::se::to_string as to_xml;
use csv:: Writer;
use std::collections::HashMap;

// export to js
#[wasm_bindgen]
pub fn json_to_yaml(json_str: &str) -> Result<String, JsValue> {
    let json_value: Value = serde_json::from_str(json_str).map_err(|e| JsValue::from_str(&e.to_string()))?;
    let yaml_string = serde_yaml::to_string(&json_value).map_err(|e| JsValue::from_str(&e.to_string()))?;
    Ok(yaml_string)
}


#[wasm_bindgen]
pub fn json_to_xml(json_str: &str) -> Result<String, JsValue> {
    let json_value: Value = serde_json::from_str(json_str).map_err(|e| JsValue::from_str(&e.to_string()))?;
    let xml_string = to_xml(&json_value).map_err(|e| JsValue::from_str(&e.to_string()))?;
    Ok(xml_string)
}
