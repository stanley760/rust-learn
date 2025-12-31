#[allow(dead_code)]
struct Solution;
#[allow(dead_code)]
impl Solution {
    pub fn closed_island(grid: Vec<Vec<i32>>) -> i32 {
        let mut grid = grid;
        let m = grid.len();
        let n = grid[0].len();

        if m < 3 || n < 3 {
            return 0;
        }

        for i in 0..m {
            let step = if i == 0 || i == m - 1 { 1 } else { n - 1 };

            let mut j = 0;
            while j < n {
                dfs(&mut grid, i as isize, j as isize);
                j += step;
            }
        }

        let mut ans = 0;
        for i in 1..m - 1 {
            for j in 1..n - 1 {
                if grid[i][j] == 0 {
                    ans += 1;
                    dfs(&mut grid, i as isize, j as isize);
                }
            }
        } 

        fn dfs(grid: &mut Vec<Vec<i32>>, i: isize, j: isize) {
            if i < 0 || i as usize >= grid.len() || j < 0 || j as usize >= grid[0].len() || grid[i as usize][j as usize] != 0 {
                return;
            }
            grid[i as usize][j as usize] = 1;
            dfs(grid, i - 1, j);
            dfs(grid, i + 1, j);
            dfs(grid, i, j - 1);
            dfs(grid, i, j + 1);
        }
        ans 
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_normal_case() {
        let grid = vec![
            vec![1, 1, 1, 1, 1, 1, 1, 0],
            vec![1, 0, 0, 0, 0, 1, 1, 0],
            vec![1, 0, 1, 0, 1, 1, 1, 0],
            vec![1, 0, 0, 0, 0, 1, 0, 1],
            vec![1, 1, 1, 1, 1, 1, 1, 0],
        ];

        let expected = 2;

        assert_eq!(Solution::closed_island(grid), expected);
    }
}
