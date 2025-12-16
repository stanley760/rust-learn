#[allow(dead_code)]
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn max_area_of_island(grid: Vec<Vec<i32>>) -> i32 {
        let mut grid = grid;
        fn dfs(grid: &mut Vec<Vec<i32>>, i: usize, j: usize) -> i32 {
            if i >= grid.len() || j >= grid[0].len() || grid[i][j] != 1 {
                return 0;
            }
            grid[i][j] = 2;
            let mut total = 1;

            total += dfs(grid, i, j + 1);
            total += dfs(grid, i, j.wrapping_sub(1));
            total += dfs(grid, i + 1, j);
            total += dfs(grid, i.wrapping_sub(1), j);

            total
        }

        let mut ans = 0;
        for i in 0..grid.len() {
            for j in 0..grid[i].len() {
                ans = ans.max(dfs(&mut grid, i, j));
            }
        }
        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    pub fn test_normal_case() {
        let grid = [
            [0, 0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 0, 0, 0],
            [0, 1, 1, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0],
            [0, 1, 0, 0, 1, 1, 0, 0, 1, 0, 1, 0, 0],
            [0, 1, 0, 0, 1, 1, 0, 0, 1, 1, 1, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0],
        ];
        let grid = grid.iter().map(|g| g.to_vec()).collect::<Vec<_>>();

        assert_eq!(Solution::max_area_of_island(grid), 6);

        let grid = [[0,0,0,0,0,0,0,0]];
        let grid = grid.iter().map(|g| g.to_vec()).collect::<Vec<_>>();

        assert_eq!(Solution::max_area_of_island(grid), 0);
    }
}
