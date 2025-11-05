struct Solution;
impl Solution {
    pub fn vowel_strings(words: Vec<String>, left: i32, right: i32) -> i32 {
        let mut count = 0;
        for (i, elem) in words.iter().enumerate() {
            if i >= left as usize && i <= right as usize {
                if (elem.starts_with('a') || elem.starts_with('e')
                    || elem.starts_with('i') || elem.starts_with('o')
                    || elem.starts_with('u')) && (elem.ends_with('a')
                    || elem.ends_with('e') || elem.ends_with('i')
                    || elem.ends_with('o') || elem.ends_with('u')) {

                    count += 1;
                }
            }
        }
        count
    }
}

fn main() {
    let words = vec![String::from("are"), String::from("amy"), String::from("u")];
    let left = 0;
    let right = 2;
    println!("{:?}", Solution::vowel_strings(words, left, right));
}