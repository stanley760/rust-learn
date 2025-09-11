#[cfg(test)]
mod test {
    use std::sync::{Arc, Mutex};
    use std::thread;

    #[test]
    fn thread_arc() {
        let counter = Arc::new(Mutex::new(0));

        let mut threads = vec![];

        for _ in 0..10 {
            let counter = Arc::clone(&counter);
            threads.push(thread::spawn(move || {
                let mut num = counter.lock().unwrap();
                *num += 1;
            }));
        }

        for t in threads {
            t.join().unwrap();
        }

        assert_eq!(10, *counter.lock().unwrap());
    }


    use std::{slice::from_raw_parts, str::from_utf8_unchecked};

    // 获取字符串的内存地址和长度
    fn get_memory_location() -> (usize, usize) {
        let string = "Hello World!";
        let pointer = string.as_ptr() as usize;
        let length = string.len();
        (pointer, length)
    }

    // 在指定的内存地址读取字符串
    fn get_str_at_location(pointer: usize, length: usize) -> &'static str {
        unsafe { from_utf8_unchecked(from_raw_parts(pointer as *const u8, length)) }
    }

    #[test]
    fn test_raw_pointer() {
        let (pointer, length) = get_memory_location();
        let message = get_str_at_location(pointer, length);
        println!(
            "The {} bytes at 0x{:X} stored: {}",
            length, pointer, message
        );
        // 如果大家想知道为何处理裸指针需要 `unsafe`，可以试着反注释以下代码
        // let message = get_str_at_location(1000, 10);
        // println!("message: {}", message);
    }
}