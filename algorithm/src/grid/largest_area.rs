#[allow(dead_code)]
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn largest_area(grid: Vec<String>) -> i32 {
        let mut grid = grid
            .iter()
            .map(|g| {
                g.chars()
                    .into_iter()
                    .map(|c| (c as u8 - b'0') as i32)
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();

        fn dfs(grid: &mut Vec<Vec<i32>>, i: usize, j: usize, ch: i32) -> i32 {
            // exclude cells that are 0 or adjacent to 0.
            if i >= grid.len() || j >= grid[0].len() || grid[i][j] == 0  {
                return i32::MIN;
            }
            // avoid reprocessing or including zero-valued cells in the result, 
            // especially since the visited marker is 6
            if grid[i][j] == 6 {
                return 0;
            }
            let mut area: i32 = 0;
           
            // notice: the condition
            if grid[i][j] == ch {
                area += 1;
                grid[i][j] = 6;
                area += dfs(grid, i, j + 1, ch);
                area += dfs(grid, i, j.wrapping_sub(1), ch);
                area += dfs(grid, i + 1, j, ch);
                area += dfs(grid, i.wrapping_sub(1), j, ch);
            }
            area
        }
        let mut ans: i32 = 0;
        for i in 0..grid.len() {
            for j in 0..grid[i].len() {
                let ch = grid[i][j];
                if ch != 0 {
                    ans = ans.max(dfs(&mut grid, i, j, ch));
                }
            }
        }
        ans
    }
}

#[cfg(test)]
mod tests {
    use crate::grid::largest_area::Solution;

    #[test]
    pub fn test_normal_case() {
        let grid = vec!["110", "231", "221"];
        let grid = grid.into_iter().map(|g| g.to_string()).collect::<Vec<_>>();

        assert_eq!(Solution::largest_area(grid), 1);
    }
}
