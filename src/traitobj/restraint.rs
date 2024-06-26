use std::ops::Add;

// 通过两种方法使用特征约束来实现 `fn sum`
fn sum<T: Add<Output = T>>(x: T, y: T) -> T {
    x + y
}

fn sum1<T>(x: T, y: T) -> T
where
    T: Add<Output = T>,
{
    x + y
}

pub fn invoke() {
    println!("sum:{}", sum(1, 2));
    println!("sum1:{}", sum1(1, 2));
}
