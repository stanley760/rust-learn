#[allow(dead_code)]
struct Solution;
#[allow(dead_code)]
impl Solution {
    pub fn my_sqrt(x: i32) -> i32 {
        if x == 0 {
            return 0;
        }
        let s =x as f64;
        let mut curr = s;
        // 牛顿迭代：next = (curr + s / curr) / 2
        loop {
            let next = (curr + s / curr) / 2.0;
            if (next - curr).abs() < 1e-12 {
                curr = next;
                break;
            }
            curr = next;
        }
        let mut res = curr as i64;
        let x64 = x as i64;
        while res * res > x64 { res -= 1; }
        while (res + 1) * (res + 1) <= x64 { res += 1; }
        res as i32
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