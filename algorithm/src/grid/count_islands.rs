#[allow(dead_code)]
struct Solution;

const DIRS: [[i32; 2]; 4] = [[0, 1], [0, -1], [1, 0], [-1, 0]];
#[allow(dead_code)]
impl Solution {
    // fn dfs(grid:&mut Vec<Vec<i32>>, i: usize, j: usize) -> i64 {
    //     let mut total = 0;
    //     if i >= grid.len() || j >= grid[0].len() || grid[i][j] <= 0 {
    //         return total;
    //     }
    //     total = grid[i][j] as i64;

    //     grid[i][j] = 0;

    //     total += dfs(grid, i, j + 1);
    //     total += dfs(grid, i, j.wrapping_sub(1));
    //     total += dfs(grid, i + 1, j);
    //     total += dfs(grid, i.wrapping_sub(1), j);

    //     total
    // }
    pub fn count_islands(grid: Vec<Vec<i32>>, k: i32) -> i32 {
        let mut grid = grid;

        let mut ans = 0;
        for i in 0..grid.len() {
            for j in 0..grid[i].len() {
                if grid[i][j] > 0 && Self::dfs(&mut grid, i as i32, j as i32) % (k as i64) == 0 {
                    ans += 1;
                }
            }
        }
        ans
    }

    fn dfs(grid: &mut Vec<Vec<i32>>, i: i32, j: i32) -> i64 {
        let mut res = grid[i as usize][j as usize] as i64;

        grid[i as usize][j as usize] = 0;
        for d in DIRS {
            let x = i + d[0];
            let y = j + d[1];
            if x >= 0
                && (x as usize) < grid.len()
                && y >= 0
                && (y as usize) < grid[x as usize].len()
                && grid[x as usize][y as usize] > 0
            {
                res += Self::dfs(grid, x, y);
            }
        }
        res
    }
}

//

#[cfg(test)]
mod tests {
    use crate::grid::count_islands::Solution;

    #[test]
    pub fn test_normal_case() {
        let grid = [
            [0, 2, 1, 0, 0],
            [0, 5, 0, 0, 5],
            [0, 0, 1, 0, 0],
            [0, 1, 4, 7, 0],
            [0, 2, 0, 0, 8],
        ];
        let k = 5;

        let grid = grid.iter().map(|g| g.to_vec()).collect::<Vec<_>>();

        assert_eq!(2, Solution::count_islands(grid, k));

        let grid = [[3, 0, 3, 0], [0, 3, 0, 3], [3, 0, 3, 0]];
        let k = 3;

        let grid = grid.iter().map(|g| g.to_vec()).collect::<Vec<_>>();

        assert_eq!(6, Solution::count_islands(grid, k));
    }
}
