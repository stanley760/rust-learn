pub fn invoke() {
    // simulate the behavior of a loop with foreach.
    let values = [1, 2, 3, 4, 5];
    for value in values.iter() {
        println!("{}", value);
    }
}

#[test]
fn invoke_test() {
    invoke();
}
