// 2090. K Radius Subarray Averages
// https://leetcode.com/problems/k-radius-subarray-averages/
// You are given a 0-indexed array nums of n integers, and an integer k.
// The k-radius average for a subarray of nums centered at some index i with radius k is the average of all elements in nums between the indices i - k and i + k (inclusive). If there are less than k elements before or after the index i, then the k-radius average is -1.
// Build and return an array avgs of length n where avgs[i] is the k-radius average for the subarray centered at index i.
// The average of x elements is the sum of the x elements divided by x, using integer division. The integer division truncates toward zero, which means losing its fractional part.
// For example, the average of four elements 2, 3, 1, and 5 is (2 + 3 + 1 + 5) / 4 = 11 / 4 = 2.75, which truncates to 2.
/// 示例 1：
/// 输入：nums = [7,4,3,9,1,8,5,2,6], k = 3
/// 输出：[-1,-1,-1,5,4,4,-1,-1,-1]
/// 解释：
/// - avg[0]、avg[1] 和 avg[2] 是 -1 ，因为在这几个下标前的元素数量都不足 k 个。
/// - 中心为下标 3 且半径为 3 的子数组的元素总和是：7 + 4 + 3 + 9 + 1 + 8 + 5 = 37 。
///   使用截断式 整数除法，avg[3] = 37 / 7 = 5 。
/// - 中心为下标 4 的子数组，avg[4] = (4 + 3 + 9 + 1 + 8 + 5 + 2) / 7 = 4 。
/// - 中心为下标 5 的子数组，avg[5] = (3 + 9 + 1 + 8 + 5 + 2 + 6) / 7 = 4 。
/// - avg[6]、avg[7] 和 avg[8] 是 -1 ，因为在这几个下标后的元素数量都不足 k 个。

/// 示例 2：

/// 输入：nums = [100000], k = 0
/// 输出：[100000]
/// 解释：
/// - 中心为下标 0 且半径 0 的子数组的元素总和是：100000 。
///   avg[0] = 100000 / 1 = 100000 。

/// 示例 3：

/// 输入：nums = [8], k = 100000
/// 输出：[-1]
/// 解释：
/// - avg[0] 是 -1 ，因为在下标 0 前后的元素数量均不足 k 。

pub struct Solution;

impl Solution {
    pub fn get_averages(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let mut sum = 0;
        let mut ans = vec![-1; nums.len()];
        let k = k as usize;
        let n = nums.len();
        for i in 0..n {
            let cur = nums[i];
            sum += cur as i64;
            if i < 2*k {
                continue;
            }
            ans[i-k] = (sum / (2 * k + 1) as i64) as _;
            sum -= nums[i - 2*k] as i64;
        }

        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_get_averages() {
        let nums = vec![7, 4, 3, 9, 1, 8, 5, 2, 6];
        let k = 3;
        let res = vec![-1, -1, -1, 5, 4, 4, -1, -1, -1];
        assert_eq!(Solution::get_averages(nums, k), res);

        let nums = vec![100000];
        let k = 0;
        let res = vec![100000];
        assert_eq!(Solution::get_averages(nums, k), res);

        let nums = vec![8];
        let k = 100000;
        let res = vec![-1];
        assert_eq!(Solution::get_averages(nums, k), res);
    }
}