use std::mem::transmute;

pub fn invoke() {
    let pointer = foo as *const ();
    let function = unsafe { transmute::<*const (), fn() -> i32>(pointer) };
    assert_eq!(function(), 0);
    println!("Success!");

    let raw_bytes = [0x78, 0x56, 0x34, 0x12];
    let numb = unsafe { transmute::<[u8; 4], i32>(raw_bytes) };
    assert_eq!(numb, 0x12345678);
    let numb = u32::from_ne_bytes(raw_bytes); //从内存转换到native字节序列类型
    assert_eq!(numb, 0x12345678);
    let numb = u32::from_le_bytes(raw_bytes); //从内存转换到little-endian字节序列类型
    assert_eq!(numb, 0x12345678);
    let numb = u32::from_be_bytes(raw_bytes); //从内存转换到big-endian字节序列类型
    assert_eq!(numb, 0x78563412);

    let ptr = &0;
    let ptr_numb = unsafe { transmute::<&i32, usize>(ptr) };
    let ptr_cast_numb = ptr as *const i32 as usize;
    println!(
        "pointer: transmute -> {}\n\tcast -> {}",
        ptr_numb, ptr_cast_numb
    );

    let ptr_mut = &mut 0;
    let ptr_mut_numb = unsafe { transmute::<&mut i32, usize>(ptr_mut) };

    let ptr_mut_cast_numb = unsafe { &mut *(ptr_mut as *mut i32 as *mut u32) };
    println!(
        "pointer's mut: transmute -> {}\n\tcast -> {}",
        ptr_mut_numb, ptr_mut_cast_numb
    );
    const STR: &str = "Rust";
    // slice
    let slice = unsafe { transmute::<&str, &[u8]>(STR) };
    assert_eq!(slice, &[82, 117, 115, 116]);

    let x = STR.as_bytes();
    assert_eq!(x, &[82, 117, 115, 116]);
    assert_eq!(b"Rust", &[82, 117, 115, 116]);
}

fn foo() -> i32 {
    0
}
