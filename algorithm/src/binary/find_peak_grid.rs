#[allow(dead_code)]
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn find_peak_grid(mat: Vec<Vec<i32>>) -> Vec<i32> {
        let (mut left, mut right) = (0, mat.len() -1);

        while left < right {
            let i = (left + right) >> 1;
            let j = mat[i].iter().enumerate()
                    .max_by_key(|(_, &value)| value)
                    .map(|(index, _)| index).unwrap_or(0);
            if mat[i][j] > mat[i + 1][j] {
                right = i;
            } else {
                left = i + 1;
            }
        } 
        let r = mat[left].iter().enumerate()
                    .max_by_key(|(_, &value)| value)
                    .map(|(index, _)| index).unwrap_or(0);
        return vec![left as _, r as _];
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_normal_case() {

       let mat = vec![vec![1,4],vec![3,2]];

       assert_eq!(Solution::find_peak_grid(mat), vec![0,1]);
    }

    #[test]
    fn test_normal_case_solution2() {
        let mat = vec![vec![10,20,15], vec![21,30,14], vec![7,16,32]];
        assert_eq!(Solution::find_peak_grid(mat), vec![1, 1]);
    }
}