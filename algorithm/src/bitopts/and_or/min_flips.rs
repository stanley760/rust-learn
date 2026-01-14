#[allow(dead_code)]
pub struct Solution;
#[allow(dead_code)]
impl Solution {
    pub fn min_flips(a: i32, b: i32, c: i32) -> i32 {
        let (mut a, mut b, mut c) = (a, b, c);
        let mut ans = 0;

        while a > 0 || b > 0 || c > 0 {
            let x = a & 1;
            let y = b & 1;
            let z = c & 1;
            a >>= 1;
            b >>= 1;
            c >>= 1;
            
            ans += ((x | y) ^ z) + (x & y & !z);
        }
        ans
    }
}
#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    pub fn test_normal_case() {
        let (a, b, c) = (2, 6, 5);
        assert_eq!(Solution::min_flips(a, b, c), 3);
        // let (a, b, c) = (4, 2, 7);
        // assert_eq!(Solution::min_flips(a, b, c), 1);
        // let (a, b, c) = (1, 2, 3);
        // assert_eq!(Solution::min_flips(a, b, c), 0);
        // let (a, b, c) = (7, 7, 7);
        // assert_eq!(Solution::min_flips(a, b, c), 0);
    }
}
