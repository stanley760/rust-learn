
#[allow(dead_code)]
struct Solution;
#[allow(dead_code)]
impl Solution {
    // deprecated!
    pub fn xor_operation(n: i32, start: i32) -> i32 {
        let mut ans = start;
        (0..n).into_iter().for_each(|x| {
            let cur = start + 2 * x;
            if x > 0 {

                ans ^=cur;
            }
        });
        ans
    }

    // O(1)
    pub fn xor_operation_v1(n: i32, start: i32) -> i32 {
        let xor_n = |n| match n % 4 {
            0 => n,
            1 => 1,
            2 => n + 1,
            _ => 0,
        };
        let a = start / 2;
        let b = n & start & 1;
        (xor_n(a + n - 1) ^ xor_n(a - 1)) * 2 + b
    }
}
#[cfg(test)]
mod tests {
    use crate::xor::xor_operation::Solution;

    #[test]
    pub fn test_normal_case() {
        assert_eq!(Solution::xor_operation(5, 0), 8);
        assert_eq!(Solution::xor_operation(4, 3), 8);
        assert_eq!(Solution::xor_operation(1, 7), 7);
        assert_eq!(Solution::xor_operation(10, 5), 2);
    }
    #[test]
    pub fn test_normal_case_v1() {
        assert_eq!(Solution::xor_operation_v1(5, 0), 8);
        assert_eq!(Solution::xor_operation_v1(4, 3), 8);
        assert_eq!(Solution::xor_operation_v1(1, 7), 7);
        assert_eq!(Solution::xor_operation_v1(10, 5), 2);
    }
}