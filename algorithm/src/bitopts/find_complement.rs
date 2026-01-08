#[allow(dead_code)]
struct Solution;
#[allow(dead_code)]
impl Solution {
    pub fn find_complement(num: i32) -> i32 {
        let mut mask = !0;
        while num & mask != 0 {
            mask <<= 1;
        }
        !mask & !num
    }
    pub fn find_complement2(num: i32) -> i32 {
        (1 << (32 - num.leading_zeros())) as i32 - 1 - num
    }

}

#[cfg(test)]
mod tests {
    use super::Solution;
    #[test]
    fn test_find_complement() {         
        assert_eq!(Solution::find_complement(5), 2);
        assert_eq!(Solution::find_complement2(1), 0);
    }
}