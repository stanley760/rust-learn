// **** 一个所有权型变量的可变引用也具有所有权特征（排它性）
#[test]
fn same_borrow_mut() {
    let mut x = 5;
    let r1 = &mut x;
    // let r2 = r1;           // ❌ value borrowed here after move
    let r2 = *r1;
    println!("r1: {}, r2: {}", r1, r2);
}
