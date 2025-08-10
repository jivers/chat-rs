use serde::{Serialize, Deserialize};
use crate::function::{Function, FunctionCall};

#[derive(Serialize, Deserialize, Debug, Clone)]
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

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ToolCall {
    id: String,
    #[serde(rename = "type")]
    r#type: ToolType,
    function: Option<FunctionCall>,
}
