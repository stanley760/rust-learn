use schemars::{JsonSchema, schema_for};
use serde::Deserialize;
use serde_json::Value;

use crate::tools::{
    Tool, ToolError,
    calculator::execute::{Operator, calculator},
};

#[derive(Deserialize, Debug, JsonSchema)]
struct CalculatorArgs {
    operator: String,
    first_number: f64,
    second_number: f64,
}

pub struct CalculatorTool;

#[async_trait::async_trait]
impl Tool for CalculatorTool {
    fn name(&self) -> &str {
        "calculator"
    }

    fn description(&self) -> &str {
        "Perform basic arithmetic: add, subtract, multiply, divide"
    }

    fn parameters(&self) -> Value {
        serde_json::to_value(schema_for!(CalculatorArgs))
            .expect("Failed to serialize CalculatorArgs schema")
    }

    async fn execute(&self, arg_json: &str) -> Result<String, ToolError> {
        let args: CalculatorArgs = serde_json::from_str(arg_json).map_err(|e| {
            ToolError::Permanent(format!("invalid calculator arguments: {e}"))
        })?;

        let operator = Operator::parse(&args.operator)?;
        let result = calculator(operator, args.first_number, args.second_number)?;
        Ok(result.to_string())
    }
}
