#[allow(dead_code)]
struct Solution;
#[allow(dead_code)]
impl Solution {
    pub fn find_min(nums: Vec<i32>) -> i32 {
        let (mut left, mut right) = (0, nums.len() - 1);
        while left < right {
            let mid = (left + right) >> 1;
            if nums[mid] > nums[right] {
                left = mid + 1;
            } else if nums[mid] < nums[right] {
                right = mid;
            } else {
                right -= 1;
            }
        }
        nums[left]
    }
}


#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_normal_case() {
        let nums = vec![2, 2, 2, 0, 1];
        assert_eq!(Solution::find_min(nums), 0);

        let nums = vec![1, 3, 5];
        assert_eq!(Solution::find_min(nums), 1);

        let nums = vec![1, 1, 1, 1, 1];
        assert_eq!(Solution::find_min(nums), 1);
    }
}