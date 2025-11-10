#[allow(dead_code)]
struct Solution;

#[allow(unused)]
impl Solution {
    pub fn find_the_distance_value(arr1: Vec<i32>, arr2: Vec<i32>, d: i32) -> i32 {
        let mut arr2 = arr2;
        arr2.sort_unstable();
        
        arr1.into_iter().filter(|&num|{
            let i = arr2.partition_point(|&x| x < num - d);
            i == arr2.len() || arr2[i] > num + d
        }).count() as i32

    }
}

// cargo test --package algorithm --lib -- binary::find_the_distance_value::tests --nocapture
#[cfg(test)]
mod tests {
    use super::Solution;   
    #[test]
    fn test_find_the_distance_value() {
        let arr1 = vec![4,5,8];
        let arr2 = vec![10,9,1,8];
        let d = 2;
        assert_eq!(Solution::find_the_distance_value(arr1.clone(), arr2.clone(), d), 2);
        let arr1 = vec![1,4,2,3];
        let arr2 = vec![-4,-3,6,10,20,30];
        let d = 3;
        assert_eq!(Solution::find_the_distance_value(arr1.clone(), arr2.clone(), d), 2);
        let arr1 = vec![2,1,100,3];
        let arr2 = vec![-5,-2,10,-3,7];
        let d = 6;
        assert_eq!(Solution::find_the_distance_value(arr1.clone(), arr2.clone(), d), 1);
    }

}