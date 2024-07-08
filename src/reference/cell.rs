use std::cell::Cell;

pub fn invoke() {
    let x = Cell::new(1);
    let y = &x;
    let z = &x;

    x.set(2);
    y.set(3);
    z.set(4);

    println!("x:{},y:{}, z:{}", x.get(), y.get(), z.get());
}

#[test]
fn invoke_test() {
    invoke();
}
