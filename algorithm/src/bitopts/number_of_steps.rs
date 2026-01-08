#[allow(dead_code)]
struct Solution;
#[allow(dead_code)]
impl Solution {
    pub fn number_of_steps(num: i32) -> i32 {
        let mut num = num;
        let mut cnt = 0;
        while num != 0 {
            cnt += 1;
            num = if num & 1 == 0 {
                num >> 1
            } else {
                num - 1
            };
        }
        cnt
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;
    #[test]
    fn test_number_of_steps() {
        assert_eq!(Solution::number_of_steps(14), 6);
        assert_eq!(Solution::number_of_steps(8), 4);
        assert_eq!(Solution::number_of_steps(123), 12);
    }
}