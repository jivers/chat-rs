use serde::Serialize;
use std::collections::HashMap;

#[derive(Serialize, Debug, Clone)]
#[serde(rename_all = "lowercase")]
pub enum JsonType {
    String,
    Number,
    Integer,
    Boolean,
}

#[derive(Serialize, Debug, Clone)]
pub struct Property {
    #[serde(rename = "type")]
    pub r#type: JsonType,
    pub description: String,

    // enum type
    #[serde(skip_serializing_if = "Option::is_none", rename = "enum")]
    pub r#enum: Option<Vec<String>>,
}

#[derive(Serialize, Debug, Clone)]
pub struct Parameters {
    #[serde(rename = "type", default = "object")]
    pub r#type: String,
    pub properties: HashMap<String, Property>,
    pub required: Vec<String>,
}

#[derive(Serialize, Debug, Clone)]
pub struct Function {
    pub name: String,
    pub description: String,
    pub parameters: Parameters,
}

