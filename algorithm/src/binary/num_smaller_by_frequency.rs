#[allow(dead_code)]
struct Solution;

#[allow(unused)]
impl Solution {
    pub fn num_smaller_by_frequency(queries: Vec<String>, words: Vec<String>) -> Vec<i32> {
        fn f(s: &str) -> i32 {
            let first_char = s.chars().min().unwrap();
            let count = s.chars().filter(|&c| c == first_char).count() as i32;
            count
        }

        let mut word_freqs: Vec<i32> = words.iter().map(|w| f(w)).collect();
        word_freqs.sort_unstable();

        queries.iter().map(|q| {
            let idx = word_freqs.partition_point(|&wf| wf <= f(q));
            (word_freqs.len() - idx) as i32
        }).collect()
    }
}

#[cfg(test)]
mod tests {

    use super::Solution;

    #[test]
    fn test_num_smaller_by_frequency() {
        let queries = vec!["cbd".to_string()];
        let words = vec!["zaaaz".to_string()];
        assert_eq!(
            Solution::num_smaller_by_frequency(queries, words),
            vec![1]
        );

        let queries = vec!["bbb".to_string(), "cc".to_string()];
        let words = vec!["a".to_string(), "aa".to_string(), "aaa".to_string(), "aaaa".to_string()];
        assert_eq!(
            Solution::num_smaller_by_frequency(queries, words),
            vec![1, 2]
        );
    }
}
