#[allow(dead_code)]
struct Solution;
#[allow(dead_code)]
impl Solution {
    pub fn maximum_or(nums: Vec<i32>, k: i32) -> i64 {
        let n = nums.len();

        // 后缀或值数组：suf[i] 表示 nums[i+1] | nums[i+2] | ... | nums[n-1]
        let mut suf = vec![0i64; n + 1];
        for i in (0..n).rev() {
            suf[i] = suf[i + 1] | nums[i] as i64;
        }

        let mut ans = 0i64;
        let mut pre = 0i64; // 前缀或值

        // 遍历每个位置，尝试将 k 次操作都用在该位置
        for i in 0..n {
            // 将 nums[i] 左移 k 位（相当于乘以 2^k）
            let shifted = (nums[i] as i64) << k;
            // 计算：前缀或 | 当前元素左移后 | 后缀或
            ans = ans.max(pre | shifted | suf[i + 1]);
            // 更新前缀或值
            pre |= nums[i] as i64;
        }

        ans
    }
}
#[cfg(test)]
mod tests {
    use crate::and_or::maximum_or::Solution;
    #[test]
    pub fn test_normal_case() {
        let nums = vec![12, 9];
        let k = 1;
        assert_eq!(Solution::maximum_or(nums, k), 30);

        let nums = vec![8, 1, 2];
        let k = 2;
        assert_eq!(Solution::maximum_or(nums, k), 35);
    }
}
