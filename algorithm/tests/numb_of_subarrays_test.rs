use algorithm::sliding_window::num_of_subarrays::Solution;

#[test]
pub fn test_number_of_subarrays() {
    let arr = vec![1, 3, 5];
    let k = 1;
    let threshold = 2;
    println!("{:?}", Solution::num_of_subarrays(arr, k, threshold));
}