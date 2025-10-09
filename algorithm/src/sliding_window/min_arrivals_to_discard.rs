// 给你两个整数 w 和 m，以及一个整数数组 arrivals，其中 arrivals[i] 表示第 i 天到达的物品类型（天数从 1 开始编号）。

// 物品的管理遵循以下规则：
//
// 每个到达的物品可以被 保留 或 丢弃 ，物品只能在到达当天被丢弃。
// 对于每一天 i，考虑天数范围为 [max(1, i - w + 1), i]（也就是直到第 i 天为止最近的 w 天）：
// 对于 任何 这样的时间窗口，在被保留的到达物品中，每种类型最多只能出现 m 次。
// 如果在第 i 天保留该到达物品会导致其类型在该窗口中出现次数 超过 m 次，那么该物品必须被丢弃。
//
// 返回为满足每个 w 天的窗口中每种类型最多出现 m 次，最少 需要丢弃的物品数量。
//
//
//
// 示例 1：
//
// 输入： arrivals = [1,2,1,3,1], w = 4, m = 2
//
// 输出： 0
//
// 解释：
//
// 第 1 天，物品 1 到达；窗口中该类型不超过 m 次，因此保留。
// 第 2 天，物品 2 到达；第 1 到第 2 天的窗口是可以接受的。
// 第 3 天，物品 1 到达，窗口 [1, 2, 1] 中物品 1 出现两次，符合限制。
// 第 4 天，物品 3 到达，窗口 [1, 2, 1, 3] 中物品 1 出现两次，仍符合。
// 第 5 天，物品 1 到达，窗口 [2, 1, 3, 1] 中物品 1 出现两次，依然有效。
//
// 没有任何物品被丢弃，因此返回 0。
//
// 示例 2：
//
// 输入： arrivals = [1,2,3,3,3,4], w = 3, m = 2
//
// 输出： 1
//
// 解释：
//
// 第 1 天，物品 1 到达。我们保留它。
// 第 2 天，物品 2 到达，窗口 [1, 2] 是可以的。
// 第 3 天，物品 3 到达，窗口 [1, 2, 3] 中物品 3 出现一次。
// 第 4 天，物品 3 到达，窗口 [2, 3, 3] 中物品 3 出现两次，允许。
// 第 5 天，物品 3 到达，窗口 [3, 3, 3] 中物品 3 出现三次，超过限制，因此该物品必须被丢弃。
// 第 6 天，物品 4 到达，窗口 [3, 4] 是可以的。
//
// 第 5 天的物品 3 被丢弃，这是最少必须丢弃的数量，因此返回 1。

use std::collections::HashMap;

pub struct Solution;

impl Solution {
    pub fn min_arrivals_to_discard(mut arrivals: Vec<i32>, w: i32, m: i32) -> i32 {
        let mut ans = 0;
        let mut count = HashMap::new();
        let window_size = w as usize;

        for i in 0..arrivals.len() {
            let item = arrivals[i];
            // 先维护滑动窗口大小
            // 当窗口大小超过 w 时，移除最左边的元素
            if i >= window_size {
                let out_item = arrivals[i - window_size];
                if let Some(out_count) = count.get_mut(&out_item) {
                    *out_count -= 1;
                    if *out_count == 0 {
                        count.remove(&out_item);
                    }
                }
            }

            // 检查当前物品是否需要丢弃
            // 如果当前窗口中该类型物品已经达到了 m 次，则需要丢弃
            let current_count = *count.get(&item).unwrap_or(&0);
            if current_count >= m {
                arrivals[i] = -1;
                ans += 1;
                // 物品被丢弃，不加入窗口计数
            } else {
                // 物品被保留，加入窗口计数
                *count.entry(item).or_insert(0) += 1;
            }
        }

        ans
    }

    #[allow(unused)]
    pub fn min_arrivals_to_discard1(mut arrivals: Vec<i32>, w: i32, m: i32) -> i32 {
        let (n, w) = (arrivals.len(), w as usize);
        let mut cnt = HashMap::new();
        let mut ans = 0;
        for r in 0..n {
            if cnt.get(&arrivals[r]).is_some_and(|&v| v == m) {
                arrivals[r] = 0;
                ans += 1;
            } else {
                *cnt.entry(arrivals[r]).or_insert(0) += 1;
            }
            if r + 1 >= w {
                *cnt.entry(arrivals[r + 1 - w]).or_insert(0) -= 1;
            }
        }
        ans
    }
}
