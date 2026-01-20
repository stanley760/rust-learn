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
            let x = a[0];
            let y = a[a.len() - 1];

            ans = ans.max(y- min).max(max - x);
            min = min.min(x);
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