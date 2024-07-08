use std::thread;

pub fn invoke() {
    let v = vec![1,2, 3];

    let handle = thread::spawn(move || {
        println!("Here's a vector: {:?}", v);
    });

    handle.join().unwrap();
    // println!("main thread,v:{:?}", v); // error:  value borrowed here after move
}

#[test]
fn invoke_test() {
    invoke();
}