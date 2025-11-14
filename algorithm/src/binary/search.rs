#[allow(dead_code)]
struct Solution;

#[allow(unused)]
impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        let (mut left, mut right) = (0, nums.len() as isize - 1);
        while left <= right {
            let mid = left + (right - left) / 2;
            let m = mid as usize;
            if nums[m] == target {
                return m as i32;
            } else if nums[m] < target {
                left = mid + 1;
            } else {
                right = mid - 1;
            }
        }
        -1
    }

    pub fn search2(nums: Vec<i32>, target: i32) -> i32 {
        let pos = nums.partition_point(|&m| m < target);
        if pos < nums.len() && nums[pos] == target {
            pos as _
        } else {
            -1
        }
    }
}

// cargo test --package algorithm --lib -- binary::search::tests --nocapture
#[cfg(test)]
mod tests {
    use super::Solution;
    #[test]
    fn test_search() {
        let nums = vec![-1, 0, 3, 5, 9, 12];
        let target = 9;
        assert_eq!(Solution::search(nums.clone(), target), 4);
        assert_eq!(Solution::search2(nums, target), 4);
    }

    #[test]
    fn test_search_not_found() {
        let nums = vec![-1, 0, 3, 5, 9, 12];
        let target = 2;
        assert_eq!(Solution::search(nums, target), -1);
    }
}
