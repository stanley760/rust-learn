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

    let n = 5;
    let start = 0;
    println!("Input: {} result4: {}", n, xor_operation(n, start)); // 输出: 8
    
    let nums = vec![1,2,3,1,1,3];
    println!("result5: {}", num_identical_pairs(nums)); // 输出: 4

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

pub fn xor_operation(n: i32, start: i32) -> i32 {
    let arr: Vec<i32> = (0..n).collect();

    let arr = arr.iter().map(|&x| start + 2*x).collect::<Vec<i32>>();
    
    println!("{:?}", arr);


    arr.iter().fold(0, |acc, x| acc ^ x)

}

pub fn num_identical_pairs(nums: Vec<i32>) -> i32 {
    let arr = nums
        .into_iter()
        .fold(vec![0u8; 100], |mut acc, num| {
            acc[num as usize - 1] += 1;
            acc
        });
    println!("{:?}", arr);
        arr.into_iter()
        .filter(|amount| amount.ne(&0))
        .map(|amount| (amount as i32) * (amount as i32 - 1) / 2)
        .sum::<i32>()
}
