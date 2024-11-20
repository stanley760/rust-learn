// **** rule: 可变引用只能同时存在一个，或者任意多个不可变引用；
#[test]
fn test_borrow_two_times() {
    // let mut a = 1;
    // let r1 = &mut a;
    // let r2 = &mut a; // ❌ cannot borrow `a` as mutable more than once at a time
    // println!("{}, {}", r1, r2);
    let mut a = 1;
    {
        let r1 = &mut a;
        println!("{}", r1);
    }
    let r2 = &mut a;
    println!("{}", r2);
}