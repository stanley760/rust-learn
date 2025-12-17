#[allow(dead_code)]
struct Solution;

#[allow(dead_code)]
impl Solution {

    pub fn island_perimeter(grid: Vec<Vec<i32>>) -> i32 {
        let m = grid.len();
        let n = grid[0].len();
        let mut ans = 0;
        for (i, row) in grid.iter().enumerate() {
            for (j, &r) in row.iter().enumerate() {
                if r == 0 {
                    continue;
                }
                if i == 0 || grid[i - 1][j] == 0 {
                    ans +=1;
                }
                if i == m -1 || grid[i + 1][j] == 0 {
                    ans += 1;
                }
                if j ==0 || grid[i][j - 1] == 0 {
                    ans += 1;
                }
                if j == n - 1 || grid[i][j + 1] ==0 {
                    ans += 1;
                }
            }
        }
        ans
    }
}

#[cfg(test)]
mod tests {
    use crate::grid::island_perimeter::Solution;

    #[test]
    pub fn test_normal_case() {
        let grid = [[0,1,0,0],[1,1,1,0],[0,1,0,0],[1,1,0,0]];
        let grid = grid.iter().map(|g| g.to_vec()).collect();
        assert_eq!(Solution::island_perimeter(grid), 16);
    }

    #[test]
    pub fn test_bound_condition() {
        let grid = vec![vec![1]];
        assert_eq!(Solution::island_perimeter(grid), 4);
        let grid = vec![vec![1,0]];
        assert_eq!(Solution::island_perimeter(grid), 4);
    }
}