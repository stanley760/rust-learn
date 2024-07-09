use crate::smart_ptr::deref::MyBox;

pub fn invoke() {
    // MyBox 被 Deref 成 String 类型
    let s = MyBox::new(String::from("hello, rust"));
    // display 函数参数为&str, 编译器发现 String 继续 Deref 成 &str
    display(&s);
    // 使用 Deref 连续转换在编译期完成
}

fn display(s: &str) {
    println!("{}", s)
}

#[test]
fn invoke_test() {
    invoke();
}
