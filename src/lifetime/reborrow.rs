#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn move_to(&mut self, x: i32, y: i32) {
        self.x = x;
        self.y = y;
    }
}

pub fn invoke() {
    let mut p = Point { x: 0, y: 0 };
    let r = &mut p;
    let rr: &Point = &*r; //rr 是对 r 的再借用

    // 再借用`rr`最后一次使用发生在这里，在它的生命周期中，我们并没有使用原来的借用`r`，因此不会报错
    println!("{:?}", rr);
    // 再借用结束后，才去使用原来的借用`r`
    r.move_to(10, 10);
    println!("{:?}", r);
}