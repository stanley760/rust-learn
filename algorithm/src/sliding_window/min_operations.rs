pub struct Solution;

impl Solution {
    pub fn min_operations(nums: Vec<i32>, x: i32) -> i32 {
        
        let mut left = 0;
        let n = nums.len();
        
        let target = nums.iter().sum::<i32>() - x;
        if target < 0 {
            return -1;
        }
        let mut ans = -1i32;
        let mut sum = 0;
        for right in 0..n {
            sum += nums[right];
            while sum > target {
                sum -= nums[left];
                left += 1;
            }
            if sum == target {
                ans = ans.max((right - left + 1) as i32);
            }
        }
        ans = if ans < 0 {
            -1
        } else {
            n as i32 - ans
        };
        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_min_operations() {
        assert_eq!(Solution::min_operations(vec![1, 1, 4, 2, 3], 5), 2);
        assert_eq!(Solution::min_operations(vec![5, 6, 7, 8, 9], 4), -1);
        assert_eq!(Solution::min_operations( vec![3, 2, 20, 1, 1, 3], 10), 5);
    }
}