

struct Solution;
impl Solution {
    pub fn bitwise_complement(n: i32) -> i32 {
        let ans = if n == 0 {
            1
        } else {
            !n << n.leading_zeros() >> n.leading_zeros()
        };
        ans
    }
}

#[cfg(test)]
mod tests {
    use crate::bitwise_complement::Solution;

    #[test]
    pub fn test_normal_case() {
        assert_eq!(Solution::bitwise_complement(5), 2);
        assert_eq!(Solution::bitwise_complement(7), 0);
        assert_eq!(Solution::bitwise_complement(10), 5);

    }
}