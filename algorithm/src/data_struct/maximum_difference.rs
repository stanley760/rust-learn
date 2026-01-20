use std::i32;

#[allow(dead_code)]
struct Solution;
#[allow(dead_code)]
impl Solution {
    pub fn maximum_difference(nums: Vec<i32>) -> i32 {
        let mut pre_min  = i32::MAX;
        let mut ans =  0;
        for x in nums {
            ans = ans.max(x - pre_min);
            pre_min = pre_min.min(x);
        }
        if ans > 0 { ans} else { -1 }
    }
}
#[cfg(test)]
mod tests {
    use super::Solution;


    #[test]
    pub fn test_normal_case() {
        let nums = vec![7, 1, 5, 4];
        assert_eq!(Solution::maximum_difference(nums), 4);
        let nums = vec![9, 4, 3, 2];
        assert_eq!(Solution::maximum_difference(nums), -1);
        let nums = vec![1, 5, 2, 10];
        assert_eq!(Solution::maximum_difference(nums), 9);
    }
}