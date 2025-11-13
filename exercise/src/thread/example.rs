use std::thread;
use std::time::Duration;

pub fn invoke() {
    let handle = thread::spawn(|| {
        for i in 1..5 {
            println!("hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_secs(1));
        }
    });
    // 调用 handle.join，可以让当前线程阻塞，直到它等待的子线程的结束
    handle.join().unwrap();
    for i in 1..5 {
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_secs(1));
    }
}

#[test]
fn invoke_test() {
    invoke();
}
