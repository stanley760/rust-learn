// 几张卡牌 排成一行，每张卡牌都有一个对应的点数。点数由整数数组 cardPoints 给出。
//
// 每次行动，你可以从行的开头或者末尾拿一张卡牌，最终你必须正好拿 k 张卡牌。
//
// 你的点数就是你拿到手中的所有卡牌的点数之和。
//
// 给你一个整数数组 cardPoints 和整数 k，请你返回可以获得的最大点数。

pub struct Solution;

impl Solution {
    pub fn max_score(card_points: Vec<i32>, k: i32) -> i32 {
        let k = k as usize;
        let mut sum = card_points.iter().take(k).sum::<i32>();
        let mut ans = sum;
        let n = card_points.len();
        for i in 1..=k {
            sum += card_points[n - i] - card_points[k - i];
            ans = ans.max(sum);
        }
        ans
    }
}