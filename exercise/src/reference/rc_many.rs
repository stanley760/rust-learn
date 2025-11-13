use std::rc::Rc;

use List::{Cons, Nil};
//
// b -> |3| |
//       \
//   a -> |5| | -> |10| | -> |Nil|
//        /
// c -> |4| |
//

#[allow(dead_code)]
enum List {
    Cons(i32, Rc<List>),
    Nil,
}

pub fn invoke() {
    let v = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    let _v1 = Cons(3, Rc::clone(&v));
    {
        let _v2 = Cons(4, Rc::clone(&v));
        println!("after invoke v2, count = {}", Rc::strong_count(&v));
    }

    println!("after drop v2, count = {}", Rc::strong_count(&v));
}

#[test]
fn invoke_test() {
    invoke();
}
