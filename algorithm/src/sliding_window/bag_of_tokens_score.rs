#[allow(dead_code)]
struct Solution;
#[allow(unused)]
impl Solution {
    pub fn bag_of_tokens_score(tokens: Vec<i32>, power: i32) -> i32 {
        if tokens.is_empty() {
            return 0;
        }
        let mut tokens = tokens;
        tokens.sort_unstable();
        let n = tokens.len();
        let mut power = power;
        let (mut left, mut right) = (0, n - 1);
        let (mut cur, mut res) = (0, 0);
        while left <= right {
            if  power >= tokens[left] {
                power -= tokens[left];
                cur += 1;
                left += 1;
            } else if power < tokens[left] && cur > 0 {
                res = res.max(cur);
                power += tokens[right];
                right -= 1;
                cur -= 1;
            } else {
                break;
            }
        }

        res.max(cur)
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_normal_case() {
        let tokens = vec![100];
        let power = 50;
        assert_eq!(Solution::bag_of_tokens_score(tokens, power), 0);
    }

    #[test]
    fn test_case_two() {
        let tokens = vec![200, 100];
        let power = 150;
        assert_eq!(Solution::bag_of_tokens_score(tokens, power), 1);
    }
    #[test]
    fn test_case_three() {
        let tokens = vec![100, 200, 300, 400];
        let power = 200;
        assert_eq!(Solution::bag_of_tokens_score(tokens, power), 2);
    }
}
