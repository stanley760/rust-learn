#[allow(dead_code)]
struct Solution;
#[allow(dead_code)]
impl Solution {
    pub fn find_min(nums: Vec<i32>) -> i32 {
        let (mut left, mut right) = (0, nums.len() - 1);
        while left < right {
            let mid = (left + right) >> 1;
            if nums[mid] < nums[nums.len() - 1] {
                right = mid;
            } else {
                left = mid + 1;
            }
        }
        nums[right]
        // let pos =  (0..nums.len() - 1)
        //  .collect::<Vec<_>>()
        //  .partition_point(|&i| nums[i] >= nums[nums.len() - 1]);
        //  nums[pos]
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_normal_case() {
        let nums = vec![3, 4, 5, 1, 2];
        assert_eq!(Solution::find_min(nums), 1);

        let nums = vec![4, 5, 6, 7, 0, 1, 2];
        assert_eq!(Solution::find_min(nums), 0);

        let nums = vec![11, 13, 15, 17];
        assert_eq!(Solution::find_min(nums), 11);
    }
}
