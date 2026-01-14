#[allow(dead_code)]
struct Solution;
#[allow(dead_code)]
impl Solution {
    pub fn longest_nice_subarray(nums: Vec<i32>) -> i32 {
        let mut ans = 0;
        let (mut or_, mut left) = (0, 0);
        
        for (right, &x) in nums.iter().enumerate() {
            while or_ & x > 0 {
                or_ ^= nums[left];
                left += 1;
            }
            or_ |= x;
            ans = ans.max(right - left + 1);
        }
        ans as _
    }
}
#[cfg(test)]
mod tests {
    use super::Solution;
    #[test]
    pub fn test_normal_case() {
        let nums = vec![1, 3, 8, 48, 10];
        assert_eq!(Solution::longest_nice_subarray(nums), 3);

        let nums = vec![3,1, 5, 11, 13];
        assert_eq!(Solution::longest_nice_subarray(nums), 1);
    }
}