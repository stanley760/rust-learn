struct Solution;

impl Solution {
    pub fn max_vowels(s: String, k: i32) -> i32 {
        // fold<Acc, Fold>(self, init: Acc, fold: Fold)
        let s = s.as_bytes();
        let k = k as usize;
        let mut ans = 0;
        let mut count = 0;
        for (i, &c) in s.iter().enumerate() {
            // 1. right node come in window
            count = if c == b'a' || c == b'e' || c == b'i' || c == b'o' || c == b'u' {
                count + 1
            } else {
                count
            };
            // 窗口大小不足 k，尚未形成第一个窗口
            if i + 1 < k {
                continue;
            }

            ans = ans.max(count);

            if ans == k {
                // 答案已经等于理论最大值
                break;
            }

            // 左端点离开窗口，为下一个循环做准备
            let out = s[i + 1 - k];
            count = if out == b'a' || out == b'e' || out == b'i' || out == b'o' || out == b'u' {
                count - 1
            } else {
                count
            };
        }

        ans as _
    }
}

fn main() {
    let s = String::from("abciiidef");
    let k = 3;
    println!("{:?}", Solution::max_vowels(s, k));
}
