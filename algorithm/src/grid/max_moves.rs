#[allow(dead_code)]
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn max_moves(grid: Vec<Vec<i32>>) -> i32 {
        let mut grid = grid;
        let m = grid.len();
        let mut ans = 0;
        for i in 0..m {
            // move from the first column.
            dfs(&mut grid, &mut ans, i, 0);
        }

        fn dfs(grid: &mut Vec<Vec<i32>>, ans: &mut usize, i: usize, j: usize) {
            // it always move by the step of one column, the result is moved by the 
            if j > *ans {
                *ans = j;
            }
            // 
            if *ans == grid[0].len() - 1 {
                return;
            }
            for x in i.saturating_sub(1)..grid.len().min(i + 2) {
                if grid[x][j + 1] > grid[i][j] {
                    dfs(grid, ans, x, j + 1);
                }
            }
            grid[i][j] = 0;
        }
        ans as _
    }
}

#[cfg(test)]
mod tests {
    use crate::grid::max_moves::Solution;

    #[test]
    pub fn test_normal_case() {
        let grid = vec![
            vec![2, 4, 3, 5],
            vec![5, 4, 9, 3],
            vec![3, 4, 2, 11],
            vec![10, 9, 13, 15],
        ];

        let expected = 3;

        assert_eq!(Solution::max_moves(grid), expected);

        let grid = vec![vec![3, 2, 4], vec![2, 1, 9], vec![1, 1, 7]];

        let expected = 0;
        assert_eq!(Solution::max_moves(grid), expected);
    }
}
