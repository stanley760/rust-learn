// 给你一个整数数组 arr ，以及 a、b 、c 三个整数。请你统计其中好三元组的数量。
//
// 如果三元组 (arr[i], arr[j], arr[k]) 满足下列全部条件，则认为它是一个 好三元组 。
//
//     0 <= i < j < k < arr.length
//     |arr[i] - arr[j]| <= a
//     |arr[j] - arr[k]| <= b
//     |arr[i] - arr[k]| <= c
//
// 其中 |x| 表示 x 的绝对值。
//
// 返回 好三元组的数量 。
//
//
//
// 示例 1：
//
// 输入：arr = [3,0,1,1,9,7], a = 7, b = 2, c = 3
// 输出：4
// 解释：一共有 4 个好三元组：[(3,0,1), (3,0,1), (3,1,1), (0,1,1)] 。
//
// 示例 2：
//
// 输入：arr = [1,1,2,2,3], a = 0, b = 0, c = 1
// 输出：0
// 解释：不存在满足所有条件的三元组。
//
//
//
// 提示：
//
//     3 <= arr.length <= 100
//     0 <= arr[i] <= 1000
//     0 <= a, b, c <= 1000
struct Solution;

impl Solution {
    /// 1. not recommand, Brute-force algorithm
    pub fn count_good_triplets_force(arr: Vec<i32>, a: i32, b: i32, c: i32) -> i32 {
        let n = arr.len();
        let mut count = 0;
        for i in 0..n {
            for j in i+1..n {
                for k in j+1..n {
                    if ((arr[i] - arr[j]).abs() <= a) && ((arr[j] - arr[k]).abs() <= b)
                        && ((arr[i] - arr[k]).abs() <= c) {
                         count += 1;
                    }
                }
            }
        }
        count
    }
    // 2. ORDER + MIDDLE + THREE POINTERS.
    /// 枚举 j 和 k，可以确定 Ai 的范围。
    ///
    ///      ∣ Ai − Aj ∣ ≤a 等价于 Aj − a ≤ Ai ≤ Aj+a。
    ///      ∣ Ai −Ak ∣ ≤ c 等价于 Ak − c ≤ Ai ≤Ak + c。
    ///      此外还有 0 ≤ Ai ≤ max(A)。
    ///
    ///  计算这三个范围（区间）的交集，得到 Ai 的范围为
    ///            [max(Aj − a,Ak − c,0),min(Aj + a,Ak + c,max(A))]
    pub fn count_good_triplets(arr: Vec<i32>, a: i32, b: i32, c: i32) -> i32 {
        let mut idx_arr = (0..arr.len()).collect::<Vec<_>>();
        idx_arr.sort_unstable_by_key(|&i| arr[i]);

        let mut res = 0i32;

        for &j in &idx_arr {
            let y = arr[j];
            let mut left = vec![];
            for &i in &idx_arr {
                if i < j && ((arr[i] - y).abs() <= a) {
                    left.push(arr[i]);
                }
            }

            let mut right = vec![];
            for &k in &idx_arr {
                if k > j && ((arr[k] - y).abs() <= b) {
                    right.push(arr[k]);
                }
            }
            let mut k1 = 0;
            let mut k2 = 0;
            for x in left {
                while k2 < right.len() && right[k2] <= x+c {
                    k2 += 1;
                }

                while k1 < right.len() && right[k1] <= x-c {
                    k1 += 1;
                }
                res += (k2 - k1) as i32;
            }

        }
        res
    }
}

fn main() {
    assert_eq!(Solution::count_good_triplets(vec![3,0,1,1,9,7], 7, 2, 3), 4);
    assert_eq!(Solution::count_good_triplets_force(vec![3,0,1,1,9,7], 7,2,3), 4);
}