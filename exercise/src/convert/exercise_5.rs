pub fn invoke() {
    let arr: [u64; 13] = [0; 13];
    assert_eq!(std::mem::size_of_val(&arr), 8 * 13);
    let a: *const [u64] = &arr;
    let b: *const [u8] = core::ptr::slice_from_raw_parts(a as *const u8, 13);
    unsafe { assert_eq!(std::mem::size_of_val(&*b), 13) }
}