fn create_fn() -> impl Fn(i32) -> i32 {
    let num = 5;

    // How does the following closure capture the environment variable `num`
    // &T, &mut T, T ?
    move |x| x + num
}

pub fn invoke() {
    let fn_plain = create_fn();
    fn_plain(1);
}

#[test]
fn invoke_test() {
    invoke();
}
