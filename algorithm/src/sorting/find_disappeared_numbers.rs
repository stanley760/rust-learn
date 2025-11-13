pub struct Solution;

// LeetCode 448
// 给你一个含 n 个整数的数组 nums ，其中 nums[i] 在区间[1, n]内。
// 请你找出所有在[1, n]范围内但没有出现在nums中的数字，
// 并以数组的形式返回结果。
// 示例 1:
// 输入：nums = [4,3,2,7,8,2,3,1]
//
// 输出：[5,6]
//
// 示例 2:
// 输入：nums = [1,1]
//
// 输出：[2]
impl Solution {
    pub fn find_disappeared_numbers(nums: Vec<i32>) -> Vec<i32> {
        let mut nums = nums;
        for i in 0..nums.len() {
            while nums[i] != i as i32 + 1 && nums[i] != nums[nums[i] as usize - 1] {
                let cur = nums[i] as usize - 1;
                nums.swap(i, cur);
            }
        }

        nums.into_iter()
            .enumerate()
            .fold(Vec::new(), |mut acc, (i, n)| {
                if i as i32 + 1 != n {
                    acc.push(i as i32 + 1);
                }
                acc
            })
    }
}
