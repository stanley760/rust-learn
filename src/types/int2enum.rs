use num_derive::FromPrimitive;
use num_traits::FromPrimitive;

#[derive(FromPrimitive)]
enum MyEnum {
    A = 1,
    B,
    C,
}

// the second way with TryFrom
impl TryFrom<i32> for MyEnum {
    type Error = ();

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        match value {
            x if x == MyEnum::A as i32 => Ok(MyEnum::A),
            x if x == MyEnum::B as i32 => Ok(MyEnum::B),
            x if x == MyEnum::C as i32 => Ok(MyEnum::C),
            _ => Err(()),

        }
    }
}

#[repr(i32)]
#[allow(dead_code)]
enum MyEnum1 {
    A = 1, 
    B, 
    C
}


pub fn invoke() {
    let x = 3;

    match FromPrimitive::from_i32(x) {
        Some(MyEnum::A) => println!("A"),
        Some(MyEnum::B) => println!("B"),
        Some(MyEnum::C) => println!("C"),
        None => println!("None"),
    }

    let b = MyEnum::B as i32;

    match b.try_into() {
        Ok(MyEnum::A) => println!("A"),
        Ok(MyEnum::B) => println!("B"),
        Ok(MyEnum::C) => println!("C"),
        Err(_) => println!("None"),
    }

    // transmute  unsafe !!!!

    let x = MyEnum1::A as i32;

    let y = unsafe { std::mem::transmute::<i32, MyEnum1>(x) };
    
    match y {
        MyEnum1::A => println!("A"),
        MyEnum1::B => println!("B"),
        MyEnum1::C => println!("C"),
    }

}

#[test]
fn invoke_test() {
    invoke();
}
