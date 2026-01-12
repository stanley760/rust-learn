#[allow(dead_code)]
struct Solution;
#[allow(dead_code)]
impl Solution {

    pub fn xor_queries(arr: Vec<i32>, queries: Vec<Vec<i32>>) -> Vec<i32> {
        let n = arr.len();
        let mut prefix = vec![0; n+1];

        for i in 0..n {
            prefix[i + 1] = prefix[i] ^ arr[i];
        }

        queries.iter().map(|x| prefix[x[0] as usize] ^ prefix[x[1] as usize + 1]).collect::<Vec<_>>()
    }
}
#[cfg(test)]
mod tests {
    use crate::xor::xor_queries::Solution;

    #[test]
    pub fn test_normal_case() {
        let arr = vec![1,3,4,8];
        let queries = vec![vec![0,1],vec![1,2],vec![0,3],vec![3,3]];
        let expected = vec![2,7,14,8];

        assert_eq!(Solution::xor_queries(arr, queries), expected);

        let arr= vec![4,8,2,10];
        let queries = vec![vec![2,3],vec![1,3],vec![0,0],vec![0,3]];
        let expected = vec![8,0,4,4];
        
        assert_eq!(Solution::xor_queries(arr, queries), expected);
    }
}