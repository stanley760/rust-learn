#[allow(dead_code)]
pub struct Solution;
#[allow(dead_code)]
impl Solution {
    pub fn longest_subarray(nums: Vec<i32>) -> i32 {
        let (mut ans, mut mx, mut cnt) = (0, 0, 0);
        for x in nums {
            
            if x > mx {
                // get the maximum's index. and count is 1.
                mx = x;
                ans = 1;
                cnt = 1;
            } else if x == mx {
                // the next maximum, calculate the answer.
                cnt += 1;
                ans = ans.max(cnt);
            } else {
                // clear the count because its out of the bound which the condition is 
                // calculating the subarray only contains the max value. 
                cnt = 0;
            }
        }
        dbg!(mx);
        ans
    }
}
#[cfg(test)]
mod tests {
    use crate::and_or::longest_subarray::Solution;

    #[test]
    pub fn test_normal_case() {
        let nums = vec![1, 2, 3, 3, 2, 2];
        assert_eq!(Solution::longest_subarray(nums), 2);

        let nums = vec![1, 2, 3, 4];
        assert_eq!(Solution::longest_subarray(nums), 1);
    }
}
