fn fn_once<F>(func: F)
// 因为`func`的类型是没有实现`Copy`特性的 `F`，所以发生了所有权的转移
where
    F: FnOnce(usize) -> bool + Copy,
{
    println!("len:3 -> {}", func(3)); // `func` moved due to this call  转移在这
    println!("len:4 -> {}", func(4)); // value used here after move  转移后再次用
}

pub fn invoke() {
    let vec1 = [1, 2, 3];
    // 1. FnOnce，该类型的闭包会拿走被捕获变量的所有权,
    //  F 实现了 Copy 特征，调用时使用的将是它的拷贝，所以并没有发生所有权的转移。
    fn_once(|len| len == vec1.len());
}

#[test]
fn test_fn_once() {
    let vec1 = [1, 2, 3];
    fn_once(|len| len == vec1.len());
}
