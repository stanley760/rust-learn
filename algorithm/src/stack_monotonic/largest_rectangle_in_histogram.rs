#[allow(dead_code)]
struct Solution;
#[allow(dead_code)]
impl Solution {
    pub fn largest_rectangle_area(heights: Vec<i32>) -> i32 {
        let mut heights = heights;
        heights.push(-1); // sentinel.
        let mut st = vec![-1];
        let mut ans = 0;
        for (r, &h) in heights.iter().enumerate() {
            let right = r as i32;
            while st.len() > 1 && heights[*st.last().unwrap() as usize] >= h {
                let i = st.pop().unwrap() as usize;
                let left = *st.last().unwrap();
                ans = ans.max(heights[i] * (right - left - 1));
            }
            st.push(right);
        }
        ans
    }
}


#[cfg(test)]
mod tests {
    use super::Solution;


    #[test]
    pub fn test_normal_case() {
        let heights = vec![2,1,5,6,2,3];
        assert_eq!(Solution::largest_rectangle_area(heights), 10);
        let heights = vec![2, 4];
        assert_eq!(Solution::largest_rectangle_area(heights), 4);
    }
}