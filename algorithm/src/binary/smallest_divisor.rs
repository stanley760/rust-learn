#[allow(dead_code)]
struct Solution;
#[allow(dead_code)]
impl Solution {
    pub fn smallest_divisor(nums: Vec<i32>, threshold: i32) -> i32 {
        let (mut left, mut right) = (1, *nums.iter().max().unwrap());

        while left < right {
            let mid = left + (right - left) / 2;
            let sum: i32 = nums.iter().map(|&num| (num + mid - 1) / mid).sum();

            if sum > threshold {
                left = mid + 1;
            } else {
                right = mid;
            }
        }

        left
    }
}

#[cfg(test)]
mod tests {

    use super::Solution;

    #[test]
    fn test_smallest_divisor() {
        let nums = vec![1, 2, 5, 9];
        let threshold = 6;
        assert_eq!(Solution::smallest_divisor(nums, threshold), 5);

        let nums = vec![2, 3, 5, 7, 11];
        let threshold = 11;
        assert_eq!(Solution::smallest_divisor(nums, threshold), 3);

        let nums = vec![19];
        let threshold = 5;
        assert_eq!(Solution::smallest_divisor(nums, threshold), 4);
    }
}

