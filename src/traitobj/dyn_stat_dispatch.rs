trait Foo {
    fn method(&self) -> String;
}

impl Foo for u8 {
    fn method(&self) -> String { format!("u8: {}", *self) }
}

impl Foo for String {
    fn method(&self) -> String { format!("string: {}", *self) }
}

// 通过泛型实现以下函数
fn static_dispatch<T:Foo>(x: T) {
    println!("generic -> {}",x.method());
}

// 通过特征对象实现以下函数
fn dynamic_dispatch(x: &dyn Foo) {
    println!("trait object -> {}",x.method())
}

pub fn invoke() {
    let x: u8 = 5;
    let y: String = "hello".to_string();

    static_dispatch(x);
    dynamic_dispatch(&y);
}
