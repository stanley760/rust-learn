pub struct Solution;

impl Solution {
    pub fn check_palindrome_formation(a: String, b: String) -> bool {
         Self::check(a.clone(), b.clone()) ||  Self::check(b, a)
    }

    fn check(a: String, b: String) -> bool {
        let(mut left, mut right) = (0, a.len() - 1);
        while left < right && a.as_bytes()[left] == b.as_bytes()[right] {
            left += 1;
            right -= 1;
        }
        return Self::is_palindrome(&a, left, right) ||  Self::is_palindrome(&b, left, right);
    }

    fn is_palindrome(s: &String, mut left: usize, mut right: usize) -> bool {
        while left < right {
            if s.as_bytes()[left] != s.as_bytes()[right] {
                return false;
            }
            left += 1;
            right -= 1;
        }
        left >= right
    }   
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_check_palindrome_formation() {
        let a = "x".to_string();
        let b = "y".to_string();
        assert_eq!(Solution::check_palindrome_formation(a, b), true);

        let a = "abdef".to_string();
        let b = "fecab".to_string();
        assert_eq!(Solution::check_palindrome_formation(a, b), true);

        let a = "ulacfd".to_string();
        let b = "jizalu".to_string();
        assert_eq!(Solution::check_palindrome_formation(a, b), true);

        let a = "abc".to_string();
        let b = "def".to_string();
        assert_eq!(Solution::check_palindrome_formation(a, b), false);
    }
}