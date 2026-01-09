#[allow(dead_code)]
struct Solution;
#[allow(dead_code)]
impl Solution {
    pub fn even_odd_bit(n: i32) -> Vec<i32> {
        let mut ans = vec![0, 0];
        let mut idx = 0;
        let mut n = n;
        while n > 0 {
            ans[idx % 2] += n & 1;
            idx += 1;
            n >>= 1;
        }
        ans
    }

    pub fn even_odd_bit_v2(n: i32) -> Vec<i32> {
        let mask = 0x55555555;
        let even_bits = n & mask;
        let odd_bits = n & !mask;
        vec![even_bits.count_ones() as i32, odd_bits.count_ones() as i32]
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;
    #[test]
    fn test_even_odd_bit() {
        assert_eq!(Solution::even_odd_bit(50), vec![1, 2]);
        assert_eq!(Solution::even_odd_bit(2), vec![0, 1]);
    }
    #[test]
    fn test_even_odd_bit_v2() {
        assert_eq!(Solution::even_odd_bit_v2(50), vec![1, 2]);
        assert_eq!(Solution::even_odd_bit_v2(2), vec![0, 1]);
    }
}
