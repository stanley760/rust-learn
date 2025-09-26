use std::collections::HashMap;

pub struct Solution;

impl Solution {
    pub fn maximum_subarray_sum(nums: Vec<i32>, k: i32) -> i64 {
        let mut sum = 0i64;
        let mut ans = 0i64;
        let mut cnt = HashMap::new();

        for (i, &e) in nums.iter().enumerate() {
            sum += e as i64;
            *cnt.entry(e).or_insert(0) += 1;

            if i < (k - 1) as usize {
                continue;
            }
            if cnt.len() == k as usize {
                ans = ans.max(sum);
            }


            let out = nums[i + 1 - k as usize];
            sum -= out as i64;
            let c = cnt.entry(out).or_insert(0);
            *c -= 1;
            if *c == 0 {
                cnt.remove(&out);
            }

        }
        ans
    }
}