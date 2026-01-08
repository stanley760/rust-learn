#[allow(dead_code)]
struct Solution;
#[allow(dead_code)]
impl Solution {
    // 方法1：使用右移遍历每一位
    pub fn binary_gap(n: i32) -> i32 {
        let mut max_gap = 0;
        let mut last_one = -1; // 上一个1的位置
        let mut n = n;
        let mut pos = 0;

        while n > 0 {
            if n & 1 == 1 { // 检查最低位是否为1
                if last_one != -1 {
                    max_gap = max_gap.max(pos - last_one);
                }
                last_one = pos;
            }
            n >>= 1; // 右移一位
            pos += 1;
        }

        max_gap
    }

    // 方法2：使用 trailing_zeros 找到每个1的位置
    pub fn binary_gap_v2(n: i32) -> i32 {
        let mut max_gap = 0;
        let mut last_one: Option<i32> = None;
        let mut n = n;

        while n > 0 {
            // 找到最低位1的位置
            let zeros = n.trailing_zeros() as i32;
            if let Some(last) = last_one {
                max_gap = max_gap.max(zeros - last);
            }
            last_one = Some(zeros);
            // 清除最低位的1
            n &= n - 1;
        }

        max_gap
    }

    
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_binary_gap() {
        assert_eq!(Solution::binary_gap(22), 2);  // 10110
        assert_eq!(Solution::binary_gap(8), 0);   // 1000
        assert_eq!(Solution::binary_gap(5), 2);   // 101
        assert_eq!(Solution::binary_gap(6), 1);   // 110
        assert_eq!(Solution::binary_gap(1), 0);   // 1
    }

    #[test]
    fn test_binary_gap_v2() {
        assert_eq!(Solution::binary_gap_v2(22), 2);
        assert_eq!(Solution::binary_gap_v2(8), 0);
        assert_eq!(Solution::binary_gap_v2(5), 2);
    }
}