use std::cell::RefCell;
use std::rc::Rc;
use List::{Cons, Nil};

#[derive(Debug)]
#[allow(dead_code)]
enum List {
    Cons(i32, RefCell<Rc<List>>),
    Nil,
}

impl List {
    fn tail(&self) -> Option<&RefCell<Rc<List>>> {
        match self {
            Cons(_, item) => Some(item),
            Nil => None,
        }
    }
}

pub fn invoke() {
    let a = Rc::new(Cons(5, RefCell::new(Rc::new(Nil))));
    println!("a rc count = {}", Rc::strong_count(&a));
    println!("a next item = {:?}", a.tail());
    let b = Rc::new(Cons(10, RefCell::new(Rc::clone(&a))));

    println!("after b created, a rc count = {}", Rc::strong_count(&a));
    println!("b rc count = {}", Rc::strong_count(&b));
    println!("b next item = {:?}", b.tail());

    if let Some(link) = a.tail() {
        *link.borrow_mut() = Rc::clone(&b);
    }

    println!("after changing a, a rc count = {}", Rc::strong_count(&a));
    println!("after changing a, b rc count = {}", Rc::strong_count(&b));

    // println!("a next item = {:?}", a.tail()); // error: thread 'circle::weak::invoke_test' has overflowed its stack
}

#[test]
fn invoke_test() {
    invoke();
}
