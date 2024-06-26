fn add_with_extra(x:i32, y:i32) -> i32 {
    let x = x + 1;
    let y = y + 2;
    x + y
}

fn ret_unit_type() {
    let x = 1;
    // if 语句块也是一个表达式，因此可以用于赋值，也可以直接返回
    // 类似三元运算符，在Rust里我们可以这样写
    let y = if x % 2 == 0 {
        "odd"
    } else {
        "even"
    };
    // 或者写成一行
    let z = if x % 2 == 1 { "odd" } else { "even" };
    println!("{} {}", y, z)
}

fn another_function(x: i32, y: f32) {
    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);
}

fn plus_or_minus(x:i32) -> i32 {
    if x > 5 {
        return x - 5
    }

    x + 5
}

pub fn invoke() {
    let x = 5;
    let y = 10.0;
    another_function(x, y);
    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);
    println!("The value of the method with plus_or_minus is: {}", plus_or_minus(x));
    println!("The value of the method with add_with_extra is: {}", add_with_extra(x, x));
    ret_unit_type();
}