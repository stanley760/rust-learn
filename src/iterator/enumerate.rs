pub fn invoke() {
    let v = vec![1, 2, 3, 4, 5, 6];
    let va = v
        .iter()
        .enumerate()
        // 留下偶数索引对应的数值， -> [1, 3, 5]
        .filter(|&(i, _)| i % 2 == 0)
        .map(|(_, x)| x)
        // 1 + 3 + 5 = 9
        .fold(0, |acc, x| acc + x);
    println!("{}", va)
}

#[test]
fn invoke_test() {
    invoke();
}
