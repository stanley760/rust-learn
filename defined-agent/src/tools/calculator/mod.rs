use schemars::schema_for;
use serde_json::Value;

use crate::tools::{
    Tool,
    calculator::execute::{CalculatorArgs, calculator},
};

pub mod calculate;
pub mod execute;

pub struct CalculatorTool;

#[async_trait::async_trait]
impl Tool for CalculatorTool {
    fn name(&self) -> &str {
        "calculator"
    }

    fn description(&self) -> &str {
        "Perform baic arithmetric for calculator"
    }

    fn parameters(&self) -> Value {
        serde_json::to_value(schema_for!(CalculatorArgs))
            .expect("Failed to serialize CalculatorArgs schema")
    }

    async fn execute(&self, arg_json: &str) -> anyhow::Result<String> {
        let args: CalculatorArgs = serde_json::from_str(arg_json)?;
        let first_number = args.first_number.as_f64();
        let second_number = args.second_number.as_f64();
        let result = calculator(&args.operator, first_number, second_number);
        match result {
            Ok(value) => Ok(value.to_string()),
            Err(err) => Ok(format!("Error: {err}")),
        }
    }
}
