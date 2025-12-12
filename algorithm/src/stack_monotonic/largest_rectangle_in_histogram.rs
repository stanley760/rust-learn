struct Solution;

impl Solution {
    pub fn largest_rectangle_area(heights: Vec<i32>) -> i32 {
        unimplemented!()
    }
}


#[cfg(test)]
mod tests {
    use crate::stack_monotonic::largest_rectangle_in_histogram::Solution;


    #[test]
    pub fn test_normal_case() {
        let heights = vec![2,1,5,6,2,3];
        assert_eq!(Solution::largest_rectangle_area(heights), 10);
    }
}