#[cfg(test)]
mod tests  {
    use algorithm::sorting::find_disappeared_numbers::Solution;

    #[test]
    pub fn test_find_disappeared_numbers() {
        let nums = vec![4,3,2,7,8,2,3,1];
        let res = Solution::find_disappeared_numbers(nums);
        assert_eq!(res, vec![5,6]);
    }

    #[test]
    pub fn test_find_disappeared_numbers_2() {
        let nums = vec![1,1];
        let res = Solution::find_disappeared_numbers(nums);
        assert_eq!(res, vec![2]);
    }
}