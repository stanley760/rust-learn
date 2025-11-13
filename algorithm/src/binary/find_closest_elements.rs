#[allow(dead_code)]
struct Solution;
#[allow(unused)]
impl Solution {
    pub fn find_closest_elements(arr: Vec<i32>, k: i32, x: i32) -> Vec<i32> {
        let n = arr.len();
        let k = k as usize;
        let (mut left, mut right) = (0, n - k);
        while left < right {
            let mid = left + (right - left) / 2;
            if x - arr[mid] > arr[mid + k] - x {
                left = mid + 1;
            } else {
                right = mid;
            }
        }
        arr[left..left + k].to_vec()
    }
}

// cargo test --package algorithm --lib -- binary::find_special_integer::tests --nocapture
#[cfg(test)]
mod tests {
    use super::Solution;
    #[test]
    fn test_find_special_integer() {
        let arr = vec![1, 2, 3, 4, 5];
        let k = 4;
        let x = 3;
        assert_eq!(Solution::find_closest_elements(arr, k, x), vec![1, 2, 3, 4]);
    }
}
