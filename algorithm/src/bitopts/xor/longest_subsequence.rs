#[allow(dead_code)]
pub struct Solution;
#[allow(dead_code)]
impl Solution {
    pub fn longest_subsequence(nums: Vec<i32>) -> i32 {
        let mut non_zero = false;
        let mut xor = 0;
        let n = nums.len();
        for e in nums {
            non_zero = non_zero || (e != 0);
            xor ^= e;
        }

        if !non_zero {
            return 0;
        }

        let mut ans = n;
        if xor == 0 {
            ans -= 1;
        }
        ans as _
    }

    pub fn longest_subsequence_v1(nums: Vec<i32>) -> i32 {
        if nums.iter().all(|&x| x == 0) {
            return 0;
        }

        let xor_sum = nums.iter().copied().reduce(|acc, x| acc ^ x).unwrap_or(0);

        (nums.len() as i32) - ((xor_sum == 0) as i32)
    }
}

#[cfg(test)]
mod tests {
    use crate::xor::longest_subsequence::Solution;

    #[test]
    pub fn test_normal_case() {
        let nums = vec![1, 2, 3];
        assert_eq!(Solution::longest_subsequence(nums), 2);

        let nums = vec![2, 3, 4];
        assert_eq!(Solution::longest_subsequence(nums), 3);

        let nums = vec![0, 0, 0];
        assert_eq!(Solution::longest_subsequence(nums), 0);
    }

    #[test]
    pub fn test_normal_case_v1() {
        let nums = vec![1, 2, 3];
        assert_eq!(Solution::longest_subsequence(nums), 2);

        let nums = vec![2, 3, 4];
        assert_eq!(Solution::longest_subsequence(nums), 3);

        let nums = vec![0, 0, 0];
        assert_eq!(Solution::longest_subsequence(nums), 0);
    }

    
}
