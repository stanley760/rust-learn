fn fn_once<F: FnOnce(usize) -> bool + Copy>(func: F) {
    println!("{}", func(3));
    println!("{}", func(4));
}

fn fn_once1<F: Fn(usize) -> bool>(func: F) {
    println!("{}", func(3));
    println!("{}", func(4));
}

pub fn invoke() {
    let vec1 = vec![1, 2, 3];
    println!("*******use fn_once*******");
    fn_once(|z| z == vec1.len());
    println!("*******use fn*******");
    fn_once1(|z| z == vec1.len());
}

#[test]
fn invoke_test() {
    invoke();
}