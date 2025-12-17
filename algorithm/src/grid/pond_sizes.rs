#[allow(dead_code)]
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn pond_sizes(land: Vec<Vec<i32>>) -> Vec<i32> {
        let mut land = land;
        fn dfs(land: &mut Vec<Vec<i32>>, i: usize, j: usize) -> i32 {
            let mut total = 1;
            if i >= land.len() || j >= land[0].len() || land[i][j] != 0 {
                return 0;
            }
            land[i][j] = 1;
            total += dfs(land, i, j + 1);
            total += dfs(land, i, j.wrapping_sub(1));
            total += dfs(land, i + 1, j);
            total += dfs(land, i.wrapping_sub(1), j);

            total += dfs(land, i + 1, j + 1); // lower right.
            total += dfs(land, i.wrapping_sub(1), j.wrapping_sub(1)); // upper left
            total += dfs(land, i + 1, j.wrapping_sub(1)); // upper right.
            total += dfs(land, i.wrapping_sub(1), j + 1); // lower left.

            total
        }
        let m = land.len();
        let n = land[0].len();
        let mut ans = Vec::new();
        for i in 0..m {
            for j in 0..n {
                if land[i][j] == 0 {
                    ans.push(dfs(&mut land, i, j));
                }
            }
        }
        ans.sort_unstable();
        ans
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    pub fn test_normal_case() {
        let land = [[0, 2, 1, 0], [0, 1, 0, 1], [1, 1, 0, 1], [0, 1, 0, 1]];
        let land = land.iter().map(|f| f.to_vec()).collect::<Vec<_>>();
        let expected = vec![1, 2, 4];
        assert_eq!(Solution::pond_sizes(land), expected);
    }
}
