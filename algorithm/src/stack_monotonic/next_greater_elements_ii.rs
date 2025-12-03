#[allow(dead_code)]
struct Solution;
#[allow(dead_code)]
impl Solution {

    pub fn next_greater_elements(nums: Vec<i32>) -> Vec<i32> {
        let n = nums.len();
        let mut ans = vec![-1; n];
        let mut st = vec![];
        for i in 0..2*n {
            let x = nums[i % n];
            while let Some(&top) = st.last() {
                if x > nums[top] {
                    ans[top] = x;
                    st.pop();
                } else {
                    break;
                }
            }
            if i < n {
                st.push(i);
            }
        }
       ans
    }
}
#[cfg(test)]
mod tests {
    use super::Solution;


    #[test]
    fn test_normal_case() {
        let nums = vec![1,2,1];
        assert_eq!(Solution::next_greater_elements(nums), vec![2, -1, 2]);

        let nums = vec![1,2,3,4,3];
        assert_eq!(Solution::next_greater_elements(nums), vec![2,3,4,-1,4]);
    }
}