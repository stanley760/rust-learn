#[allow(dead_code)]
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn flood_fill(mut image: Vec<Vec<i32>>, sr: i32, sc: i32, color: i32) -> Vec<Vec<i32>> {
        let sr = sr as usize;
        let sc = sc as usize;
        let original_color = image[sr][sc];

        if original_color != color {
            Self::dfs(&mut image, sr, sc, original_color, color);
        }

        image
    }

    fn dfs(image: &mut Vec<Vec<i32>>, i: usize, j: usize, original_color: i32, new_color: i32) {
        // checkout the value matched or not（avoid reprocessing）
        if image[i][j] != original_color {
            return;
        }

        // Coloring 
        image[i][j] = new_color;

        let rows = image.len();
        let cols = image[0].len();

        // Recursively visit the four adjacent cells, ensuring bounds are not exceeded
        if i > 0 {
            Self::dfs(image, i - 1, j, original_color, new_color);
        }
        if i + 1 < rows {
            Self::dfs(image, i + 1, j, original_color, new_color);
        }
        if j > 0 {
            Self::dfs(image, i, j - 1, original_color, new_color);
        }
        if j + 1 < cols {
            Self::dfs(image, i, j + 1, original_color, new_color);
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::grid::flood_fill::Solution;

    #[test]
    pub fn test_normal_case() {
        let image = vec![vec![1, 1, 1], vec![1, 1, 0], vec![1, 0, 1]];
        let sr = 1;
        let sc = 1;
        let color = 2;

        let expected = vec![vec![2, 2, 2], vec![2, 2, 0], vec![2, 0, 1]];
        assert_eq!(Solution::flood_fill(image, sr, sc, color), expected);
    }
}
