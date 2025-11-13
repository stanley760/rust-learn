#[allow(dead_code)]
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn count_fair_pairs(nums: Vec<i32>, lower: i32, upper: i32) -> i64 {
        let mut nums = nums;
        nums.sort_unstable();

        let cnt = |bound| -> i64 {
            let mut res = 0i64;
            let mut j = nums.len() - 1;
            for (i, &x) in nums.iter().enumerate() {
                while j > i && nums[j] + x > bound {
                    j -= 1;
                }
                if j == i {
                    break;
                }
                res += (j - i) as i64;
            }
            res
        };

        cnt(upper) - cnt(lower - 1)
    }

    fn count_fair_pairs_binary(nums: Vec<i32>, lower: i32, upper: i32) -> i64 {
        let mut nums = nums;
        nums.sort_unstable();

        let mut ans = 0i64;

        for j in 0..nums.len() {
            let x = nums[j];
            let left = nums[..j].partition_point(|&y| y + x < lower);
            let right = nums[..j].partition_point(|&y| y + x <= upper);
            ans += (right - left) as i64;
        }

        ans
    }
}
// cargo test --package algorithm --lib -- binary::count_fair_pairs::tests --nocapture
#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_count_fair_pairs() {
        let nums = vec![0, 1, 7, 4, 4, 5];
        let lower = 3;
        let upper = 6;
        assert_eq!(Solution::count_fair_pairs(nums.clone(), lower, upper), 6);
        assert_eq!(
            Solution::count_fair_pairs_binary(nums.clone(), lower, upper),
            6
        );

        let nums = vec![1, 7, 9, 2, 5];
        let lower = 11;
        let upper = 11;
        assert_eq!(Solution::count_fair_pairs(nums.clone(), lower, upper), 1);
        assert_eq!(
            Solution::count_fair_pairs_binary(nums.clone(), lower, upper),
            1
        );
    }
}
