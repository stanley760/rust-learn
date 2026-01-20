#[allow(dead_code)]
struct Solution;
#[allow(dead_code)]
impl Solution {
    pub fn find_anagrams(s: String, p: String) -> Vec<i32> {
        let mut cnt_p = vec![0; 26];
        // count p's character
        for c in p.bytes() {
            cnt_p[(c - b'a') as usize] += 1;
        }
        let mut cnt_s = vec![0; 26];
        let s = s.as_bytes();
        let mut result = Vec::new();

        for (right, &c) in s.iter().enumerate() {
            cnt_s[(c - b'a') as usize] += 1;
            if right >= p.len() {
                cnt_s[(s[right - p.len()] - b'a') as usize] -= 1;
            }

            if cnt_s == cnt_p {
                result.push(right as i32 - p.len() as i32 + 1);
            }
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use crate::sliding_window::find_anagrams::Solution;

    #[test]
    pub fn test_normal_case() {
        let s = "cbaebabacd".to_string();
        let p = "abc".to_string();

        assert_eq!(Solution::find_anagrams(s, p), vec![0, 6]);

        let s = "abab".to_string();
        let p = "ab".to_string();
        assert_eq!(Solution::find_anagrams(s, p), vec![0, 1, 2]);
    }
}
