#[allow(dead_code)]
struct Solution;
#[allow(dead_code)]
impl Solution {
    pub fn guess_number(n: i32) -> i32 {
        let mut left = 1;
        let mut right = n;
        while left < right {
            let mid = (left + right) >> 1;
            unsafe {
                match guess(mid) {
                    -1 => right = mid,
                    1 => left = mid + 1,
                    0 => return mid,
                    _ => unreachable!(),
                }
            }
        }
        left
    }
}

unsafe fn guess(num: i32) -> i32 {
    extern "C" {
        fn guess(num: i32) -> i32;
    }
    guess(num)
}