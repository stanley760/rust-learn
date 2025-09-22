// 给你一个整数数组 arr 和两个整数 k 和 threshold 。
//
// 请你返回长度为 k 且平均值大于等于 threshold 的子数组数目。
//
//
//
// 示例 1：
//
// 输入：arr = [2,2,2,2,5,5,5,8], k = 3, threshold = 4
// 输出：3
// 解释：子数组 [2,5,5],[5,5,5] 和 [5,5,8] 的平均值分别为 4，5 和 6 。其他长度为 3 的子数组的平均值都小于 4 （threshold 的值)。
//
// 示例 2：
//
// 输入：arr = [11,13,17,23,29,31,7,5,2,3], k = 3, threshold = 5
// 输出：6
// 解释：前 6 个长度为 3 的子数组平均值都大于 5 。注意平均值不是整数。
//
//
//
// 提示：
//
//     1 <= arr.length <= 105
//     1 <= arr[i] <= 104
//     1 <= k <= arr.length
//     0 <= threshold <= 104
pub struct Solution;

impl Solution {
    pub fn num_of_subarrays(arr: Vec<i32>, k: i32, threshold: i32) -> i32 {
        let k = k as usize;
        let mut sum = 0;
        let mut ans  = 0;

        for (i, &e) in arr.iter().enumerate() {
            sum += e;
            if i + 1 < k {
                continue;
            }

            if sum >= k as i32 * threshold {
                ans += 1;
            }

           sum -= arr[i + 1 - k];
        }
        ans
    }
}

