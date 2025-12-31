use std::collections::VecDeque;

#[allow(dead_code)]
struct Solution;
const DIRS :[(i32, i32); 4] = [(0, 1), (0, -1), (1, 0), (-1, 0)];
#[allow(dead_code)]
impl Solution {
    pub fn update_matrix(mat: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut mat = mat;
        let m = mat.len();
        let n = mat[0].len();

        let mut queue = VecDeque::new();

        for i in 0..m {
            for j in 0..n {
                if mat[i][j] == 0 {
                    queue.push_back((i, j));
                } else {
                    mat[i][j] = -1;
                }
            }
        }

        while let Some((x, y)) = queue.pop_front() {
            for (dx, dy) in DIRS {
                let tx = x as i32 + dx;
                let ty = y as i32 + dy;
                if tx >= 0 && tx < m as i32 && ty >= 0 && ty < n as i32 {
                    let tx = tx as usize;
                    let ty = ty as usize;
                    if mat[tx][ty] == -1 {
                        mat[tx][ty] = mat[x][y] + 1;
                        queue.push_back((tx, ty));
                    }
                }
            }
        }
        mat
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;


    #[test]
    pub fn test_case_1() {
        let mat = vec![vec![0, 0, 0], vec![0, 1, 0], vec![0, 0, 0]];

        let expected = vec![vec![0,0,0],vec![0,1,0],vec![0,0,0]];

        assert_eq!(Solution::update_matrix(mat), expected);
    }

    #[test]
    pub fn test_case_2() {
        let mat = vec![vec![0, 0, 0], vec![0, 1, 0], vec![1, 1, 1]];

        let expected = vec![vec![0,0,0],vec![0,1,0],vec![1,2,1]];

        assert_eq!(Solution::update_matrix(mat), expected);
    }
}
