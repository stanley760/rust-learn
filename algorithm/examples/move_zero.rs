pub struct Solution;

impl Solution {
    pub fn move_zeroes(nums: &mut Vec<i32>) {
        let mut temp = 0;
        for index in 0..nums.len() {
            if nums[index] != 0 {
                nums.swap(index, temp);
                temp += 1;
            }
        }
    }
}

fn main() {
    let mut nums = vec![1, 2, 0, 3, 0];
    Solution::move_zeroes(&mut nums);
    println!("{:?}", nums);

    let mut nums = vec![0];
    Solution::move_zeroes(&mut nums);
    println!("{:?}", nums);
}
