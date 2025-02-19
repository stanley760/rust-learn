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

fn call_once(arg: String, c: impl FnOnce(String) -> (String, String)) -> (String, String) {
    c(arg)
}

fn not_closure(arg: String) -> (String, String) {
    (arg, "Rosie".into())
}


mod test {
    use super::*;
    #[test]
    pub fn test_closure() {
        let name = String::from("Tyr");

        // 这个闭包会 clone 内部的数据返回，所以它不是 FnOnce
        let c = move |greeting: String| (greeting, name.clone());

        // 所以 c1 可以被调用多次

        println!("c1 call once: {:?}", c("qiao".into()));
        println!("c1 call twice: {:?}", c("bonjour".into()));

        // 然而一旦它被当成 FnOnce 被调用，就无法被再次调用
        println!("result: {:?}", call_once("hi".into(), c));


        // Fn 也可以被当成 FnOnce 调用，只要接口一致就可以
        println!("result: {:?}", call_once("hola".into(), not_closure));
    }
}