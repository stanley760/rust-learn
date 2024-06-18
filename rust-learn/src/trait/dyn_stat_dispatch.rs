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
    println!("{}",x.method());
}

// 通过特征对象实现以下函数
fn dynamic_dispatch(x: &dyn Foo) {
    println!("{}",x.method())
}
