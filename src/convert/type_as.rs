#[allow(overflowing_literals)]
pub fn invoke() {
    let decimal = 97.123_f32;

    let integer: u8 = decimal as u8;

    let c1: char = decimal.to_string().chars().next().unwrap();
    let c2 = integer as char;

    assert_eq!(integer, 'a' as u8);
    println!("c1: {}, c2: {}", c1, c2);
    println!("Success!");

    assert_eq!(u8::MAX, 255);
    // 如上所示，u8 类型允许的最大值是 255.
    // 因此以下代码会报溢出的错误： literal out of range for `u8`.
    // **请仔细查看相应的编译错误，从中寻找到解决的办法**
    // **不要修改 main 中的任何代码**

    let _v = 1000 as u8;

    println!("Success!")
}
