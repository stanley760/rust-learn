#[allow(dead_code)]
struct Solution;
#[allow(dead_code)]
impl Solution {
    pub fn min_eating_speed(piles: Vec<i32>, h: i32) -> i32 {
        
        let mut left = 1;
        let mut right = *piles.iter().max().unwrap();
        while left < right {
            let mid = (left + right) >> 1;
            // -1 is used to apply the ceiling function to the result.
            let total = piles.iter().map(|&p|(p + mid - 1) / mid).sum::<i32>();
            if total > h {
                left = mid + 1;
            } else {
                right = mid;
            }
        }

        left 
    }
}

#[cfg(test)]
mod tests {
    use crate::binary::min_eating_speed::Solution;


    #[test]
    fn test_normal_case() {
        let piles = vec![3,6,7,11]; let h = 8;
        assert_eq!(Solution::min_eating_speed(piles, h), 4);

        let piles = vec![30,11,23,4,20];
        let h = 5;
        assert_eq!(Solution::min_eating_speed(piles, h), 30);

        let piles = vec![30,11,23,4,20]; let h = 6;
        assert_eq!(Solution::min_eating_speed(piles, h), 23);
    }
}