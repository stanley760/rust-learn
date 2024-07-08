pub fn invoke() {
    let mut s = String::from("hello");

    let r1 = &s;
    let r2 = &s;
    println!("{} and {}", r1, r2);
    // 新编译器中，r1,r2作用域在这里结束
    //引用的生命周期从借用处开始，一直持续到最后一次使用的地方。
    let r3 = &mut s;
    println!("{}", r3);
}
