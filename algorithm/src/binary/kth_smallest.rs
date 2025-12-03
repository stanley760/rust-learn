#[allow(dead_code)]
struct Solution;
#[allow(dead_code)]
impl Solution {
    pub fn kth_smallest(matrix: Vec<Vec<i32>>, k: i32) -> i32 {
        let n = matrix.len();

        let check = |mx: i32| -> bool {
            let mut cnt = 0; // matrix 中的 <= mx 的元素个数
            let mut i = 0;
            let mut j = n as i32 - 1; // 从右上角开始
            while i < n && j >= 0 && cnt < k {
                if matrix[i][j as usize] > mx {
                    j -= 1; // 排除第 j 列
                } else {
                    cnt += j + 1; // 从 matrix[i][0] 到 matrix[i][j] 都 <= mx
                    i += 1;
                }
            }
            cnt >= k
        };

        let mut left = matrix[0][0];
        let mut right = matrix[n - 1][n - 1];
        while left < right {
            let mid = left + (right - left) / 2;
            if check(mid) {
                right = mid;
            } else {
                left = mid + 1;
            }
        }
        right
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_kth_smallest() {
        let matrix = vec![vec![1, 5, 9], vec![10, 11, 13], vec![12, 13, 15]];
        let k = 8;
        assert_eq!(Solution::kth_smallest(matrix, k), 13);
        let matrix = vec![vec![-5]];
        let k = 1;
        assert_eq!(Solution::kth_smallest(matrix, k), -5);
    }
}
