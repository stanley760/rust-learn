#[allow(dead_code)]
struct Solution; 
#[allow(dead_code)]
impl Solution {

    pub fn min_end(n:i32, x:i32) -> i64 {
        let mut n = n;
        // n -1
        n -= 1;
        let mut ans = x as i64;
        let (mut i, mut j) = (0, 0);
        while (n >> j) > 0 {
            // the i-th bit of x is zero . 
            if (ans >>i &1) == 0 {
                // n >> j & 1  is filled with the j-th bit of n
                ans |= ((n >>j & 1) <<i) as i64;
                j += 1;
                
            }
            i += 1;
        }
        ans
    }
}
#[cfg(test)]
mod tests {
    use crate::and_or::min_end::Solution;


    #[test]
    pub fn test_normal_case() {
        let n = 3; let x =4;
        assert_eq!(Solution::min_end(n, x), 6);

        let n = 2; let x =7;
        assert_eq!(Solution::min_end(n, x), 15);
    }
}