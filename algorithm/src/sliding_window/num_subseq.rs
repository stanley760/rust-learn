struct Solution;


impl Solution {
    pub fn num_subseq(nums: Vec<i32>, target: i32) -> i32 {
        const MOD :i32 = 1_000_000_007;
        let n = nums.len();
        let (mut left, mut right) = (0, n - 1);
        
        let mut pow = vec![1; n];
        for i in 1..n {
            pow[i] = (pow[i - 1] * 2) % MOD;
        }
        let mut ans = 0;
        let mut nums = nums;
        nums.sort_unstable();
        while left <= right {
            if nums[left] + nums[right] <= target {
                ans = (ans + pow[right - left]) % MOD;
                left += 1;
            } else {
                if right == 0 {
                    break;
                }
                right -= 1;
            }
        }
        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_num_subseq() {
        let nums = vec![3,5,6,7];
        let target = 9;
        assert_eq!(Solution::num_subseq(nums, target), 4);

        let nums = vec![3,3,6,8];
        let target = 10;
        assert_eq!(Solution::num_subseq(nums, target), 6);

        let nums = vec![2,3,3,4,6,7];
        let target = 12;
        assert_eq!(Solution::num_subseq(nums, target), 61);

        let nums = vec![5,2,4,1,7,6,8];
        let target = 16;
        assert_eq!(Solution::num_subseq(nums, target), 127);
    }
}