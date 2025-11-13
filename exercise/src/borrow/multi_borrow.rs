// **** 只有全是多级可变引用的情况下，才能修改到目标资源的值。
// 对于多级引用（包含可变和不可变），打印语句中，可以自动为我们解引用正确的层数，直
// 到访问到目标资源的值，这很符合人的直觉和业务的需求。
#[test]
fn test_multiple_borrow() {
    let mut a = 10u32;
    let mut b = &mut a;
    *b = 20;
    println!("{:?}", b);
    let mut c = &mut b;
    **c = 30;
    println!("{:?}", c);
    let d = &mut c;
    ***d = 40;
    println!("{d}");
}
