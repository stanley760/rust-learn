#[allow(dead_code)]    
struct Solution;

#[allow(unused)]
impl Solution {
    pub fn search_range(nums: Vec<i32>, target: i32) -> Vec<i32> {
    
        let left = Self::lower_bound(&nums, target);
        if left == nums.len() || nums[left] != target {
            return vec![-1, -1];
        }
        let right = Self::lower_bound(&nums, target + 1) - 1;
        vec![left as i32, right as i32]
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

    pub fn search_range2(nums: Vec<i32>, target: i32) -> Vec<i32> {
    
        let left = Self::lower_bound2(&nums, target);
        if left == nums.len() || nums[left] != target {
            return vec![-1, -1];
        }
        let right = Self::lower_bound2(&nums, target + 1) - 1;
        vec![left as i32, right as i32]
    }



    fn lower_bound2(nums: &Vec<i32>, target: i32) -> usize {
        // 开区间表示：left = -1, right = n
        let mut left: isize = -1;
        let mut right: isize = nums.len() as isize;

        while left + 1 < right {
            let mid = left + (right - left) / 2;
            let m = mid as usize;
            if nums[m] < target {
                left = mid;
            } else {
                right = mid;
            }
        }
        right as usize
    }

// cargo test --package algorithm --lib -- binary::search_range::tests --nocapture
}

#[cfg(test)]
mod tests {
    use super::Solution;
    #[test]
    fn test_normal_case() {
        let nums = vec![5, 7, 7, 8, 8, 10];
        let target = 8;
        let result = Solution::search_range(nums.clone(), target);
        let result2 = Solution::search_range2(nums, target);
        assert_eq!(result, vec![3, 4]);
        assert_eq!(result2, vec![3, 4]);
    }   

    #[test]
    fn test_not_found_case() {
        let nums = vec![5, 7, 7, 8, 8, 10];
        let target = 6;
        let result = Solution::search_range(nums.clone(), target);
        let result2 = Solution::search_range2(nums, target);
        assert_eq!(result, vec![-1, -1]);   
        assert_eq!(result2, vec![-1, -1]);   
    }
}