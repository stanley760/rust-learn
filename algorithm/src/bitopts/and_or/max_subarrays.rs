#[allow(dead_code)]
struct Solution;
#[allow(dead_code)]
impl Solution {
    pub fn max_subarrays(nums: Vec<i32>) -> i32 {
        let mut ans = 0;
        let mut a = -1;

        for x in nums {
            a &= x;
            if a == 0 {
                ans += 1;
                a = -1;
            }
        }
        // if answer is zero, it indicates that all nums is larger than zero, so answer is one. 
        ans.max(1)
    }
}

// cargo test --package algorithm --lib -- bitopts::and_or::max_subarrays::tests --nocapture
#[cfg(test)]
mod tests {
    use crate::and_or::max_subarrays::Solution;

    #[test]
    pub fn test_normal_case() {
        let nums = vec![1,0,2,0,1,2];
        assert_eq!(Solution::max_subarrays(nums), 3);

        let nums = vec![5,7,1,3];
        assert_eq!(Solution::max_subarrays(nums), 1);
    }
}