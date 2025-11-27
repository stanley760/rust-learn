#[allow(dead_code)]
struct Solution;
#[allow(dead_code)]
impl Solution {
    pub fn split_array(nums: Vec<i32>, k: i32) -> i32 {
        let mut left = *nums.iter().max().unwrap();
        let mut right = nums.iter().sum::<i32>();
        let check = |max| -> bool {
            let mut cnt = 1;
            let mut s = 0;
            for &n in &nums {
                if s + n <= max {
                    s += n;
                    continue;
                }
                if cnt == k {
                    return false;
                }
                cnt += 1;
                s = n;
            }
            true
        };
        
       
        while left < right {
            let mid = (left + right) >> 1;
            if check(mid) {
                right = mid;
            } else {
                left = mid + 1;
            }
        }
        right
    }
}

#[cfg(test)]
mod tests {
    use crate::binary::split_array::Solution;

    #[test]
    fn test_case1() {
        let nums = vec![7, 2, 5, 10, 8];
        let k = 2;

        assert_eq!(Solution::split_array(nums, k), 18);
    }
}
