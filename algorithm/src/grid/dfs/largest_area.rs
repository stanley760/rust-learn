#[allow(dead_code)]
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn largest_area(grid: Vec<String>) -> i32 {
        let mut grid = grid.iter()
            .map(|g| g.chars().into_iter().map(|c| (c as u8 - b'0') as i32).collect::<Vec<_>>())
            .collect::<Vec<_>>();
        fn dfs(grid: &mut Vec<Vec<i32>>, i: isize, j: isize, ch: i32) -> i32 {
            // out of bound
            if i < 0 || i >= (grid.len() as isize) || j < 0 || j >= (grid[0].len() as isize)  {
                 return -1111;
            }
            let i = i as usize;
            let j = j as usize;
            // water or visited
            if grid[i][j] == 0 {
                return -1111;
            } 
            if grid[i][j] == 6 {
                return 0;
            }
            let mut total = 0;
            if grid[i][j] == ch {
                total += 1;
                grid[i][j] = 6;
                total += dfs(grid, i as isize + 1 , j as isize, ch);
                total += dfs(grid, i as isize - 1, j as isize, ch);
                total += dfs(grid, i as isize, j as isize + 1, ch);
                total += dfs(grid, i as isize, j as isize - 1 , ch);
                
            } 
            total
            
        }

        let mut ans = 0;
        for i in 0..grid.len() {
            for j in 0..grid[0].len() {
                let ch = grid[i][j];
                if ch != 0 {
                    ans = ans.max(dfs(&mut grid, i as isize, j as isize, ch));
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
        let grid = vec!["110", "231", "221"];
        let grid = grid.into_iter().map(|g| g.to_string()).collect::<Vec<_>>();

        assert_eq!(Solution::largest_area(grid), 1);
    }
}
