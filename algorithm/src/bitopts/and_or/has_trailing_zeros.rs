
pub struct Solution;

impl Solution {
    pub fn has_trailing_zeros(nums: Vec<i32>) -> bool {
        let mut n = nums.len() as i32;
        for x in nums {
            n -= x % 2;
        }
        n >=2
    }
}
#[cfg(test)]
mod tests {
    use crate::and_or::has_trailing_zeros::Solution;

    #[test]
    pub fn test_normal_case() {
        let nums = vec![1,2,3,4,5];
        assert_eq!(Solution::has_trailing_zeros(nums), true);
        let nums = vec![2,4,8,16];
        assert_eq!(Solution::has_trailing_zeros(nums), true);
        let nums = vec![1,3,5,7,9];
        assert_eq!(Solution::has_trailing_zeros(nums), false);
    }
}