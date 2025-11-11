#[allow(dead_code)]
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn find_special_integer(arr: Vec<i32>) -> i32 {
        let n = arr.len();
        let threshold = n / 4;
        let idx = [threshold, 2 * threshold, 3 * threshold];
        for &i in &idx {
            let left = arr.partition_point(|&x| x < arr[i]);
            let right = arr.partition_point(|&x| x <= arr[i]);
            if right - left > threshold {
                return arr[i];
            }
        }
        arr[threshold * 3 + 2]
    }
}

// cargo test --package algorithm --lib -- binary::find_special_integer::tests --nocapture
#[cfg(test)]
mod tests {

    use super::Solution;
    #[test]
    fn test_find_special_integer() {
        let arr = vec![1, 2, 2, 6, 6, 6, 6, 7, 10];
        assert_eq!(Solution::find_special_integer(arr), 6);
        let arr = vec![1, 1];
        assert_eq!(Solution::find_special_integer(arr), 1);
        let arr = vec![1, 2, 2, 3, 2];
        assert_eq!(Solution::find_special_integer(arr), 2);
    }
}
