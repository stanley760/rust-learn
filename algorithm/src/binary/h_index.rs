#[allow(dead_code)]
struct Solution;
#[allow(dead_code)]
impl Solution {
    pub fn h_index(citations: Vec<i32>) -> i32 {
        let mut left = 0;
        let mut right = citations.len();

        while left < right {
            let mid = (left + right + 1) >> 1;
            if citations[citations.len() - mid] >= mid as i32 {
                left = mid;
            } else {
                right = mid - 1;
            }
        }
        left as i32
    }
}

#[cfg(test)]
mod tests {
    use crate::binary::h_index::Solution;

    #[test]
    fn test_normal_case() {
        let citations = vec![0,1,3,5,6];
        assert_eq!(Solution::h_index(citations), 3);
    }
}