use std::vec;
#[allow(dead_code)]
pub struct Solution;
const DIRS : [(i32, i32); 4] = [(-1, 0), (1, 0), (0, 1), (0, -1)];
#[allow(dead_code)]
impl Solution {
    pub fn oranges_rotting(grid: Vec<Vec<i32>>) -> i32 {
        let mut grid = grid;
        let m = grid.len();
        let n = grid[0].len();
        // count all of fresh oranges.
        let mut fresh = 0;
        let mut q = vec![];

        for i in 0..m {
            for j in 0..n {
                if grid[i][j] == 1 {
                    fresh += 1;
                } else if grid[i][j] == 2 {
                    q.push((i, j));
                }
            }
        }
        let mut ans = 0;
        // The condition fresh > 0 avoids running the loop one extra time.
        while fresh > 0 && !q.is_empty() {
            ans += 1;
            let mut next = vec![];
            for (x, y) in q {
                for (dx, dy) in DIRS {
                    let i = x as i32 + dx;
                    let j = y as i32 + dy;
                    if i >= 0 && i < m as i32 && j >= 0 && j < n as i32 {
                        let (i, j) = (i as usize, j as usize);
                        if grid[i][j] == 1 {
                            fresh -= 1;
                            grid[i][j] = 2;
                            next.push((i, j));
                        }
                    }
                }
            }
            q = next;
        }
        let res = if fresh == 0 { ans } else { -1 };
        res
    }
}
#[cfg(test)]
mod tests {

    #[test]
    pub fn test_normal_case() {
        let grid = vec![vec![2, 1, 1], vec![1, 1, 0], vec![0, 1, 1]];
        let expected = 4;
        assert_eq!(super::Solution::oranges_rotting(grid), expected);
    }
}
