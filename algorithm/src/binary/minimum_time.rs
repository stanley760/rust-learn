#[allow(dead_code)]
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn minimum_time(time: Vec<i32>, total_trips: i32) -> i32 {
        let min_time = *time.iter().min().unwrap() as i64;
        let mut left = min_time;
        let mut right = min_time * total_trips as i64;

        while left < right {
            let mid = left + (right - left) / 2;
            let trips: i64 = time.iter().map(|&t| mid / t as i64).sum();

            if trips < total_trips as i64 {
                left = mid + 1;
            } else {
                right = mid;
            }
        }
        right as i32
    }
}

#[cfg(test)]
mod tests {

    use super::Solution;

    #[test]
    fn test_minimum_time() {
        let time = vec![1, 2, 3];
        let total_trips = 5;
        assert_eq!(Solution::minimum_time(time, total_trips), 3);

        let time = vec![2];
        let total_trips = 1;
        assert_eq!(Solution::minimum_time(time, total_trips), 2);

        let time = vec![5, 10, 10];
        let total_trips = 9;
        assert_eq!(Solution::minimum_time(time, total_trips), 25);
    }
}
