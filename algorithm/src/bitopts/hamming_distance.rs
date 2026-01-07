#[allow(dead_code)]
struct Solution;
#[allow(dead_code)]
impl Solution {
    pub fn hamming_distance(x: i32, y: i32) -> i32 {
        (x ^ y).count_ones() as i32
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_hamming_distance() {
        let x = 1;
        let y = 4;
        let expected = 2;
        assert_eq!(Solution::hamming_distance(x, y), expected);
    }
    #[test]
    fn test_hamming_distance_same() {
        let x = 3;
        let y = 1;
        let expected = 1;
        assert_eq!(Solution::hamming_distance(x, y), expected);
    }
}