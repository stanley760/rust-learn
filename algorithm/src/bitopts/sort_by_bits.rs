#[allow(dead_code)]
struct Solution;
#[allow(dead_code)]
impl Solution {
    pub fn sort_by_bits(arr: Vec<i32>) -> Vec<i32> {
        let mut arr = arr;
        arr.sort_by_key(|&x| (x.count_ones(), x));
        arr
    }
}
#[cfg(test)]
mod tests {
    use super::Solution;


    #[test]
    fn test_normal_case() {
        let arr = vec![0, 1, 2, 3, 4, 5, 6, 7, 8];
        let expected = vec![0, 1, 2, 4, 8, 3, 5, 6, 7];
        assert_eq!(Solution::sort_by_bits(arr), expected);
    }
}