#[allow(dead_code)]
struct Solution;
#[allow(dead_code)]
impl Solution {
    pub fn search_matrix(matrix: Vec<Vec<i32>>, target: i32) -> bool {
        let m = matrix.len();
        let n = matrix[0].len();
        let mut left = 0;
        let mut right = m * n;
        while left < right {
            let mid = (left + right) / 2;
            let val = matrix[mid / n][mid % n];
            if val == target {
                return true;
            } else if val < target {
                left = mid + 1;
            } else {
                right = mid;
            }
        }
        false
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_search_matrix() {
        let matrix = vec![
            vec![1, 3, 5, 7],
            vec![10, 11, 16, 20],
            vec![23, 30, 34, 60],
        ];
        assert_eq!(Solution::search_matrix(matrix.clone(), 3), true);
        assert_eq!(Solution::search_matrix(matrix.clone(), 13), false);
    }
}
