use serde::Serialize;
use crate::function::Function;

#[derive(Serialize, Debug, Clone)]
#[serde(rename_all = "lowercase")]
pub enum ToolType {
    Function
}

#[derive(Serialize, Debug, Clone)]
pub struct Tool {
    #[serde(rename = "type")]
    pub r#type: ToolType,

    // can we use other tool types in the future, like web search?
    #[serde(skip_serializing_if = "Option::is_none")]
    pub function: Option<Function>, 
}
