use async_openai::types::chat::{ChatCompletionTool, ChatCompletionTools, FunctionObjectArgs};
use serde_json::json;

pub fn calculate_tool_definition() -> ChatCompletionTools {
    ChatCompletionTools::Function(ChatCompletionTool {
        function: FunctionObjectArgs::default()
            .name("calculator")
            .description("Perform basic arithmetric operations.")
            .parameters(json!({
                "type": "object",
                "properties": {
                    "operator": {
                        "type": "string",
                        "description": "Arithmetric operation to Perform",
                        "enum": ["add", "substract", "multiply", "divide"]
                    },
                    "first_number": {
                        "type": "number",
                        "description": "First number for the calculator",
                    },
                    "second_number": {
                        "type": "number",
                        "description": "Second number for the calculator",
                    },
                }
            }))
            .build()
            .expect("fail to build calculator tool definition."),
    })
}
