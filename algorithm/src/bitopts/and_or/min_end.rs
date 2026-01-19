#[allow(dead_code)]
struct Solution;
#[allow(dead_code)]
impl Solution {
    pub fn min_end(n: i32, x: i32) -> i64 {
        let mut n = n;
        // n -1
        n -= 1;
        let mut ans = x as i64;
        let (mut i, mut j) = (0, 0);
        while (n >> j) > 0 {
            // the i-th bit of x is zero .
            if (ans >> i & 1) == 0 {
                // n >> j & 1  is filled with the j-th bit of n
                ans |= ((n >> j & 1) << i) as i64;
                j += 1;
            }
            i += 1;
        }
        ans
    }

    pub fn min_end_v2(n: i32, x: i32) -> i64 {
        let mut n = n;
        // n -1
        n -= 1;
        let mut j = 0;
        let mut ans = x as i64;
        let mut t = !ans;
        while (n >> j) > 0 {
            let lb = t & -t;
            ans |= (n >> j & 1) as i64 * lb;
            j += 1;
            t ^= lb;
        }
        ans
    }

    /// 基于《Hacker's Delight》第 7.5 节 "sheep and goats" 操作的优化版本
    /// 使用类似 PDEP (Parallel Deposit) 的思想
    pub fn min_end_optimized(n: i32, x: i32) -> i64 {
        let mut n = n - 1; // 减去 1，因为第一个数就是 x
        let mut ans = x as i64;

        // 逐位处理，找到 x 中为 0 的位置并填入 n 的对应位
        let mut bit_pos = 0;
        while n > 0 {
            // 找到下一个 x 中为 0 的位置
            while bit_pos < 64 && ((ans >> bit_pos) & 1) != 0 {
                bit_pos += 1;
            }

            // 如果 n 的最低位是 1，则在找到的位置设置 1
            if (n & 1) != 0 {
                ans |= 1i64 << bit_pos;
            }

            bit_pos += 1;
            n >>= 1;
        }

        ans
    }

    /// 真正的 PDEP 风格实现（更接近硬件指令）
    pub fn min_end_pdep_style(n: i32, x: i32) -> i64 {
        let n = n - 1; // 减去 1，因为第一个数就是 x
        let mut ans = x as i64;

        // 创建 mask：x 中所有为 0 的位
        let mask = !x as i64;
        dbg!(mask);
        // 模拟 PDEP 指令：将 n 的位分散到 mask 指定的位置
        let mut src = n as i64;
        let mut m = mask;

        while src != 0 && m != 0 {
            let lowest_bit = m & -m; 
            dbg!(lowest_bit);
            // 找到最低位的 1 0011
            if (src & 1) != 0 {
                ans |= lowest_bit;
                dbg!(ans);
            }
            src >>= 1;
            m ^= lowest_bit;
        }

        ans
    }
}
#[cfg(test)]
mod tests {
    use crate::and_or::min_end::Solution;

    #[test]
    pub fn test_normal_case() {
        let n = 3;
        let x = 4;
        assert_eq!(Solution::min_end(n, x), 6);

        let n = 2;
        let x = 7;
        assert_eq!(Solution::min_end(n, x), 15);
    }

    #[test]
    pub fn test_case_v2() {
        let n = 3;
        let x = 4;
        assert_eq!(Solution::min_end_v2(n, x), 6);

        let n = 2;
        let x = 7;
        assert_eq!(Solution::min_end_v2(n, x), 15);
    }

    #[test]
    pub fn test_optimized_version() {
        let n = 3;
        let x = 4;
        assert_eq!(Solution::min_end_optimized(n, x), 6);

        let n = 2;
        let x = 7;
        assert_eq!(Solution::min_end_optimized(n, x), 15);

        // 额外测试用例
        let n = 4;
        let x = 1;
        assert_eq!(Solution::min_end_optimized(n, x), 7);
    }

    #[test]
    pub fn test_pdep_style() {
        let n = 3;
        let x = 4;
        assert_eq!(Solution::min_end_pdep_style(n, x), 6);

        let n = 2;
        let x = 7;
        assert_eq!(Solution::min_end_pdep_style(n, x), 15);

        // 额外测试用例
        let n = 4;
        let x = 1;
        assert_eq!(Solution::min_end_pdep_style(n, x), 7);
    }
}
