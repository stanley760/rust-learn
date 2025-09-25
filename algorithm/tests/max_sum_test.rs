#[cfg(test)]
mod test {
    use algorithm::max_sum::Solution;

    #[test]
    pub fn test_instance1() {
        let nums = vec![2, 6, 7, 3, 1, 7];
        let m = 3;
        let k = 4;
        assert_eq!(Solution::max_sum(nums, m, k), 18);
    }

    #[test]
    pub fn test_instance2() {
        let nums = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        let m = 5;
        let k = 5;
        assert_eq!(Solution::max_sum(nums, m, k), 20);
    }

    #[test]
    pub fn test_instance3() {
        let nums = vec![1,2,1,2,1,2,1];
        let m = 3;
        let k = 3;
        assert_eq!(Solution::max_sum(nums, m, k), 0);
    }

    #[test]
    pub fn test_i32_value() {
        let i: usize = 2;
        let k : usize = 3;
        assert_eq!(i as i32 -k as i32 + 1, 0);
    }
}
