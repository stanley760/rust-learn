#[allow(dead_code)]
struct Solution;

#[allow(unused)]
impl Solution {
    pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
        let (mut left, mut right) = (0, nums.len());
        while left < right {
            let mid = left + (right - left) / 2;
            if nums[mid] < target {
                left = mid + 1;
            } else {
                right = mid;
            }
        }
        left as i32
    }
}
// cargo test --package algorithm --lib -- binary::search_insert::tests --nocapture
mod tests {
    use super::Solution;

    #[test]
    fn test_search_insert() {
        let nums = vec![1, 3, 5, 6];
        let target = 5;
        assert_eq!(Solution::search_insert(nums.clone(), target), 2);

        let target = 2;
        assert_eq!(Solution::search_insert(nums.clone(), target), 1);

        let target = 7;
        assert_eq!(Solution::search_insert(nums.clone(), target), 4);

        let target = 0;
        assert_eq!(Solution::search_insert(nums.clone(), target), 0);
    }
}