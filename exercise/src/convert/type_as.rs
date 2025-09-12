#[allow(overflowing_literals)]
pub fn invoke() {
    let decimal = 97.123_f32;

    let integer: u8 = decimal as u8;

    let c1: char = decimal.to_string().chars().next().unwrap();
    let c2 = integer as char;

    assert_eq!(integer, 'a' as u8);
    println!("c1: {}, c2: {}", c1, c2);
    println!("Success!");

    assert_eq!(u8::MAX, 255);
    // 如上所示，u8 类型允许的最大值是 255.
    // 因此以下代码会报溢出的错误： literal out of range for `u8`.
    // **请仔细查看相应的编译错误，从中寻找到解决的办法**
    // **不要修改 main 中的任何代码**

    let _v = 1000 as u8;

    println!("Success!")
}
#[allow(dead_code)]
#[derive(Debug)]
struct Number {
    value: i32,
}

impl From<i32> for Number {
    fn from(v: i32) -> Number {
        Number { value: v }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_from_into() {
        let i1: i32 = false.into();
        let i2: i32 = i32::from(false);
        assert_eq!(i1, i2);
        assert_eq!(i1, 0);
        let _i3: u32 = 'a'.into();
        let _s: String = 'a'.into();
    }

    #[test]
    fn test_from_trait() {
        let num = Number::from(30);
        assert_eq!(num.value, 30);

        let num: Number = 30.into();
        assert_eq!(num.value, 30);
    }

    #[test]
    fn test_try_into_trait() {
        let n: i16 = 256;

        let n: u8 = n.try_into().unwrap_or_else(|_| {
            println!("Failed to convert i16 to u8");
            0
        });

        assert_eq!(n, 0);
    }

    #[test]
    fn test_from_trait_error() {
        use std::fs;
        use std::io::Error;
        use std::num::ParseIntError;
        #[derive(Debug)]
        enum CliErr {
            IoError(Error),
            ParseError(ParseIntError),
        }

        impl From<Error> for CliErr {
            fn from(value: Error) -> Self {
                CliErr::IoError(value)
            }
        }

        impl From<ParseIntError> for CliErr {
            fn from(value: ParseIntError) -> Self {
                CliErr::ParseError(value)
            }
        }

        fn open_and_parse_file(file_name: &str) -> Result<i32, CliErr> {
            let contents = fs::read_to_string(file_name)?;
            let num = contents.trim().parse::<i32>()?;
            Ok(num)
        }

        let res = open_and_parse_file("exercise_6.rs");
        println!("{:#?}", res)
    }
    use std::cell::RefCell;
    use std::rc::Rc;
    use std::sync::Once;
    use std::thread;
    use List::{Cons, Nil};
    #[derive(Debug)]
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
    #[test]
    fn test_out_memory() {
        let a = Rc::new(Cons(5, RefCell::new(Rc::new(Nil))));

        println!("a的初始化rc计数 = {}", Rc::strong_count(&a));
        println!("a指向的节点 = {:?}", a.tail());

        // 创建`b`到`a`的引用
        let b = Rc::new(Cons(10, RefCell::new(Rc::clone(&a))));

        println!("在b创建后，a的rc计数 = {}", Rc::strong_count(&a));
        println!("b的初始化rc计数 = {}", Rc::strong_count(&b));
        println!("b指向的节点 = {:?}", b.tail());

        // 利用RefCell的可变性，创建了`a`到`b`的引用
        if let Some(link) = a.tail() {
            *link.borrow_mut() = Rc::clone(&b);
        }

        println!("在更改a后，b的rc计数 = {}", Rc::strong_count(&b));
        println!("在更改a后，a的rc计数 = {}", Rc::strong_count(&a));

        // 下面一行println!将导致循环引用
        // 我们可怜的8MB大小的main线程栈空间将被它冲垮，最终造成栈溢出
        println!("a next item = {:?}", a.tail());
    }

    static mut VAL: usize = 0;
    static INIT: Once = Once::new();

    #[test]
    fn test_once() {
        let handle2 = thread::spawn(move || {
            println!("Thread 2: about to call call_once");
            INIT.call_once(|| unsafe {
                println!("Thread 2: executing closure");
                VAL = 2;
            });
            println!("Thread 2: call_once completed");
        });

        let handle1 = thread::spawn(move || {
            println!("Thread 1: about to call call_once");
            INIT.call_once(|| {
                println!("Thread 1: executing closure");
                unsafe {
                    VAL = 1;
                }
            });
            println!("Thread 1: call_once completed");
        });


        handle2.join().unwrap();
        handle1.join().unwrap();

        println!("VAL: {}", unsafe { VAL });
    }
}
