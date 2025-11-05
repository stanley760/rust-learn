
pub struct Solution;

impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        use std::collections::HashMap;
        let mut map = HashMap::new();
        let (mut left, mut right) = (0, 0);
        let mut ans = 0;
        let s = s.as_bytes();
        while right < s.len() {
            let c = s[right];
            *map.entry(c).or_insert(0) += 1;
            while map[&c] > 1 {
                let left_c = s[left];
                *map.entry(left_c).or_insert(0) -= 1;
                left += 1;
            }
            ans = ans.max(right - left + 1);
            right += 1;
        }
        ans as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_length_of_longest_substring() {
        let s = "abcabcbb".to_string();
        assert_eq!(Solution::length_of_longest_substring(s), 3);

        let s = "bbbbb".to_string();
        assert_eq!(Solution::length_of_longest_substring(s), 1);

        let s = "pwwkew".to_string();
        assert_eq!(Solution::length_of_longest_substring(s), 3);

        let s = "".to_string();
        assert_eq!(Solution::length_of_longest_substring(s), 0);
    }

    
}
