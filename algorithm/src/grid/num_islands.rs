struct Solution;

// set a flag that represents to visited.
impl Solution {
    pub fn num_islands(grid: Vec<Vec<char>>) -> i32 {
        let mut grid = grid;

        fn dfs(grid: &mut Vec<Vec<char>>, i: usize, j: usize) {
            // Check if the value that represents an island
            // is out of bounds or fails to meet the condition
            if i >= grid.len() || j >= grid[i].len() || grid[i][j] != '1' {
                return;
            }
            grid[i][j] = '2';
            dfs(grid, i, j + 1);
            dfs(grid, i, j.wrapping_sub(1));
            dfs(grid, i + 1, j);
            dfs(grid, i.wrapping_sub(1), j);
        }

        let mut ans = 0;
        for i in 0..grid.len() {
            for j in 0..grid[i].len() {
                if grid[i][j] == '1' {
                    dfs(&mut grid, i, j);
                    ans += 1;
                }
            }
        }
        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_normal_case() {
        let grid = vec![
            vec!['1', '1', '1', '1', '0'],
            vec!['1', '1', '0', '1', '0'],
            vec!['1', '1', '0', '0', '0'],
            vec!['0', '0', '0', '0', '0']
        ];

        assert_eq!(Solution::num_islands(grid), 1);

        let grid = vec![
            vec!['1', '1', '0', '0', '0'],
            vec!['1', '1', '0', '0', '0'],
            vec!['0', '0', '1', '0', '0'],
            vec!['0', '0', '0', '1', '1']
        ];

        assert_eq!(Solution::num_islands(grid), 3);
    }
}
