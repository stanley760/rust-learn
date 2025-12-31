#[allow(dead_code)]
struct Solution;

const DX: [isize; 4] = [-1, 0, 1, 0];
const DY: [isize; 4] = [0, 1, 0, -1];

#[allow(dead_code)]
impl Solution {
    pub fn color_border(grid: Vec<Vec<i32>>, row: i32, col: i32, color: i32) -> Vec<Vec<i32>> {
        let mut grid = grid;
        let row = row as usize;
        let col = col as usize;
        let original_color = grid[row][col];
        let mut visited: Vec<Vec<bool>> = vec![vec![false; grid[0].len()]; grid.len()];
        Self::dfs(&mut visited, &mut grid, row, col, original_color, color);
        grid
    }

    fn dfs(visited: &mut Vec<Vec<bool>>,grid: &mut Vec<Vec<i32>>, x: usize, y: usize, original_color: i32, color: i32) {
            
            visited[x][y] = true;

            for i in 0..4 {
                let dx = x as isize + DX[i];
                let dy = y as isize + DY[i];
                let len_row = grid.len() as isize;
                let len_col = grid[0].len() as isize;

                if dx < 0 || dx >= len_row || dy < 0 || dy >= len_col || grid[dx as usize][dy as usize] != original_color {
                    if dx >= 0 && dx < len_row && dy >= 0 && dy < len_col && visited[dx as usize][dy as usize] {
                        continue;
                    }
                    grid[x][y] = color;
                    continue;
                }
                let dx = dx as usize;
                let dy = dy as usize;

                if grid[dx][dy] == original_color && !visited[dx][dy] {
                    Self::dfs(visited, grid, dx, dy, original_color, color);
                }
            }
        }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_normal_case() {
        let grid = vec![vec![1, 1], vec![1, 2]];
        let row = 0;
        let col = 0;
        let color = 3;

        let expected = vec![vec![3, 3], vec![3, 2]];

        assert_eq!(Solution::color_border(grid, row, col, color), expected);
    }

    #[test]
    pub fn tes_case_example_one() {
        let grid = vec![vec![1, 2, 2], vec![2, 3, 2]];
        let row = 0;
        let col = 1;
        let color = 3;

        let expected = vec![vec![1, 3, 3], vec![2, 3, 3]];

        assert_eq!(Solution::color_border(grid, row, col, color), expected);
    }

    #[test]
    pub fn tes_case_example_two() {
        let grid = vec![vec![1, 1, 1], vec![1, 1, 1], vec![1, 1, 1]];
        let row = 1;
        let col = 1;
        let color = 2;

        let expected = vec![vec![2, 2, 2], vec![2, 1, 2], vec![2, 2, 2]];

        assert_eq!(Solution::color_border(grid, row, col, color), expected);
    }
}
