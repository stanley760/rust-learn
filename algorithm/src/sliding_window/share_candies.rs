use std::collections::{HashMap, HashSet};

// 您将获得一个 从0开始的 整数数组 candies ，其中 `candies[i]`表示第 i 个糖果的味道。你妈妈想让你和你妹妹分享这些糖果，给她 k 个 连续 的糖果，但你想保留尽可能多的糖果口味。
// 在与妹妹分享后，返回 最多 可保留的 独特 口味的糖果。
// 输入: candies = [1,2,2,3,4,3], k = 3
// 输出: 3
// 解释:
// 将[1,3]（含[2,2,3]）范围内的糖果加入[2,2,3]口味。
// 你可以吃各种口味的糖果[1,4,3]。
// 有3种独特的口味，所以返回3。
//
// 输入: candies = [2,2,2,2,3,3], k = 2
// 输出: 2
// 解释:
// 在[3,4]范围内（含[2,3]）的糖果中加入[2,3]口味。
// 你可以吃各种口味的糖果[2,2,2,3]。
// 有两种独特的口味，所以返回2。
// 请注意，你也可以分享口味为[2,2]的糖果，吃口味为[2,2,3,3]的糖果。
//
// 输入: candies = [2,4,5], k = 0
// 输出: 3
// 解释:
// 你不必给任何糖果。
// 你可以吃各种口味的糖果[2,4,5]。
// 有3种独特的口味，所以返回3。
pub struct Solution;

impl Solution {
    /// 分析：candies = [1,2,2,3,4,3], k = 3
    /// 1.保证是连续的子数组长度为k,因此得出第一个窗口candies[0,k-1]
    pub fn share_candies(candies: Vec<i32>, k: i32) -> i32 {
        let k = k as usize;
        let mut cnt = HashMap::new();
        let n = candies.len();

        if k == 0 {
            return candies.iter().collect::<HashSet<_>>().len() as i32;
        }

        // 初始化哈希表, 统计第一个窗口 [0, k-1]出现的次数
        for i in k..n {
            *cnt.entry(candies[i]).or_insert(0) += 1;
        }

        let mut ans = cnt.len();
        // 滑动窗口从[1, k]到[n - k, n - 1]，i代表滑动窗口的右边界
        for i in k..n {
            // 移除即将进入窗口的元素
            let remove = candies[i];
            *cnt.entry(remove).or_insert(0) -= 1;
            if *cnt.entry(remove).or_insert(0) == 0 {
                cnt.remove(&remove);
            }
            // 添加即将离开窗口的元素
            let add = candies[i - k];
            *cnt.entry(add).or_insert(0) += 1;
            ans = ans.max(cnt.len());
            println!("i: {}, cnt: {:?}, ans: {:?}", i, cnt, ans);
        }
        ans as _
    }
}
