use std::rc::Rc;

pub fn invoke() {
    // <B>|对于父子引用关系，可以让父节点通过 Rc 来引用子节点，然后让子节点通过 Weak 来引用父节点。</B>

    let v = Rc::new(5);
    // Rc<T> 调用 downgrade 方法转换成 Weak<T>
    let weak_v = Rc::downgrade(&v);
    // Weak<T> 可使用 upgrade 方法转换成 Option<Rc<T>>
    let strong_v = weak_v.upgrade();

    assert_eq!(*strong_v.unwrap(), 5);
    drop(v);

    // drop 后，Option 为 None
    let strong_v = weak_v.upgrade();
    assert_eq!(strong_v, None)
}

#[cfg(test)]
fn invoke_test() {
    invoke();
}
