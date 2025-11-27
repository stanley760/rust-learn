#[allow(dead_code)]
struct Solution;
#[allow(dead_code)]
impl Solution {
    pub fn my_sqrt(x: i32) -> i32 {
        if x == 0 {
            return 0;
        }
        let s = x as f64;
        let mut dizzy = s; // 初始猜测值

        loop {
            let ans = (dizzy + s / dizzy) / 2.0;
            // 当变化足够小时停止（避免浮点误差导致无限循环）
            if (dizzy - ans).abs() < 1e-7 {
                return ans as i32;
            }
            dizzy = ans;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_my_sqrt() {
        assert_eq!(Solution::my_sqrt(4), 2);
        assert_eq!(Solution::my_sqrt(8), 2);
    }
}
