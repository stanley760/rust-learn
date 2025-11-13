#[cfg(test)]
mod tests {
    use algorithm::minimum_recolors::Solution;

    #[test]
    pub fn test_min_recolors() {
        let block: String = String::from("WBWBBBW");
        let k = 2;
        let res = Solution::minimum_recolors(block, k);
        assert_eq!(res, 0);
    }

    #[test]
    pub fn test_min_recolors_instance() {
        let block = String::from("WBBWWBBWBW");
        let k = 7;
        let res = Solution::minimum_recolors(block, k);
        assert_eq!(res, 3);
    }
}
