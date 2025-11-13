struct Solution;

#[allow(unused)]
impl Solution {
    pub fn maximum_count(nums: Vec<i32>) -> i32 {
        let positive_count = nums.len() as i32 - Self::lower_bound(&nums, 1) as i32;
        let negative_count = Self::lower_bound(&nums, 0) as i32;
        positive_count.max(negative_count)
    }

    fn lower_bound(nums: &Vec<i32>, target: i32) -> usize {
        let (mut left, mut right) = (0, nums.len());
        while left < right {
            let mid = left + (right - left) / 2;
            if nums[mid] < target {
                left = mid + 1;
            } else {
                right = mid;
            }
        }
        left
    }
}
#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_maximum_count() {
        let nums = vec![-2, -1, -1, 1, 2, 3];
        assert_eq!(Solution::maximum_count(nums.clone()), 3);

        let nums = vec![-3, -2, -1, 0, 0, 1, 2];
        assert_eq!(Solution::maximum_count(nums.clone()), 3);

        let nums = vec![5, 20, 66, 1314];
        assert_eq!(Solution::maximum_count(nums.clone()), 4);
    }
}
