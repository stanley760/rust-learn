#[allow(dead_code)]
struct Solution;
#[allow(dead_code)]
impl Solution {
    pub fn count_bits(n: i32) -> Vec<i32> {
        // let mut ans =vec![];
        // for i in 0..n {
        //     ans.push(i.count_ones() as i32);
        // }
        // ans

        (0..n + 1).map(|x| x.count_ones() as i32).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;
    #[test]
    fn test_count_bits() {
        assert_eq!(Solution::count_bits(2), vec![0,1,1]);
        assert_eq!(Solution::count_bits(5), vec![0,1,1,2,1,2]);
    }
}