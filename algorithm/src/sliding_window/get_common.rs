#[allow(dead_code)]
struct Solution;
#[allow(unused)]
impl Solution {
    pub fn get_common(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
        let (mut i, mut j) = (0, 0);
        while i < nums1.len() && j < nums2.len() {
            if nums1[i] == nums2[j] {
                return nums1[i];
            } else if nums1[i] < nums2[j] {
                i += 1;
            } else {
                j += 1;
            }
        }
        -1
    }
}

#[test]
fn test_normal_case() {
    let nums1 = vec![1, 2, 3, 6];
    let nums2 = vec![2, 4, 6, 8];
    let result = Solution::get_common(nums1, nums2);
    assert_eq!(result, 2);
}
// cargo test --package algorithm --lib -- sliding_window::get_common::test_normal_case --exact --nocapture
