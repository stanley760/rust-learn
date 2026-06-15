
pub struct Solution;

impl Solution {
    pub fn my_sqrt(x: i32) -> i32 {
        let mut l = 0; 
        let mut r = x;
        let mut ans = -1;
        while l <= r {
            let mid = (r - l) / 2 + l;
            if mid * mid <= x {
                ans = mid;
                l = mid + 1;
            } else {
                r = mid - 1;
            }
        }
        return ans;
    }
    pub fn my_sqrt_by_newton_iter(x: i32) -> i32 {
        if x == 0 {
            return x;
        }
        let c: f32 = x as f32;
        let mut x0: f32 = x as f32;
        loop {
            let x1 = 0.5*(x0 + c/x0);
            if (x0 - x1).abs() < 1e-7 {
                break;
            }
            x0 = x1;
        }
        return x0 as i32;
    }
}


#[cfg(test)]
mod tests {
    use crate::math::sqrt_val::Solution;


    #[test]
    fn test_common() {
        let x = 4;
        let result = Solution::my_sqrt(x);
        assert_eq!(2, result);
        let x1 = 8;
        assert_eq!(2, Solution::my_sqrt(x1));
    }
}