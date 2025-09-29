// 好友给自己制定了一份健身计划。想请你帮他评估一下这份计划是否合理。
//
// 给定一个数组 calories，其中 calories[i] 代表好友第 i 天需要消耗的卡路里总量。
// 再给定 lowerlower 代表较低消耗的卡路里，upperupper 代表较高消耗的卡路里。再给定一个整数 k，代表连续 k 天。
//
// 如果你的好友在这一天以及之后连续 k 天内消耗的总卡路里 T 小于 lowerlower，则这一天的计划相对糟糕，并失去 1 分。
// 如果你的好友在这一天以及之后连续 k 天内消耗的总卡路里 T 高于 upperupper，则这一天的计划相对优秀，并得到 1 分。
// 如果你的好友在这一天以及之后连续 k 天内消耗的总卡路里 T 大于等于 lowerlower，并且小于等于 upperupper，则这份计划普普通通，分值不做变动。
//
// 要求：输出最后评估的得分情况。
// 示例 1:
// 输入：calories = [1,2,3,4,5], k = 1, lower = 3, upper = 3
// 输出：0
// 解释：calories[0], calories[1] < lower 而 calories[3], calories[4] > upper, 总分 = 0.
// 示例 2:
// 输入：calories = [3,2], k = 2, lower = 0, upper = 1
// 输出：1
// 解释：calories[0] + calories[1] > upper, 总分 = 1.
pub struct Solution;

impl Solution {
    pub fn diet_plan_performance(calories: Vec<i32>, k: i32, lower: i32, upper: i32) -> i32 {
        let k = k as usize;
        let mut ans = 0;
        // 需要检查 calories.len() - k + 1 个窗口
        for i in 0..calories.len() - k + 1 {
            // 当前窗口 [i, i+k) 内所有元素的和
            let sum: i32 = calories[i..i + k].iter().sum();
            if sum < lower {
                ans -= 1;
            } else if sum > upper {
                ans += 1;
            }
        }
        ans
    }
}