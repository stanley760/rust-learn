use std::fmt;

// 定义一个 newtype `Pretty`
struct Pretty(String);

impl fmt::Display for Pretty {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "\"{}\"", self.0.clone() + ", world")
    }
}

pub fn invoke() {
    let w = Pretty("hello".to_string());
    println!("w = {}", w);
}
