#[allow(dead_code)]
struct Solution;

#[allow(unused)]
impl Solution {
    pub fn find_the_distance_value(arr1: Vec<i32>, arr2: Vec<i32>, d: i32) -> i32 {
        let mut arr2 = arr2;
        arr2.sort_unstable();

        arr1.into_iter()
            .filter(|&num| {
                let i = arr2.partition_point(|&x| x < num - d);
                i == arr2.len() || arr2[i] > num + d
            })
            .count() as i32
    }

    fn find_the_distance_value2(arr1: Vec<i32>, arr2: Vec<i32>, d: i32) -> i32 {
        let mut arr2 = arr2;
        arr2.sort_unstable();

        let cnt = |e: i32| -> bool {
            let index = arr2.partition_point(|&x| x < e - d);
            index == arr2.len() || arr2[index] > e + d
        };

        arr1.into_iter().filter(|&a| cnt(a)).count() as i32
    }
}

// cargo test --package algorithm --lib -- binary::find_the_distance_value::tests --nocapture
#[cfg(test)]
mod tests {
    use super::Solution;
    #[test]
    fn test_find_the_distance_value() {
        let arr1 = vec![4, 5, 8];
        let arr2 = vec![10, 9, 1, 8];
        let d = 2;
        assert_eq!(
            Solution::find_the_distance_value(arr1.clone(), arr2.clone(), d),
            2
        );
        assert_eq!(
            Solution::find_the_distance_value2(arr1.clone(), arr2.clone(), d),
            2
        );
        let arr1 = vec![1, 4, 2, 3];
        let arr2 = vec![-4, -3, 6, 10, 20, 30];
        let d = 3;
        assert_eq!(
            Solution::find_the_distance_value(arr1.clone(), arr2.clone(), d),
            2
        );
        assert_eq!(
            Solution::find_the_distance_value2(arr1.clone(), arr2.clone(), d),
            2
        );
        let arr1 = vec![2, 1, 100, 3];
        let arr2 = vec![-5, -2, 10, -3, 7];
        let d = 6;
        assert_eq!(
            Solution::find_the_distance_value(arr1.clone(), arr2.clone(), d),
            1
        );
        assert_eq!(
            Solution::find_the_distance_value2(arr1.clone(), arr2.clone(), d),
            1
        );
    }

    #[test]
    fn test_partition_point() {
        let arr = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
        let target = 5;
        let i = arr.partition_point(|&x| x < target);
        assert_eq!(i, 4);
    }
}
