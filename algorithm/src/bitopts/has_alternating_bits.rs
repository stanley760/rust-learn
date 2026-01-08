
struct Solution;
#[allow(dead_code)]
impl Solution {
    pub fn has_alternating_bits(n: i32) -> bool {
        let num = n ^ (n >> 1);
        num & (num + 1) == 0 
    }
}
#[cfg(test)]
mod tests {
    use crate::has_alternating_bits::Solution;


    #[test]
    pub fn test_normal_case() {
        assert_eq!(Solution::has_alternating_bits(5), true);
        assert_eq!(Solution::has_alternating_bits(7), false);
        assert_eq!(Solution::has_alternating_bits(11), false);
    }
}