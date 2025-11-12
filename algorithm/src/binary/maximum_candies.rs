#[allow(dead_code)]
struct Solution;
#[allow(dead_code)]
impl Solution {
    pub fn maximum_candies(candies: Vec<i32>, k: i64) -> i32 {
        let mut left = 0;
        let mut right = (candies.iter().map(|&c| c as i64).sum::<i64>() / k) as i32;
        while left < right {
            let mid = (left + right + 1) >> 1;
            let count = candies.iter().map(|&c| (c / mid) as i64).sum::<i64>();
            if count >= k as _ {
                left = mid;
            } else {
                right = mid -1;
            }
        }
        left
    }
}


#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_maximum_candies() {
        let candies = vec![5,8,6];
        let k = 3;
        assert_eq!(Solution::maximum_candies(candies, k), 5);

        let candies = vec![2,5];
        let k = 11;
        assert_eq!(Solution::maximum_candies(candies, k), 0);
    }
}
