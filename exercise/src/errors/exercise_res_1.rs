use std::num::ParseIntError;

fn multiply(n1_str: &str, n2_str: &str) -> Result<i32, ParseIntError> {
    let n1 = n1_str.parse::<i32>();
    let n2 = n2_str.parse::<i32>();
    Ok(n1.unwrap() * n2.unwrap())
}

pub fn invoke() {
    let result = multiply("10", "2");
    assert_eq!(result, Ok(20i32));

    let result = multiply("4", "2");
    assert_eq!(result, Ok(8i32));

    println!("Success!")
}
