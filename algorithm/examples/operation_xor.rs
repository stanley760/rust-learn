struct Solution;

impl Solution {
    
    pub fn xor_operation(n: i32, start: i32) -> i32 {
        let mut nums = vec![0; n as usize];

        for i in 0..n {
            nums[i as usize] = start + 2*i;
        }

        let res = nums.iter().fold(0, |acc, &x| acc ^ x);
        res
    }
}

fn main() {
    assert_eq!(Solution::xor_operation(4, 3), 2);
}