#[allow(dead_code)]
struct Solution;
#[allow(dead_code)]
impl Solution {
    pub fn find_kth_number(m: i32, n: i32, k: i32) -> i32 {
        let mut left = 1;
        let mut right = m * n;
        while left < right {
            let mid = (left + right) >> 1;
            let mut count = 0;
            for i in 1..=m {
                count += std::cmp::min(mid / i, n);
            }
            if count >= k {
                right = mid;
            } else {
                left = mid + 1;
            }
        }
        right
        
    }
}
#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_find_kth_number() {
        assert_eq!(Solution::find_kth_number(3, 3, 5), 3);
        assert_eq!(Solution::find_kth_number(2, 3, 6), 6);
    }
}