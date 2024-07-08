pub fn invoke() {
    // simulate the behavior of a loop with foreach.
    let values = vec![1, 2, 3, 4, 5];
    match IntoIterator::into_iter(values) {
        mut iter => {
            while let Some(value) = iter.next() {
                println!("{}", value);
            }
        }
    }
}

#[test]
fn invoke_test() {
    invoke();
}
