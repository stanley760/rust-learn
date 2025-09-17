use std::rc::Rc;

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
    // 2. Prefix Sum
    /// 枚举 j 和 k，可以确定 Ai 的范围。
    ///
    ///      ∣ Ai − Aj ∣ ≤a 等价于 Aj − a ≤ Ai ≤ Aj+a。
    ///      ∣ Ai −Ak ∣ ≤ c 等价于 Ak − c ≤ Ai ≤Ak + c。
    ///      此外还有 0 ≤ Ai ≤ max(A)。
    ///
    ///  计算这三个范围（区间）的交集，得到 Ai 的范围为
    ///            [max(Aj − a,Ak − c,0),min(Aj + a,Ak + c,max(A))]
    pub fn count_good_triplets(arr: Vec<i32>, a: i32, b: i32, c: i32) -> i32 {
        todo!()
    }
}

fn main() {
    assert_eq!(Solution::count_good_triplets(vec![3,0,1,1,9,7], 7,2,3), 4);
}