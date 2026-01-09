use std::collections::HashSet;

#[allow(dead_code)]
struct Solution;
#[allow(dead_code)]
impl Solution {

    ///$$
    /// Time's Complexity:  O(n)
    /// Space's Complexity: O(1)
    ///$$
    pub fn find_final_value(nums: Vec<i32>, original: i32) -> i32 {
        let mut mask = 0;
        for x in nums {
            let k = x / original;
            // the issues of 2^n
            if x % original == 0 && (k & (k - 1)) == 0 {
                println!("k:{:b}", k);
                mask |= k;
            }
        }
        println!("mask:{:b}", mask);
        mask = !mask;
        original *(mask & -mask)
    }

    ///$$
    /// Time's Complexity:  O(n+log(original / U))
    /// Space's Complexity: O(log(original / U))
    ///$$
    pub fn find_final_value_v1(nums: Vec<i32>, original: i32) -> i32 {
        let mut original = original;
        let st = nums.into_iter()
        .filter(|x| x % original == 0 && ((x /original) & (x / original - 1)) == 0)
        .collect::<HashSet<_>>();
        
        while st.contains(&original) {
            original *= 2;
        }
        original
    }
}

#[cfg(test)]
mod tests {
    use crate::find_final_value::Solution;

    #[test]
    pub fn test_normal_case() {
        let nums = vec![5,3,6,1,12];
        let original = 3;
        assert_eq!(Solution::find_final_value(nums, original), 24);

        // let nums = vec![2,7,9];
        // let original = 4;
        // assert_eq!(Solution::find_final_value(nums, original), 4);
    }

    #[test]
    pub fn test_normal_case_v1() {
let nums = vec![5,3,6,1,12];
        let original = 3;
        assert_eq!(Solution::find_final_value_v1(nums, original), 24);

        let nums = vec![2,7,9];
        let original = 4;
        assert_eq!(Solution::find_final_value_v1(nums, original), 4);
    }
}