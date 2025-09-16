use std::collections::HashSet;

/// 给定一个未排序的整数数组 nums ，找出数字连续的最长序列（不要求序列元素在原数组中连续）的长度。
///
/// 请你设计并实现时间复杂度为 O(n) 的算法解决此问题。
///
///
///
/// 示例 1：
///
/// 输入：nums = [100,4,200,1,3,2]
/// 输出：4
/// 解释：最长数字连续序列是 [1, 2, 3, 4]。它的长度为 4。
///
/// 示例 2：
///
/// 输入：nums = [0,3,7,2,5,8,4,6,0,1]
/// 输出：9
///
/// 示例 3：
///
/// 输入：nums = [1,0,1,2]
/// 输出：3


struct Solution;

impl Solution {
    pub fn longest_consecutive(nums: Vec<i32>) -> i32 {

        let hset = nums.into_iter().collect::<HashSet<i32>>();
        let mut res = 0;
        hset.iter().for_each(|e| {
            if hset.contains(&(e - 1)) {
                return;
            }
            let mut y = e + 1;
            while hset.contains(&y) {
                y += 1;
            }
            res = res.max(y - e);
            if res * 2 >= hset.len() as i32 {
                return;
            }
        });
        res
    }
}

fn main() {
    let x = vec![100,4,200,1,3,2];
    assert_eq!(Solution::longest_consecutive(x), 4);

    let x1 = vec![0,3,7,2,5,8,4,6,0,1];
    assert_eq!(Solution::longest_consecutive(x1), 9);

    let x2 = vec![1,0,1,2];
    assert_eq!(Solution::longest_consecutive(x2), 3);

    let x3 =vec![0, 0];
    assert_eq!(Solution::longest_consecutive(x3), 1);

    let x4 = vec![1, 100];
    assert_eq!(Solution::longest_consecutive(x4), 1);
}