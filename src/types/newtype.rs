use std::fmt;
// 使用元组结构体的方式将已有的类型包裹起
struct Wrapper(Vec<String>);

impl fmt::Display for Wrapper {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0.join(", "))
    }
}

#[test]
fn invoke_test() {
    let w = Wrapper(vec!["hello".to_string(), "world".to_string()]);
    println!("w:{}", w);
}