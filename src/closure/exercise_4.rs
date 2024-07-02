pub fn invoke() {
    let example_closure = |x| x;

    let s = example_closure(String::from("hello"));
    println!("s: {}", s);
    /* Make it work, only change the following line */
    let n = example_closure(5.to_string());
    println!("n: {}", n);
}

#[test]
fn invoke_test() {
    invoke();
}