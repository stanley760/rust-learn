#[allow(dead_code)]
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn maximal_square(matrix: Vec<Vec<char>>) -> i32 {
        let n = matrix[0].len();
        let mut heights = vec![0; n + 1];
        let mut ans = 0;
        for row in matrix {
            for (j, column) in row.into_iter().enumerate() {
                if column == '0' {
                    heights[j] = 0;
                } else {
                    heights[j] += 1;
                }
            }
            ans = ans.max(Self::largest_area(&heights));
        }
        ans *ans
    }

    fn largest_area(heights: &[i32]) -> i32 {
        let mut st = vec![-1];
        let mut ans = 0;
        for (r, &h) in heights.iter().enumerate() {
            let right = r as i32;
            while st.len() > 1 && heights[*st.last().unwrap() as usize] >= h {
                let i = st.pop().unwrap()  as usize;
                let left = *st.last().unwrap();
                ans = ans.max(heights[i].min(right - left  - 1));
            }
            st.push(right);
        }
        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_normal_case() {
        let matrix = vec![
            vec!['1', '0', '1', '0', '0'],
            vec!['1', '0', '1', '1', '1'],
            vec!['1', '1', '1', '1', '1'],
            vec!['1', '0', '0', '1', '0'],
        ];
        assert_eq!(Solution::maximal_square(matrix), 4);
    }

    #[test]
    fn test_bound_conditions() {
        let matrix = vec![vec!['0', '1'], vec!['1', '0']];
        assert_eq!(Solution::maximal_square(matrix), 1);

        let matrix = vec![vec!['0']];
        assert_eq!(Solution::maximal_square(matrix), 0);
    }
}
