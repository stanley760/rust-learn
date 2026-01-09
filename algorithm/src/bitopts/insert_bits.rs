#[allow(dead_code)]
struct Solution;
#[allow(dead_code)]
impl Solution {

    pub fn insert_bits(n: i32, m: i32, i:i32, j: i32) -> i32 {
        let mut n = n;
        n &= !(((1<< (j - i + 1)) - 1) << i);
        n | (m << i)
    }
}
#[cfg(test)]
mod tests {
    use crate::insert_bits::Solution;

    #[test]
    pub fn test_normal_case() {
        let n = 1024; let m = 19;
        let i = 2; let j = 6;
        assert_eq!(Solution::insert_bits(n, m, i, j), 1100); 
    }
    #[test]
    pub fn test_case_2() {
        let n = 0; let m =31;
        let i = 0; let j = 4;
        assert_eq!(Solution::insert_bits(n, m, i, j), 31);
    }
}