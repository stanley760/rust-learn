use std::rc::Rc;

pub fn invoke() {
    let s = String::from("hello, rust");
    // let _a = Box::new(s);
    // 所有权转移后报错
    // let b = Box::new(s);
    let a = Rc::new(s);
    let b = Rc::clone(&a);

    assert_eq!(Rc::strong_count(&a), 2);
    assert_eq!(Rc::strong_count(&a), Rc::strong_count(&b));
    let a = Rc::new(String::from("test ref counting"));
    println!("count after creating a = {}", Rc::strong_count(&a));
    let _b = Rc::clone(&a);
    println!("count after creating b = {}", Rc::strong_count(&a));
    {
        let _c = Rc::clone(&a);
        println!("count after creating c = {}", Rc::strong_count(&a));
    }
    println!("count after c goes out of scope = {}", Rc::strong_count(&a));
    println!("Success");
}

#[test]
fn invoke_test() {
    invoke();
}
