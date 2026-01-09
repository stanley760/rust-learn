#[allow(dead_code)]
struct Solution;
#[allow(dead_code)]
impl Solution {
    pub fn is_power_of_two(n: i32) -> bool {
        n > 0 && (n & (n - 1)) == 0
    }
}
#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    pub fn test_normal_case() {
        assert_eq!(Solution::is_power_of_two(1), true);
        assert_eq!(Solution::is_power_of_two(16), true);
        assert_eq!(Solution::is_power_of_two(3), false);
    }
}