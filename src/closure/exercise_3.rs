use std::fmt::Display;

pub fn invoke() {
    let movable = Box::new(3);

    let consume = move|| {
        take(&movable);
    };

    consume();
    consume();
}

fn take<T: Display + Clone>(v: &T) {
    println!("`take`: {}", v);
}

#[test]
fn invoke_test() {
    invoke();
}