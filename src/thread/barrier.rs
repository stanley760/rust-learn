use std::sync::{Arc, Barrier};
use std::thread;

pub fn invoke() {
    let mut vec = Vec::with_capacity(6);
    let barrier = Arc::new(Barrier::new(6));

    for _ in 0..6 {
        let b = barrier.clone();
        vec.push(thread::spawn(move || {
            println!("waiting");
            b.wait();
            println!("done");
        }))
    }
    for thread in vec {
        thread.join().unwrap();
    }
}

#[test]
fn invoke_test() {
    invoke();
}