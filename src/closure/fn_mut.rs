pub fn invoke() {
    let mut s = String::new();
    //  let mut update_string =  |str| s.push_str(str);
    let update_string = |str| s.push_str(str);

    exec(update_string);

    println!("{:?}", s);
}

/**
 *  exec(mut f: F)表明我们的exec接收的是一个可变类型的闭包
 *  F: FnMut(&'a str) 表示F是一个可变类型的闭包，它接收一个&str类型的参数
 */
fn exec<'a, F: FnMut(&'a str)>(mut f: F) {
    f("hello")
}
