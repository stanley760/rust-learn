#[allow(dead_code)]
struct Solution;
#[allow(unused)]
impl Solution {
    pub fn answer_queries(nums: Vec<i32>, queries: Vec<i32>) -> Vec<i32> {
        let mut nums = nums;
        nums.sort_unstable();

        nums.iter_mut().fold(0, |s, x| {
            *x += s;
            *x
        });

        // queries.iter_mut().for_each(|x|*x=nums.partition_point(|y|y<=x) as i32);
        // return queries;
        let n = queries.len();
        let mut res = vec![0; n];

        for (i, &q) in queries.iter().enumerate() {
            let idx: usize = nums.partition_point(|&x| x <= q);
            res[i] = idx as i32;
        }

        res
    }
}

// cargo test --package algorithm --lib -- binary::answer_queries::tests --nocapture
#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_answer_queries() {
        let nums = vec![4, 5, 2, 1];
        let queries = vec![3, 10, 21];
        assert_eq!(
            Solution::answer_queries(nums.clone(), queries.clone()),
            vec![2, 3, 4]
        );
        let nums = vec![2, 3, 4, 5];
        let queries = vec![1];
        assert_eq!(
            Solution::answer_queries(nums.clone(), queries.clone()),
            vec![0]
        );
    }
}
