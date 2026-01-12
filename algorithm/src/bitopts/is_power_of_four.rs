#[allow(dead_code)]
struct Solution;
#[allow(dead_code)]
impl Solution {
    pub fn is_power_of_four(n: i32) -> bool {
        n > 0 && n & (n - 1) == 0 && n & 0x55555555 > 0
    }
}

#[cfg(test)]
mod tests {
    use crate::is_power_of_four::Solution;

    #[test]
    pub fn test_normal_case() {
        assert_eq!(Solution::is_power_of_four(16),true);
        assert_eq!(Solution::is_power_of_four(5), false);
        assert_eq!(Solution::is_power_of_four(1), true);
        assert_eq!(Solution::is_power_of_four(8), false);
    }
}