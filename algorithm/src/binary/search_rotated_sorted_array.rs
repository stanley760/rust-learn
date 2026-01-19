#[allow(dead_code)]
struct Solution;
#[allow(dead_code)]
impl Solution {
    fn find_min(nums: &[i32]) -> usize {

        let n = nums.len();
        let pos = (0..n - 1)
            .collect::<Vec<_>>()
            .partition_point(|&i| nums[i] >= nums[nums.len() - 1]);
        pos
    }

    // 有序数组中找 target 的下标
    fn lower_bound(nums: &[i32], mut left: usize, mut right: usize, target: i32) -> i32 {
        while left < right {
            let mid =  (left + right) >> 1;
            if nums[mid] < target {
                left = mid + 1;
            } else {
                right = mid;
            }
        }

        // 检查找到的位置是否是目标值
        if nums[left] == target {
            left as i32
        } else {
            -1
        }
    }

    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        let i = Self::find_min(&nums);
        if target > nums[nums.len() - 1] {
            // target 在第一段
            Self::lower_bound(&nums ,0, i, target) // 左闭右开区间 [0, i)
        } else {
            // target 在第二段
            Self::lower_bound(&nums, i, nums.len(), target) // 左闭右开区间 [i, n)
        }
    }

    pub fn search_v1(nums: Vec<i32>, target: i32) -> i32 {
        let is_blue = |i| -> bool{
            if nums[0] <= nums[i] {
                nums[0] <= target && target <= nums[i]
            } else {
                target <= nums[i] || nums[0] <= target
            }
        };
        let mut left = 0;
        let mut right = nums.len();
        while left < right {
            let mid = (left + right) >> 1;
            if is_blue(mid) {
                right = mid;
            } else {
                left = mid + 1;
            }
        }
        if left == nums.len() || nums[left] != target {
            -1
        } else {
            left as i32
        }
    }
}

//  cargo test --package algorithm --lib -- binary::search_33::tests --nocapture
#[cfg(test)]
mod tests {
    use super::Solution;
    #[test]
    fn test_search() {
        let nums = vec![4, 5, 6, 7, 0, 1, 2];
        let target = 0;
        assert_eq!(Solution::search(nums.clone(), target), 4);
    }

    #[test]
    fn test_search_not_found() {
        let nums = vec![4, 5, 6, 7, 0, 1, 2];
        let target = 3;
        assert_eq!(Solution::search(nums, target), -1);
    }

    #[test]
    fn test_search_single_element() {
        let nums = vec![1];
        let target = 0;
        assert_eq!(Solution::search(nums, target), -1);
    }

    #[test]
    fn test_case_v1() {
         let nums = vec![4, 5, 6, 7, 0, 1, 2];
        let target = 0;
        assert_eq!(Solution::search_v1(nums.clone(), target), 4);
        let nums = vec![4, 5, 6, 7, 0, 1, 2];
        let target = 3;
        assert_eq!(Solution::search(nums, target), -1);
         let nums = vec![4, 5, 6, 7, 0, 1, 2];
        let target = 3;
        assert_eq!(Solution::search(nums, target), -1);
    }
}
