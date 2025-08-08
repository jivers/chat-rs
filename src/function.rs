use serde::Serialize;

#[derive(Serialize)]
#[serde(rename_all = "lowercase")]
enum JsonType {
    String,
    Number,
    Integer,
    Boolean,
}


#[derive(Serialize)]
struct Parameter {
    type: JsonType,
    description: &str,

    // enum type
    #[serde(skip_serializing_if = "Option::is_none")]
    enum: Option<Vec<&str>>,
}

#[derive(Serialize)]
struct FunctionBuilder {
    name: String,
    description: &str,
    properties: Properties,
}

pub fn_select_chain = Function {
    name: "select_terminal_tool",
    description: "Check a prompt and see if a tool should be selected",
    properties: {
        tool : {
            type: String,
            description: "The command line tool to use",
            enum: Some(vec!["ffmpeg", "ls"]), 
        }
    }
}
