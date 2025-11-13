// **** rule: 一个所有权型变量的不可变引用和可变引用的作用域不能共存
// 1. 所有权型变量的作用域从定义开始一直到 "}"结束，执行Drop
// 2. 引用型变量的作用域是从定义到最后一次使用结束
// 3. 引用型变量的作用域不能长于所有权型变量的作用域; 若出现则出现悬垂引用
// tip: println!("{a}");默认会对所有权变量做不可变借用（borrow）
#[test]
fn test_borrow_mut() {
    //let a = 10;
    let mut a = 10; // a 所有权变量，生命周期一直到 "}"结束，执行Drop
    let b = &mut a; // ❌ cannot borrow as mutable
    *b += 1;
    // println!("{a}");       // ❌ cannot borrow a as immutable
    // println!("{b}");       // 在这里a的不可变引用和可变引用作用域重叠
    println!("{b}");
    println!("{a}")
}
