pub fn invoke() {
    let n: i16 = 256;

    // Into 特征拥有一个方法`into`,
    // 因此 TryInto 有一个方法是 ?
    let n: u8 = n.try_into().unwrap_or_else(|e: std::num::TryFromIntError| {
        println!(
            "there is an error when converting: {:?}, but we catch it",
            e.to_string()
        );
        0
    });

    assert_eq!(n, 0);

    println!("Success!")
}
