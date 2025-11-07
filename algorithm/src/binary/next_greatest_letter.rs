#[allow(dead_code)]
struct Solution;

#[allow(unused)]
impl Solution {
    pub  fn next_greatest_letter(letters: Vec<char>, target: char) -> char {
        let (mut left, mut right) = (0, letters.len() - 1);
        while left < right {
            let mid = left + right >> 1;
            if letters[mid] <= target {
                left = mid + 1;
            } else {
                right = mid;
            }
        }
        if letters[left] > target {
            letters[left]
        } else {
            letters[0]
        }
    }
}
// cargo test --package algorithm --lib -- binary::search::tests --nocapture
mod tests {
    use super::Solution;
    #[test]
    fn test_search_insert() {
        let letters = vec!['c', 'f', 'j'];
        let target = 'a';
        assert_eq!(Solution::next_greatest_letter(letters.clone(), target), 'c');

        let letters = vec!['c', 'f', 'j'];
        let target = 'c';
        assert_eq!(Solution::next_greatest_letter(letters.clone(), target), 'f');

        let letters = vec!['x', 'x', 'y', 'y'];
        let target = 'z';
        assert_eq!(Solution::next_greatest_letter(letters.clone(), target), 'x');

    }
}