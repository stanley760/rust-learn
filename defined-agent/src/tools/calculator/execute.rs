use crate::tools::ToolError;

/// 计算器支持的运算符。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Operator {
    Add,
    Subtract,
    Multiply,
    Divide,
}

impl Operator {
    /// 解析 LLM 传来的运算符字符串，兼容符号写法。
    pub fn parse(raw: &str) -> Result<Self, ToolError> {
        match raw.trim() {
            "add" | "+" => Ok(Self::Add),
            "subtract" | "substract" | "-" => Ok(Self::Subtract),
            "multiply" | "*" | "x" => Ok(Self::Multiply),
            "divide" | "/" => Ok(Self::Divide),
            other => Err(ToolError::Permanent(format!(
                "unsupported operation: {other}"
            ))),
        }
    }
}

/// 执行一次二元运算。
///
/// 运算本身是纯计算，不存在临时性故障，因此错误一律为确定性错误。
pub fn calculator(operator: Operator, lhs: f64, rhs: f64) -> Result<f64, ToolError> {
    match operator {
        Operator::Add => Ok(lhs + rhs),
        Operator::Subtract => Ok(lhs - rhs),
        Operator::Multiply => Ok(lhs * rhs),
        Operator::Divide => {
            if rhs == 0.0 {
                return Err(ToolError::Permanent("division by zero".to_string()));
            }
            Ok(lhs / rhs)
        }
    }
}
