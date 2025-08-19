// 给你一个整数 x ，如果 x 是一个回文整数，返回 true ；否则，返回 false 。

// 回文数是指正序（从左向右）和倒序（从右向左）读都是一样的整数。

// 例如，121 是回文，而 123 不是。
 

// 示例 1：

// 输入：x = 121
// 输出：true
// 示例 2：

// 输入：x = -121
// 输出：false
// 解释：从左向右读, 为 -121 。 从右向左读, 为 121- 。因此它不是一个回文数。

fn main() {
    let x = 121;
    println!("Input: {} result1: {}", x, is_palindrome(x)); // 输出: true

    let y = -121;
    println!("Input: {} result2: {}", y, is_palindrome(y)); // 输出: false

    let z = 10;
    println!("Input: {} result3: {}", z, is_palindrome(z)); // 输出: false
}

pub fn is_palindrome(x: i32) -> bool {
    if x < 0 {
        return false;
    }
    let mut reversed = 0;
    let mut original = x;
    
    while original > 0 {
        reversed = reversed * 10 + original % 10;
        original /= 10;
    }
    
    reversed == x
}