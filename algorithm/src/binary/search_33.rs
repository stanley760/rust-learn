#[allow(dead_code)]
struct Solution;
#[allow(dead_code)]
impl Solution {
    fn find_min(nums: &[i32]) -> usize {
        let mut left = 0;
        let mut right = nums.len() - 1; // 左闭右开区间 [0, n-1)
        while left < right {
            // 区间不为空
            let mid = left + (right - left) / 2;
            if nums[mid] < nums[nums.len() - 1] {
                right = mid;
            } else {
                left = mid + 1;
            }
        }
        left
    }

    // 有序数组中找 target 的下标
    fn lower_bound(nums: &[i32],left: usize, target: i32) -> i32 {
       
        let index = nums.partition_point(|&x| x < target);

        // 检查找到的位置是否是目标值
        if index < nums.len() && nums[index] == target {
            (left + index) as i32
        } else {
            -1
        }
    }

    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        let i = Self::find_min(&nums);
        if target > nums[nums.len() - 1] {
            // target 在第一段
            Self::lower_bound(&nums[0..i], 0, target) // 左闭右开区间 [0, i)
        } else {
            // target 在第二段
            Self::lower_bound(&nums[i..nums.len()], i, target) // 左闭右开区间 [i, n)
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
}
