use algorithm::sliding_window::max_satisfied::Solution;
#[test]
pub fn test_max_satisfied() {
    let satisfaction = vec![1, 0, 1, 2, 1, 1, 7, 5];
    let hours = vec![0, 1, 0, 1, 0, 1, 0, 1];
    assert_eq!(Solution::max_satisfied(satisfaction, hours, 3), 16);
}

#[test]
pub fn test_max_satisfied_2() {
    let satisfaction = vec![1];
    let hours = vec![0];
    assert_eq!(Solution::max_satisfied(satisfaction, hours, 1), 1);
}
