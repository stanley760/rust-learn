struct Solution;

impl Solution {
    pub fn find_max_average(nums: Vec<i32>, k: i32) -> f64 {
        let k = k as usize;
        let mut sum = 0;
        let mut ans = i32::MIN;
        nums.iter().enumerate().for_each(|(i, &e)| {
            // 1. right node come in window
            sum += e;
            // 2. curren window size is less than k
            if i + 1 < k {
                return;
            }

            ans = ans.max(sum);
            // 3. left node leave window
            sum -= nums[i + 1 - k];
        });

        ans as f64 / k as f64
    }
}

fn main() {
    let nums = vec![1, 12, -5, -6, 50, 3];
    let k = 4;
    println!("{:?}", Solution::find_max_average(nums, k));
}
