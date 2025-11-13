struct Solution;

impl Solution {
    pub fn max_score(s: String) -> i32 {
        let mut right_ones = s.bytes().filter(|&c| c == b'1').count() as i32;

        let mut res = 0;

        let mut left_zeros = 0;

        let arr = s.as_bytes();
        // 遍历除最后一个字符外的所有字符（因为右子字符串不能为空）
        for i in 0..arr.len() - 1 {
            if arr[i] == b'0' {
                // 遇到'0'：左子字符串中0的个数增加
                left_zeros += 1;
            } else {
                // 遇到'1'：右子字符串中1的个数减少
                right_ones -= 1;
            }
            // 更新最大得分：左子字符串中0的个数 + 右子字符串中1的个数
            res = res.max(left_zeros + right_ones);
        }

        res
    }
}

fn main() {
    let s = String::from("011101");
    assert_eq!(5, Solution::max_score(s))
}
