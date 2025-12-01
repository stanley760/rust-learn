#[allow(dead_code)]
struct Solution;
#[allow(dead_code)]
impl Solution {
    pub fn peak_index_in_mountain_array(arr: Vec<i32>) -> i32 {
        let (mut left, mut right) = (0, arr.len() - 1);
        while left < right {
            let mid = (left + right) >> 1;
            if arr[mid] < arr[mid + 1] {
                left = mid + 1;
            } else {
                right = mid;
            }
        }
        left as _
    }
}

#[cfg(test)]
mod tests {
    use super::*;


    #[test]
    fn test_peak_index_in_mountain_array() {
        let arr = vec![0, 1, 0];
        assert_eq!(Solution::peak_index_in_mountain_array(arr), 1);

        let arr = vec![0, 2, 1, 0];
        assert_eq!(Solution::peak_index_in_mountain_array(arr), 1);

        let arr = vec![0, 10, 5, 2];
        assert_eq!(Solution::peak_index_in_mountain_array(arr), 1);

        let arr = vec![3, 4, 5, 1];
        assert_eq!(Solution::peak_index_in_mountain_array(arr), 2);

        let arr = vec![24, 69, 100, 99, 79, 78, 67, 36, 26, 19];
        assert_eq!(Solution::peak_index_in_mountain_array(arr), 2);
    }
}