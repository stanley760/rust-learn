#[allow(dead_code)]
struct Solution;

#[allow(dead_code)]
impl Solution {
     pub fn single_non_duplicate(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let mut left = 0;
        let mut right = n / 2;
        while left < right {
            let mid = (left + right) >> 1;
            if nums[mid * 2] == nums[mid * 2 + 1] {
                left = mid + 1;
            } else {
                right = mid;
            }
        }
        nums[left * 2]
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_single_non_duplicate() {
        let nums = vec![1,1,2,3,3,4,4,8,8];
        assert_eq!(Solution::single_non_duplicate(nums), 2);
    }

    #[test]
    fn test_single_non_duplicate_2() {
        let nums = vec![3,3,7,7,10,11,11];
        assert_eq!(Solution::single_non_duplicate(nums), 10);
    }
}