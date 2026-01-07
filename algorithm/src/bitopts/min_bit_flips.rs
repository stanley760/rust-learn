#[allow(dead_code)]
struct Solution;
#[allow(dead_code)]
impl Solution {
     pub fn min_bit_flips(start: i32, goal: i32) -> i32 {
        (start ^ goal).count_ones() as _
    }
}

#[cfg(test)]
mod tests {
    use crate::min_bit_flips::Solution;

    #[test]
    pub fn test_normal_case() {
        let start = 10;
        let goal = 7;
        let expected = 3;

        assert_eq!(Solution::min_bit_flips(start, goal), expected);

        let start = 3;
        let goal = 4;
        let expected = 3;
        assert_eq!(Solution::min_bit_flips(start, goal), expected);
    }


}