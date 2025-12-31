#[allow(dead_code)]
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn find_max_fish(grid: Vec<Vec<i32>>) -> i32 {
        let mut grid = grid;

        fn dfs(grid: &mut Vec<Vec<i32>>, i: usize, j: usize) -> i32 {
            if i >= grid.len() || j >= grid[0].len() || grid[i][j] == 0 {
                return 0;
            }
            let mut total = grid[i][j];
            grid[i][j] = 0;
            total += dfs(grid, i, j + 1);
            if j > 0 {
                total += dfs(grid, i, j - 1);
            }
            total += dfs(grid, i + 1, j);
            if i > 0 {
                total += dfs(grid, i - 1, j);
            }

            total
        }

        let mut ans = 0;
    
        for i in 0..grid.len() {
            for j in 0..grid[0].len() {
                // the condition that 
                if grid[i][j] > 0 {
                    ans = ans.max(dfs(&mut grid, i, j));
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
    pub fn test_normal_case() {
        let grid: [[i32; 4]; 4] = [[0,2,1,0],[4,0,0,3],[1,0,0,4],[0,3,2,0]];
        let grid: Vec<Vec<i32>> = grid.iter().map(|g| g.to_vec()).collect::<Vec<_>>();

        assert_eq!(Solution::find_max_fish(grid), 7);
        
        let grid = vec![vec![1,0,0,0],vec![0,0,0,0],vec![0,0,0,0],vec![0,0,0,1]];
        assert_eq!(Solution::find_max_fish(grid), 1);
    }
}