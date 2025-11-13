// 示例 1:

// 输入: s = "abcabcbb"
// 输出: 3
// 解释: 因为无重复字符的最长子串是
// "abc"
// ，所以其长度为 3。
// 示例 2:

// 输入: s = "bbbbb"
// 输出: 1
// 解释: 因为无重复字符的最长子串是
// "b"
// ，所以其长度为 1。
// 示例 3:

// 输入: s = "pwwkew"
// 输出: 3
// 解释: 因为无重复字符的最长子串是
// "wke"
// ，所以其长度为 3。
//      请注意，你的答案必须是 子串 的长度，
// "pwke"
//  是一个子序列，不是子串。

// 滑动窗口
/// 依次移动右指针 rrr，每次移动时，将 cnt[s[r]] 的值加 1，
/// 然后判断当前窗口 [l,r] 内 cnt[s[r]] 是否大于 1，
/// 如果大于 1，说明当前窗口内有重复字符，我们需要移动左指针 l，
/// 直到窗口内没有重复字符为止。然后，我们更新答案 ans=max⁡(ans,r−l+1)

fn main() {
    let s = "abcabcbb";
    let res = SlidingWindow::length_of_longest_substring(s);
    println!("result 1:{}", res);
    let s = "bbbbb";
    let res = SlidingWindow::length_of_longest_substring(s);
    println!("result 2:{}", res);
    let s = "pwwkew";
    let res = SlidingWindow::length_of_longest_substring(s);
    println!("result 3:{}", res);
}

struct SlidingWindow;

impl SlidingWindow {
    fn length_of_longest_substring(s: &str) -> i32 {
        let mut cnt = vec![0; 128];
        let mut res = 0;
        let mut left = 0;
        s.chars().enumerate().fold(0, |_, (right, c)| {
            cnt[c as usize] += 1;
            while cnt[c as usize] > 1 {
                cnt[s.chars().nth(left).unwrap() as usize] -= 1;
                left += 1;
            }
            res = res.max(right - left + 1);
            res
        }) as i32
    }
}
