#[allow(dead_code)]
struct Solution; 
#[allow(dead_code)]
impl Solution {
   

    pub fn total_hamming_distance(nums: Vec<i32>) -> i32 {
        let mut ans = 0;
        let n = nums.len();
        for i in 0..32 {
            let mut ones = 0;
            for j in 0..n {
                // count how many 1s at bit i
                ones += (nums[j] >> i) & 1;
            }
            ans += ones * (n as i32 - ones);
        }
        ans
    }
}

#[cfg(test)]
mod tests {

    
    use super::*;

    #[test]
    fn test_total_hamming_distance() {
        assert_eq!(Solution::total_hamming_distance(vec![4, 14, 4]), 4);
    }

    #[test]
    fn tesst_case_2() {
        assert_eq!(Solution::total_hamming_distance(vec![4, 14, 2]), 6);
    }
}