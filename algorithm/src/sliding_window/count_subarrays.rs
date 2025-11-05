#[allow(dead_code)]
pub struct Solution;
#[allow(unused)]
impl Solution {
    /// [LeetCode][lc] 2302.
    /// [left,right],[left+1,right],…,[right,right]
    pub fn count_subarrays(nums: Vec<i32>, k: i64) -> i64 {
        let mut sum = 0;
        let mut left = 0;
        let mut ans =0;
        for (right, &x) in nums.iter().enumerate() {
            sum += x as i64;
            while sum * (right - left + 1) as i64 >= k {
                sum -= nums[left] as i64;
                left += 1;
            }
            ans += right - left + 1;
        }
        ans as i64
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_subarrays() {
        assert_eq!(Solution::count_subarrays(vec![2, 1, 4, 3, 5], 10), 6);
    }
}
