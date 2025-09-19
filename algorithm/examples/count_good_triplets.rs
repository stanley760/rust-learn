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
            for j in i + 1..n {
                for k in j + 1..n {
                    if ((arr[i] - arr[j]).abs() <= a)
                        && ((arr[j] - arr[k]).abs() <= b)
                        && ((arr[i] - arr[k]).abs() <= c)
                    {
                        count += 1;
                    }
                }
            }
        }
        count
    }
    // 2. PREFIX SUM
    /// ```
    /// 枚举 j 和 k，可以确定 Ai的范围。
    ///
    ///     ∣Ai − Aj∣≤a 等价于 Aj−a≤Ai≤Aj+a。
    ///     ∣Ai−Ak∣≤c 等价于 Ak−c≤Ai≤Ak+c。
    ///     此外还有 0≤Ai≤max(A)。
    ///
    /// 计算这三个范围（区间）的交集，得到 Ai 的范围为
    /// [max(Aj−a,Ak−c,0),min(Aj+a,Ak+c,max(A))]
    /// ```
    pub fn count_good_triplets_prefix_sum(arr: Vec<i32>, a: i32, b: i32, c: i32) -> i32 {
        let max = *arr.iter().max().unwrap();
        let mut s = vec![0; (max + 2) as usize];
        let mut res = 0;

        arr.iter().enumerate().for_each(|(j, &y)| {
            arr[j + 1..].iter().for_each(|&z| {
                if (y - z).abs() > b {
                    return;
                }
                let l = (y - a).max(z - c).max(0);
                let r = (y + a).min(z + c).min(max);
                // l > r + 1, s[r + 1] - s[l]  < 0
                res += 0.max(s[(r + 1) as usize] - s[l as usize]);
            });
            (y + 1..max + 2).for_each(|v| {
                s[v as usize] += 1;
            });
        });

        res
    }

    // 3. ORDER + MIDDLE + THREE POINTERS.
    // x: arr[i] y: arr[j] z: arr[k]
    // 1. firstly, loop all elements of the middle array which is arr[j];
    pub fn count_good_triplets(arr: Vec<i32>, a: i32, b: i32, c: i32) -> i32 {
        let mut idx_arr = (0..arr.len()).collect::<Vec<_>>();
        idx_arr.sort_unstable_by_key(|&i| arr[i]);

        let mut res = 0i32;

        for &j in &idx_arr {
            let y = arr[j];
            // 收集满足条件的左侧元素 (i < j 且 |arr[i] - arr[j]| <= a)
            let left: Vec<i32> = idx_arr
                .iter()
                .filter(|&&i| (i < j) && (arr[i] - y).abs() <= a)
                .map(|&i| arr[i])
                .collect();

            // 收集满足条件的右侧元素 (k > j 且 |arr[k] - arr[j]| <= b)
            let right: Vec<i32> = idx_arr
                .iter()
                .skip_while(|&&i| i <= j)
                .filter(|&&k| (k > j) && (arr[k] - y).abs() <= b)
                .map(|&k| arr[k])
                .collect();

            // 使用双指针技术计算满足 |x - z| <= c 的组合数
            let mut k1 = 0;
            let mut k2 = 0;
            for x in left {
                while k2 < right.len() && right[k2] <= x + c {
                    k2 += 1;
                }

                while k1 < right.len() && right[k1] <= x - c {
                    k1 += 1;
                }
                res += (k2 - k1) as i32;
            }
        }
        res
    }
}

fn main() {
    assert_eq!(
        Solution::count_good_triplets(vec![3, 0, 1, 1, 9, 7], 7, 2, 3),
        4
    );
    assert_eq!(
        Solution::count_good_triplets_prefix_sum(vec![3, 0, 1, 1, 9, 7], 7, 2, 3),
        4
    );
    assert_eq!(
        Solution::count_good_triplets_force(vec![3, 0, 1, 1, 9, 7], 7, 2, 3),
        4
    );
}
