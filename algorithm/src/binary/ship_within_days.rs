#[allow(dead_code)]
struct Solution;
#[allow(dead_code)]
impl Solution {
    pub fn ship_within_days(weights: Vec<i32>, days: i32) -> i32 {
        let mut left = *weights.iter().max().unwrap();
        let mut right: i32 =  weights.iter().sum();
        while left < right {
            let mid = (left + right) >> 1;
            // let mut count_days = 1;
            // let mut current = 0;
            // for &weight in weights.iter() {
            //     if current + weight > mid {
            //         count_days += 1;
            //         current = weight;
            //     } else {
            //         current += weight;
            //     }
            // }
            let count_days =  weights.iter().fold((1, 0), |(days, current), &weight| {
                if current + weight > mid {
                    (days + 1, weight)
                } else {
                    (days, current + weight)
                }
            }).0;
            if count_days > days {
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
    use super::Solution;


    #[test]
    fn test_normal_case() {
        let weights = vec![1,2,3,4,5,6,7,8,9,10];
        let days = 5;
        assert_eq!(Solution::ship_within_days(weights, days), 15);

        let weights = vec![3,2,2,4,1,4];
        let days = 3;
        assert_eq!(Solution::ship_within_days(weights, days), 6);

        let weights = vec![1,2,3,1,1];
        let days = 4;
        assert_eq!(Solution::ship_within_days(weights, days), 3);
    }
}