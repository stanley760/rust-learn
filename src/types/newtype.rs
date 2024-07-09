use std::fmt;
// 使用元组结构体的方式将已有的类型包裹起
struct Wrapper(Vec<String>);

impl fmt::Display for Wrapper {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0.join(", "))
    }
}

pub fn invoke() {
    let w = Wrapper(vec![String::from("hello"), String::from("world")]);
    println!("w:{}", w);
}

#[test]
fn invoke_test() {
    invoke();
}
