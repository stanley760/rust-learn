use schemars::JsonSchema;
use serde::Deserialize;

pub fn calculator<T>(operator: &str, first_number: T, second_number: T) -> anyhow::Result<T, String>
where
    T: Copy
        + PartialEq
        + Default
        + std::ops::Add<Output = T>
        + std::ops::Sub<Output = T>
        + std::ops::Mul<Output = T>
        + std::ops::Div<Output = T>,
{
    let operator = operator.trim();
    if operator == "" {
        return Err("operator must not be empty".to_string());
    }
    match operator {
        "add" | "+" => Ok(first_number + second_number),
        "substract" | "-" => Ok(first_number - second_number),
        "multiply" | "*" | "x" => Ok(first_number * second_number),
        "divide" | "/" => Ok(first_number / second_number),
        _ => Err(format!("unsupported operation:{}", operator)),
    }
}

#[derive(Deserialize, Debug, JsonSchema)]
pub struct CalculatorArgs {
    pub operator: String,
    pub(super) first_number: CalculatorNumber,
    pub(super) second_number: CalculatorNumber,
}

#[derive(serde::Deserialize, schemars::JsonSchema, Debug)]
#[serde(untagged)]
pub(super) enum CalculatorNumber {
    Integer(i64),
    Float(f64),
}

impl CalculatorNumber {
    pub(crate) fn as_f64(&self) -> f64 {
        match self {
            Self::Integer(value) => *value as f64,
            Self::Float(value) => *value,
        }
    }
}
