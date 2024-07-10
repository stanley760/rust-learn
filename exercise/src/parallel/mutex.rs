use std::{sync::{Arc, Mutex}, thread};

pub fn invoke() {
    // Mutex<T> 有死锁风险
    let numb = Arc::new(Mutex::new(0));

    let mut handles = vec![];

    for _ in 0..10 {
        let numb = Arc::clone(&numb);
        let handle = thread::spawn(move || {
            let mut num = numb.lock().unwrap();
            *num += 1;
        });
        handles.push(handle);
    }
    handles.into_iter().for_each(|h| h.join().unwrap());
    println!("result: {}", *numb.lock().unwrap());
}

#[test]
fn invoke_test() {
    invoke();
}