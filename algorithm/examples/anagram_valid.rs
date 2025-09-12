struct Solution;
struct Solution1;
impl Solution {
    pub fn is_anagram(s: String, t: String) -> bool {
        let mut a: Vec<_> = s.bytes().collect();
        let mut b: Vec<_> = t.bytes().collect();
        a.sort_unstable();
        b.sort_unstable();
        a == b
    }
}

impl Solution1 {
    pub fn is_anagram(s: String, t: String) -> bool {


        let mut cnt = [0; 26];
        for c in s.bytes() {
            cnt[(c - b'a') as usize] += 1;
        }
        for c in t.bytes() {
            cnt[(c - b'a') as usize] -= 1;
        }
        cnt.iter().all(|&x| x == 0)
    }
}

fn main() {

    let s = "anagram";
    let t = "nagaram";
    assert_eq!(Solution::is_anagram(s.to_owned(), t.to_owned()), true);

    let s = "rat";
    let  t = "car";
    assert_eq!(Solution1::is_anagram(s.to_owned(), t.to_owned()), false);
}