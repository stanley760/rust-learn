#[allow(dead_code)]
pub struct Solution;

#[allow(unused)]
impl Solution {
    pub fn apply_operations(nums: Vec<i32>) -> Vec<i32> {
        let mut nums = nums;
        let mut j = 0;

        for i in 0..nums.len() {
            // 
            if i + 1 < nums.len() && nums[i] == nums[i + 1] {
                nums[i] *= 2;
                nums[i + 1] = 0;
            }
            if nums[i] != 0 {
                nums.swap(i, j);
                j+=1;
            }
        }
        nums
    }
}

#[test]
pub fn test_normal_case() {
    let nums = vec![1,2,2,1,1,0];

    let result = Solution::apply_operations(nums);

    let expect = vec![1, 4, 2, 0, 0, 0];

    assert_eq!(result, expect);
}