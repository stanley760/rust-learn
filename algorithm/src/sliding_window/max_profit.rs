// 给你两个整数数组 prices 和 strategy，其中：
//
// prices[i] 表示第 i 天某股票的价格。
// strategy[i] 表示第 i 天的交易策略，其中：
// -1 表示买入一单位股票。
// 0 表示持有股票。
// 1 表示卖出一单位股票。
//
// 同时给你一个 偶数 整数 k，你可以对 strategy 进行 最多一次 修改。一次修改包括：
//
// 选择 strategy 中恰好 k 个 连续 元素。
// 将前 k / 2 个元素设为 0（持有）。
// 将后 k / 2 个元素设为 1（卖出）。
//
// 利润 定义为所有天数中 strategy[i] * prices[i] 的 总和 。
//
// 返回你可以获得的 最大 可能利润。
//
// 注意： 没有预算或股票持有数量的限制，因此所有买入和卖出操作均可行，无需考虑过去的操作。
//
//
//
// 示例 1：
//
// 输入： prices = [4,2,8], strategy = [-1,0,1], k = 2
//
// 输出： 10
//
//
// 示例 2：
//
// 输入： prices = [5,4,3], strategy = [1,1,0], k = 2
//
// 输出： 9
//
// 因此，最大可能利润是 9，无需任何修改即可达成。

#[allow(dead_code)]
pub struct PrefixSumSolution;
#[allow(dead_code)]
pub struct SlidingWindowSolution;
impl PrefixSumSolution {
    /// 前缀和
    ///
    #[allow(unused)]
    pub fn max_profit(prices: Vec<i32>, strategy: Vec<i32>, k: i32) -> i32 {
        let k = k as usize;
        let n = prices.len();
        // prices*strategy 前缀和
        let mut sum = vec![0; n + 1];
        // prices 前缀和
        let mut sum_prices = vec![0; n + 1];

        for i in 0..n {
            sum[i + 1] = sum[i] + prices[i] * strategy[i];
            sum_prices[i + 1] = sum_prices[i] + prices[i];
        }
        // 不移动的情况下
        let mut ans = sum[n];
        // 移动的情况下, 修改的子数组[i - k, i - 1]修改后的利润由三部分组成：
        //
        // [0,i−k−1] 的 prices[i]⋅strategy[i] 之和，即 sum[i−k]。
        // [i - k, i- k/2 -1] 的 prices[i]⋅strategy[i] 之和(strategory=0)，即 0。
        // [i−k/2,i−1] 的 prices[i] 之和(strategory=1)，即 sumSell[i]−sumSell[i−k/2]。
        // [i,n−1] 的 prices[i]⋅strategy[i] 之和，即 sum[n]−sum[i]。
        for i in k..=n {
            ans = ans.max(sum[i - k] + sum[n] - sum[i] + sum_prices[i] - sum_prices[i - k / 2]);
        }
        ans
    }
}

impl SlidingWindowSolution {
    /// 滑动窗口
    ///
    #[allow(unused)]
    pub fn max_profit(prices: Vec<i32>, strategy: Vec<i32>, k: i32) -> i32 {
        let mut total = 0;
        let mut sum = 0;
        let mut max_sum = 0;
        let k = k as usize;
        for i in 0..prices.len() {
            total += prices[i] * strategy[i];
            // 1. into window
            sum += prices[i] * (1 - strategy[i]);
            // window size is less than the size of k.
            if i < (k - 1) {
                if i >= (k / 2 - 1) {
                    sum -= prices[i - (k / 2 - 1)]
                }
                continue;
            }
            // 2. update the result.
            max_sum = max_sum.max(sum);

            // 3. out of window.
            sum -= prices[i - (k / 2 - 1)] - prices[i - (k - 1)] * strategy[i - (k - 1)];
        }

        total + max_sum
    }
}

#[test]
pub fn test_max_profit() {
    let prices = vec![4, 2, 8];
    let strategy = vec![-1, 0, 1];
    let k = 2;
    let ans = PrefixSumSolution::max_profit(prices.clone(), strategy.clone(), k);
    assert_eq!(ans, 10);
    let ans = SlidingWindowSolution::max_profit(prices, strategy, k);
    assert_eq!(ans, 10);
}
