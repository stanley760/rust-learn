use std::i32;

#[allow(dead_code)]
struct Solution;
#[allow(dead_code)]
impl Solution {

    pub fn max_distance(arrays: Vec<Vec<i32>>) -> i32 {
        let mut ans = 0;
        let mut min = i32::MAX / 2;
        let mut max = i32::MIN /2 ;
        for a in arrays {
            // - 每个数组的最小值是 a[0]
            // - 每个数组的最大值是 a[a.len() - 1]
            let x = a[0];
            let y = a[a.len() - 1];

            ans = ans.max(y- min).max(max - x);
            // 记录上一个数组中最小的值和当前较小值中最小的值
            min = min.min(x);
            // 记录上一个数组中最大的和当前较大值中最大的值
            max = max.max(y);
        }
        ans
    }
}

#[cfg(test)]
mod tests {

    use super::Solution;

    #[test]
    pub fn test_normal_case() {
        let nums = vec![vec![1,2,3], vec![4,5], vec![1,2,3]];
        assert_eq!(Solution::max_distance(nums), 4);

        let nums = vec![vec![1], vec![1]];
        assert_eq!(Solution::max_distance(nums), 0);
    }
}