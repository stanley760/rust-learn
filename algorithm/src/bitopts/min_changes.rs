#[allow(dead_code)]
struct Solution;
#[allow(dead_code)]
impl Solution {
    pub fn min_changes(n: i32, k: i32) -> i32 {
        if n & k != k {
            return -1;
        }
        (n ^ k).count_ones() as _
    }
}

#[cfg(test)]
mod tests {
    use crate::min_changes::Solution;

    #[test]
    pub fn test_normal_case() {
        let (n, k) = (13, 4);
        let expected = 2;
        assert_eq!(Solution::min_changes(n, k), expected);
    }

    #[test]
    pub fn test_value_equals() {
        let (n, k) = (21, 21);
        let expected = 0;
        assert_eq!(Solution::min_changes(n, k), expected);
    }

    #[test]
    pub fn test_less_value() {
        let (n, k) = (14, 13);
        let expected = -1;
        assert_eq!(Solution::min_changes(n, k), expected);
    }
}
