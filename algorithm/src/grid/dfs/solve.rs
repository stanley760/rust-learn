use std::isize;

#[allow(dead_code)]
struct Solution;
#[allow(dead_code)]
impl Solution {
    pub fn solve(board: &mut Vec<Vec<char>>) {
        let m = board.len();
        let n = board[0].len();

        for i in 0..m {
            for j in 0..n {
                let cur = board[i][j];
                // check the bound
                let is_edge = i == 0 || j == 0 || i == m - 1 || j == n - 1;
                if is_edge && cur == 'O' {
                    dfs(board, i as isize, j as isize);
                }
            }
        }
        for i in 0..m {
            for j in 0..n {
                if board[i][j] == 'O' {
                    board[i][j] = 'X';
                }
                if board[i][j] == '#' {
                    board[i][j] = 'O';
                }
            }
        }


        fn dfs(board: &mut Vec<Vec<char>>, i: isize, j: isize) {
            if i < 0
                || i as usize >= board.len()
                || j < 0
                || j as usize >= board[0].len()
                || board[i as usize][j as usize] != 'O'
            {
                return;
            }
            // check top/bottom/left/right bound side
            board[i as usize][j as usize] = '#';

            dfs(board, i + 1, j);
            dfs(board, i - 1, j);
            dfs(board, i, j + 1);
            dfs(board, i, j - 1);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_normal_case() {
        let mut board = vec![
            vec!['X', 'X', 'X', 'X'],
            vec!['X', 'O', 'O', 'X'],
            vec!['X', 'X', 'O', 'X'],
            vec!['X', 'O', 'X', 'X'],
        ];

        let expected = vec![
            vec!['X', 'X', 'X', 'X'],
            vec!['X', 'X', 'X', 'X'],
            vec!['X', 'X', 'X', 'X'],
            vec!['X', 'O', 'X', 'X'],
        ];
        Solution::solve(&mut board);
        assert_eq!(board, expected);
    }

}
