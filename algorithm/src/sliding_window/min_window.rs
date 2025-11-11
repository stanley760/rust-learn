pub struct Solution;

impl Solution {
    pub fn min_window(s: String, t: String) -> String {
        let n = t.len();
        let m = s.len();
        if n > m {
            return "".to_string();
        }
        let mut left = 0;
        for (mut right, x) in s.bytes().enumerate() {
            if t.contains(x) {
                if right - left + 1 == n {
                    return s[left..=right].to_string();
                }
            } else {
                left += 1;
                right -= 1;
            }
        }
        "".to_string()
        
    }
}