#[cfg(test)]
mod tests {
    use algorithm::share_candies::Solution;

    #[test]
    fn test_share_candies() {
        assert_eq!(Solution::share_candies(vec![1,2,2,3,4,3], 3), 3);
        assert_eq!(Solution::share_candies(vec![2,2,2,2,3,3], 2), 2);
        assert_eq!(Solution::share_candies(vec![2,4,5], 0), 3);
    }
}
