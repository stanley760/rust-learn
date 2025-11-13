struct Solution;

#[allow(unused)]
impl Solution {
    pub fn successful_pairs(spells: Vec<i32>, potions: Vec<i32>, success: i64) -> Vec<i32> {
        let mut potions = potions;
        potions.sort_unstable();
        let n = potions.len();

        spells
            .iter()
            .map(|&spell| {
                // note: ⌈a/b​⌉=⌊(a+b−1)/b​⌋
                let target = (success - 1) / spell as i64;
                if target >= potions[potions.len() - 1] as i64 {
                    0
                } else {
                    (n - Self::lower_bound(&potions, target as i32)) as i32
                }
            })
            .collect()
    }

    fn lower_bound(nums: &Vec<i32>, target: i32) -> usize {
        let (mut left, mut right) = (0, nums.len());
        while left < right {
            let mid = left + right >> 1;
            if nums[mid] <= target {
                left = mid + 1;
            } else {
                right = mid;
            }
        }
        left
    }

    pub fn successful_pairs2(spells: Vec<i32>, potions: Vec<i32>, success: i64) -> Vec<i32> {
        let mut spells = spells;
        let n = potions.len();
        for s in spells.iter_mut() {
            let success = (success as f64) / (*s as f64);
            let target = potions.partition_point(|&x| (x as f64) < success);

            *s = n as i32 - target as i32;
        }
        spells
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;
    #[test]
    fn test_successful_pairs() {
        let spells = vec![5, 1, 3];
        let potions = vec![1, 2, 3, 4, 5];
        let success = 7;

        assert_eq!(
            Solution::successful_pairs(spells.clone(), potions.clone(), success),
            vec![4, 0, 3]
        );
        assert_eq!(
            Solution::successful_pairs2(spells.clone(), potions.clone(), success),
            vec![4, 0, 3]
        );
    }

    #[test]
    fn test_successful_pairs_no_pairs() {
        let spells = vec![3, 1, 2];
        let potions = vec![8, 5, 8];
        let success = 16;
        assert_eq!(
            Solution::successful_pairs(spells.clone(), potions.clone(), success),
            vec![2, 0, 2]
        );
    }
}
