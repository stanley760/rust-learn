#[allow(unused)]
fn factory(x: i32) -> Box<dyn Fn(i32) -> i32> {
    let num = 5;
    
    if x > 1 {
        Box::new(move |x| num + x)
    } else {
        Box::new(move |x| num - x)
    }
}

#[test]
fn invoke_test() {
    let f = factory(1);
    println!("result:{}", f(1));
}