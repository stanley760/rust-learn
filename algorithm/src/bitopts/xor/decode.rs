#[allow(dead_code)]
struct Solution;
#[allow(dead_code)]
impl Solution {
    // O(n)
    pub fn decode(encoded: Vec<i32>, first: i32) -> Vec<i32> {
        let mut ans = vec![first];
        let mut first = first;

        for e in encoded {
            first ^= e;
            ans.push(first);
        }
        ans
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;
    #[test]
    pub fn test_normal_case() {
        let encoded = vec![1, 2, 3];
        let first = 1;
        let expected = vec![1, 0, 2, 1];
        assert_eq!(Solution::decode(encoded, first), expected);

        let encoded = vec![6, 2, 7, 3];
        let first = 4;
        let expected = vec![4, 2, 0, 7, 4];

        assert_eq!(Solution::decode(encoded, first), expected);
    }
}
