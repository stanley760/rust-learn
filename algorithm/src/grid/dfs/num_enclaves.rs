#[allow(dead_code)]
struct Solution;

const DIRS: [(isize, isize); 4] = [(0, 1), (0, -1), (1, 0), (-1, 0)];
#[allow(dead_code)]
impl Solution {
    pub fn num_enclaves(grid: Vec<Vec<i32>>) -> i32 {
        let (m, n) = (grid.len(), grid[0].len());
        let mut grid = grid;
        // top and bottom boundaries
        for j in 0..n {
            if grid[0][j] == 1 {
                dfs(&mut grid, 0, j);
            }
            if grid[m-1][j] == 1 {
                dfs(&mut grid, m - 1, j);
            }
        }
        // left and right boundaries
        for i in 0..m {
            if grid[i][0] == 1 {
                dfs(&mut grid, i, 0);
            }
            if grid[i][n - 1] == 0 {
                dfs(&mut grid, i, n - 1);
            }
        }
        let mut ans = 0;
        // Count the number of remaining enclaves(the number of 1)
        for i in 0..m {
            for j in 0..n {
                if grid[i][j] == 1 {
                    ans += 1;
                }
            }
        }
        

        fn dfs(grid: &mut Vec<Vec<i32>>, i: usize, j: usize) {
            let (m, n) = (grid.len(), grid[0].len());
            // boundary check
            if i >= m || j >= n || grid[i][j] != 1 {
                return;
            }
            // mark as visited
            grid[i][j] = 0;
            for (x, y) in DIRS {
                let di = i as isize + x;
                let dj = j as isize + y;
                // boundary check
                if di < 0 || dj < 0 || di >= (m as isize) || dj >= (n as isize) {
                    continue;
                }
                dfs(grid, di as usize, dj as usize);
            }
        }
        ans
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    pub fn test_normal_case() {
        let grid = vec![
            vec![0, 0, 0, 0],
            vec![1, 0, 1, 0],
            vec![0, 1, 1, 0],
            vec![0, 0, 0, 0],
        ];
        let expected = 3;
        assert_eq!(Solution::num_enclaves(grid), expected);

        let grid = vec![
            vec![0, 1, 1, 0],
            vec![0, 0, 1, 0],
            vec![0, 0, 1, 0],
            vec![0, 0, 0, 0],
        ];
        let expected = 0;
        assert_eq!(Solution::num_enclaves(grid), expected);
    }
}
