use std::num::ParseIntError;

// 使用两种方式填空: map, and then
fn add_two(n_str: &str) -> Result<i32, ParseIntError> {
   n_str.parse::<i32>().map(|n| n + 2)
}

fn add_two1(n_str: &str) -> Result<i32, ParseIntError> {
    n_str.parse::<i32>().and_then(|n| Ok(n + 2))
 }

pub fn invoke() {
    assert_eq!(add_two("4").unwrap(), 6);
    assert_eq!(add_two1("4").unwrap(), 6);

    println!("Success!")
}