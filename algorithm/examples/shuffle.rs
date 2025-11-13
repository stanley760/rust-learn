struct Solution;

impl Solution {
    pub fn shuffle(nums: Vec<i32>, n: i32) -> Vec<i32> {
        let x = nums.iter().take(n as usize).collect::<Vec<_>>();
        let y = nums.iter().skip(n as usize).collect::<Vec<_>>();
        let mut new_arr = Vec::with_capacity(n as usize);
        for i in 0..n {
            new_arr.push(*x[i as usize]);
            new_arr.push(*y[i as usize]);
        }
        new_arr
    }
}

fn main() {
    let nums = vec![2, 5, 1, 3, 4, 7];
    let n = 3;
    let target = vec![2, 3, 5, 4, 1, 7];
    assert_eq!(target, Solution::shuffle(nums, n));
}
