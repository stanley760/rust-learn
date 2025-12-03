#[allow(dead_code)]
struct Solution;
#[allow(dead_code)]
impl Solution {
    pub fn guess_number(n: i32) -> i32 {
        let mut left = 1;
        let mut right = n;
        while left < right {
            let mid = (left + right) >> 1;
            match guess(mid) {
                -1 => right = mid,
                1 => left = mid + 1,
                0 => return mid,
                _ => unreachable!(),
            }
        }
        left
    }
}

// 模拟 LeetCode 的 guess API：假设正确答案是 6
const PICK: i32 = 6;

fn guess(num: i32) -> i32 {
    if num < PICK {
        1   // 猜小了，应该更大
    } else if num > PICK {
        -1  // 猜大了，应该更小
    } else {
        0   // 猜对了！
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_guess_number() {
        assert_eq!(Solution::guess_number(10), PICK);
    }
}