#[allow(dead_code)]
struct Solution;
#[allow(dead_code)]
impl Solution {
    pub fn find_the_prefix_common_array(a: Vec<i32>, b: Vec<i32>) -> Vec<i32> {
        
        let mut ans = vec![];
        let mut mask_a = 0i64;
        let mut mask_b = 0i64;
    
        for (&x, &y) in a.iter().zip(b.iter()) {
            mask_a |= 1i64 << x;
            mask_b |= 1i64 << y;
            ans.push((mask_a & mask_b).count_ones() as i32);
        }
        ans
    }
}
#[cfg(test)]
mod tests {
    use crate::find_the_prefix_common_array::Solution;

    #[test]
    pub fn test_normal_case() {
        let a = vec![1,3,2,4];
        let b = vec![3,1,2,4];
        let c = vec![0,2,3, 4];
        assert_eq!(Solution::find_the_prefix_common_array(a, b), c);

        let a = vec![2,3,1];
        let b = vec![3,1,2];
        let c = vec![0,1,3];
        assert_eq!(Solution::find_the_prefix_common_array(a, b), c);
    }
}