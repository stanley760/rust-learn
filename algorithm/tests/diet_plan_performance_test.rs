use algorithm::sliding_window::diet_plan_performance::Solution;
#[test]
fn test() {
    assert_eq!(
        Solution::diet_plan_performance(vec![1, 2, 3, 4, 5], 1, 3, 3),
        0
    );
    assert_eq!(
        Solution::diet_plan_performance(vec![3, 2], 2, 0, 1),
        1
    );
}