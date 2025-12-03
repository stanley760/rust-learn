use std::collections::HashMap;

#[allow(dead_code)]
struct Solution;
#[allow(dead_code)]
impl Solution {

    pub fn  next_greater_element(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
        
        let index = nums1.iter().enumerate().map(|(i, &x)| (x, i)).collect::<HashMap<_, _>>();
        let n = nums1.len();
        let mut res = vec![-1; n];
        let mut stack= Vec::new();
        for x in nums2 {
            while let Some(top) = stack.last() {
                if x <= *top {
                    break;
                } else {
                    res[index[top]] = x;
                    stack.pop();
                } 
            }
            stack.push(x);
        }
        
        res
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_next_greater_element() {
        let nums1 = vec![4, 1, 2];
        let nums2 = vec![1, 3, 4, 2];
        assert_eq!(Solution::next_greater_element(nums1, nums2), vec![-1, 3, -1]);
    }

    #[test]
    fn test_next_greater_element_2() {
        let nums1 = vec![2, 4];
        let nums2 = vec![1, 2, 3, 4];
        assert_eq!(Solution::next_greater_element(nums1, nums2), vec![3, -1]);
    }
}