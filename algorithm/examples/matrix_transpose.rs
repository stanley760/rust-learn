struct Solution;

impl Solution {
    pub fn transpose(matrix: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let l = matrix.len();
        let w = matrix[0].len();

        let mut transpose_matrix = vec![vec![0; l];w];

        matrix.into_iter().enumerate().for_each(|(i, r)| {
            r.into_iter().enumerate().for_each(|(j, e)| {
                transpose_matrix[j][i] = e;
            });
        });
        transpose_matrix
    }
}

fn main() {
    let target = vec![vec![1,4,7], vec![2,5,8], vec![3,6,9]];
    let matrix = vec![vec![1,2,3], vec![4,5,6], vec![7,8,9]];
    assert_eq!(target, Solution::transpose(matrix));
}