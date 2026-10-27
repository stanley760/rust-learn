
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

