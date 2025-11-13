#[allow(dead_code)]
struct Solution;

#[allow(unused)]
impl Solution {
    pub fn sort_colors(nums: &mut [i32]) {
        let mut low = 0;
        let mut mid = 0;
        let mut high = nums.len() - 1;

        while mid <= high {
            match nums[mid] {
                0 => {
                    nums.swap(low, mid);
                    low += 1;
                    mid += 1;
                }
                1 => {
                    mid += 1;
                }
                2 => {
                    nums.swap(mid, high);
                    if high == 0 {
                        break;
                    }
                    high -= 1;
                }
                _ => {}
            }
        }
    }

    pub fn sort_colors_insert(nums: &mut [i32]) {
        let mut p0 = 0;
        let mut p1 = 0;
        // [2, 0, 2, 1, 1, 0]
        // [0, 2, 2, 1, 1, 0]
        // [0, 2, 2, 1, 1, 0]
        // [0, 1, 2, 2, 1, 0]
        // [0, 1, 1, 2, 2, 0]
        // [0, 0, 1, 1, 2, 2]
        for i in 0..nums.len() {
            let x = nums[i];
            nums[i] = 2;
            if x <= 1 {
                nums[p1] = 1;
                p1 += 1;
            }

            if x == 0 {
                nums[p0] = 0;
                p0 += 1;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;
    use std::time::Instant;

    #[test]
    fn test_sort_colors() {
        let mut nums = vec![2, 0, 2, 1, 1, 0];

        let start = Instant::now();
        Solution::sort_colors(&mut nums);
        let elapsed = start.elapsed();
        assert_eq!(nums, vec![0, 0, 1, 1, 2, 2]);

        println!(
            "Elapsed original: {:.3?} ({} ns)",
            elapsed,
            elapsed.as_nanos()
        );
    }

    #[test]
    fn test_sort_colors_insert() {
        let mut nums = vec![2, 0, 2, 1, 1, 0];
        let start = Instant::now();
        Solution::sort_colors_insert(&mut nums);
        let elapsed = start.elapsed();
        assert_eq!(nums, vec![0, 0, 1, 1, 2, 2]);
        println!(
            "Elapsed insert: {:.3?} ({} ns)",
            elapsed,
            elapsed.as_nanos()
        );
    }
}
