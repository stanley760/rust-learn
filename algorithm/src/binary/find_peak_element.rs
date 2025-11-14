#[allow(dead_code)]
struct Solution;
#[allow(dead_code)]
impl Solution {
    pub fn find_peak_element(nums: Vec<i32>) -> i32 {
        let (mut left, mut right) = (0, nums.len() - 1);
        while left < right {
            let mid = (left + right) >> 1;
            if nums[mid] > nums[mid + 1] {
                right = mid;
            } else {
                left = mid + 1;
            }
        }
        left as _

        // (0..nums.len() - 1)
        // .collect::<Vec<_>>()
        // .partition_point(|&i| nums[i] <= nums[i + 1]) as _
    }
}

#[cfg(test)]
mod tests {
    use crate::binary::find_peak_element::Solution;

    #[test]
    fn test_normal_case() {
        let nums = vec![1, 2, 3, 1];
        assert_eq!(Solution::find_peak_element(nums), 2);
    }
}
