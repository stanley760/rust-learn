#[allow(dead_code)]
struct Solution;
#[allow(unused)]
impl Solution {
    pub fn add_spaces(s: String, spaces: Vec<i32>) -> String {
        let mut ans = String::with_capacity(s.len() + spaces.len());
        let mut left = 0;
        for right in spaces {
            ans.push_str(&s[left..right as usize]);
            ans.push(' ');
            left = right as usize;
        }
        ans.push_str(&s[left..]);
        ans
    }
}

#[test]
fn test_normal_case() {
    let s = "LeetcodeHelpsMeLearn".to_string();
    let spaces = vec![8, 13, 15];
    let result = Solution::add_spaces(s, spaces);
    assert_eq!(result, "Leetcode Helps Me Learn");
}
// cargo test --package algorithm --lib -- sliding_window::add_spaces::test_normal_case --exact --nocapture
