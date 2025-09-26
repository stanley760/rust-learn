
#[cfg(test)]
mod tests {
    use algorithm::sliding_window::maximum_subarray_sum::Solution;

    #[test]
    pub fn test_maximum_subarray_sum() {
        let nums = vec![1,5,4,2,9,9,9];
        let k = 3;

        assert_eq!(Solution::maximum_subarray_sum(nums, k), 15);
    }
}