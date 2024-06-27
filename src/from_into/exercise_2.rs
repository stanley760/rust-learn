#[derive(Debug)]
struct Number {
    value: i32,
}

impl From<i32> for Number {
    fn from(value: i32) -> Self {
        Number {
            value,
        }
    }
    // 实现 `from` 方法

}

// 填空
pub fn invoke() {
    let num = Number::from(30);
    assert_eq!(num.value, 30);

    let num: Number = Number{value: 30};
    assert_eq!(num.value, 30);

    println!("Success!")
}