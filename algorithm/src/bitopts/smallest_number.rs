#[allow(dead_code)]
struct Solution;
#[allow(dead_code)]
impl Solution {
    pub fn smallest_number(n: i32) -> i32 {
        let bit_length = 32 - n.leading_zeros();
        return (1<< bit_length) - 1;
    }
}
#[cfg(test)]
mod tests {
    use super::Solution;


    #[test]
    pub fn test_normal_case() {
        let n = 5;
        assert_eq!(Solution::smallest_number(n), 7);

        let n = 10;
        assert_eq!(Solution::smallest_number(n), 15);

        let n = 3;
        assert_eq!(Solution::smallest_number(n), 3);
    }
}