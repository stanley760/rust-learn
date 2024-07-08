fn call_me<F: Fn()>(f: F) {
    f();
}

fn function() {
    println!("I'm a function!");
}

pub fn invoke() {
    let closure = || println!("I'm a closure!");

    call_me(closure);
    call_me(function);
}

#[test]
fn invoke_test() {
    invoke();
}
