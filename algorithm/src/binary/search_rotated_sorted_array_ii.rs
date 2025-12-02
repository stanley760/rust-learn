#[allow(dead_code)]
struct Solution;
#[allow(dead_code)]
impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> bool {
        let check = |mid: usize, right: usize| -> bool {
           
           if nums[mid] > nums[right] {
                // mid 在左半部分 单调递减
                target > nums[right] && target <= nums[mid]
           } else {
                // mid 在右半部分 单调递增
                target > nums[right] || target <= nums[mid]
           }
        };

        let (mut left, mut right) = (0, nums.len() - 1);
        while left < right {
            let mid = (left + right) >> 1;
            if check(mid, right) {
                right = mid;
            } else if nums[mid] == nums[right] {
                //  right 当作「数组最后一个数的下标」
                right -= 1;
            } else {
                left = mid + 1;
            }
        }
        nums[left] == target
    }
}
#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_search() {
        let nums = vec![2, 5, 6, 0, 0, 1, 2];
        let target = 0;
        assert_eq!(Solution::search(nums, target), true);

        let nums = vec![2, 5, 6, 0, 0, 1, 2];
        let target = 3;
        assert_eq!(Solution::search(nums, target), false);

        let nums = vec![1, 0, 1, 1, 1];
        let target = 0;
        assert_eq!(Solution::search(nums, target), true);

        let nums = vec![1, 1, 1, 1, 1];
        let target = 0;
        assert_eq!(Solution::search(nums, target), false);
    }
}