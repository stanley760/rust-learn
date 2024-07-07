use std::ops::Deref;
use std::ops::DerefMut;
struct MyBox<T> {
    v: T
}

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        return MyBox {v:x}
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.v
    }
}
///
/// 1.要实现 DerefMut 必须要先实现 Deref 特征：
/// pub trait DerefMut: Deref
/// 
/// 2. &mut MyBox<String> 转换为 &mut String
impl<T> DerefMut for MyBox<T> {
    
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.v
    }
}

fn display(s: &mut String) {
    s.push_str("rust");
    println!("{}",s);
}

pub fn invoke() {
    let mut s = MyBox::new(String::from("hello, "));
    display(&mut s);
}

#[test]
fn invoke_test() {
    invoke();
}