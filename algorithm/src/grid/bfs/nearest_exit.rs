use std::collections::VecDeque;

#[allow(dead_code)]
struct Solution;

const DIRS: [(i32, i32); 4] = [(0, -1), (0, 1), (1, 0), (-1, 0)];
#[allow(dead_code)]
impl Solution {
    pub fn nearest_exit(maze: Vec<Vec<char>>, entrance: Vec<i32>) -> i32 {
        let m = maze.len();
        let n = maze[0].len();
        let mut visited = vec![vec![false; n]; m];

        let sx = entrance[0];
        let sy = entrance[1];

        visited[sx as usize][sy as usize] = true;
        let mut queue = VecDeque::new();
        queue.push_back((sx as i32, sy as i32));
        let mut ans = 0;
        while !queue.is_empty() {
            let level_size = queue.len();
            
            for _ in 0..level_size {
                let (cx, cy) = queue.pop_front().unwrap();
                for d in DIRS {
                    let x_i32 = cx + d.0;
                    let y_i32 = cy + d.1;
                    let m_i32 = m as i32;
                    let n_i32 = n as i32;

                    if x_i32 >= 0 && x_i32 < m_i32  && 0 <= y_i32 && y_i32 < n_i32 {
                        let x = x_i32 as usize;
                        let y = y_i32 as usize;
                        if maze[x][y] == '.' && !visited[x][y] {
                            // Check if it's an exit
                            if x == 0 || y == 0 || x == m - 1 || y == n - 1 {
                                
                                return ans + 1;
                            } 
                        
                            visited[x][y] = true;
                            queue.push_back((x_i32, y_i32));
                        }
                        
                    }
                }
            }
            ans += 1;
        }
        -1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_case1() {
        let maze = vec![
            vec!['+', '+', '.', '+'],
            vec!['.', '.', '.', '+'],
            vec!['+', '+', '+', '.'],
        ];
        let entrance = vec![1, 2];

        assert_eq!(Solution::nearest_exit(maze, entrance), 1);
    }

    #[test]
    pub fn test_case_2() {
        let maze = vec![
            vec!['+', '+', '+'],
            vec!['.', '.', '.'],
            vec!['+', '+', '+'],
        ];
        let entrance = vec![1, 0];

        assert_eq!(Solution::nearest_exit(maze, entrance), 2);
    }

    #[test]
    pub fn test_case_3() {
        let maze = vec![vec!['.', '+']];
        let entrance = vec![0, 0];
        assert_eq!(Solution::nearest_exit(maze, entrance), -1);
    }
}
