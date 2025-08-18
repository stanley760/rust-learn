
// 示例 1：
// 输入：nums = [2,7,11,15], target = 9
// 输出：[0,1]
// 解释：因为 nums[0] + nums[1] == 9 ，返回 [0, 1] 。

// 示例 2：
// 输入：nums = [3,2,4], target = 6
// 输出：[1,2]

// 示例 3：
// 输入：nums = [3,3], target = 6
// 输出：[0,1]

use std::collections::HashMap;

fn main() {
    let nums = vec!{2,7,11,15};
    let res = HashSolution::two_sum(nums, 9);
    let res1 = HashSolution::two_sum(vec!{3,2,4},6);
    let res2 = HashSolution::two_sum(vec!{3,3},6);
    println!("hash table");
    println!("res1:{:?}", res);
    println!("res2:{:?}", res1);
    println!("res3:{:?}", res2);

}
struct HashSolution;

impl  HashSolution {
     fn two_sum(arr: Vec<i32>, target: i32) -> Vec<i32> {
        let mut map = HashMap::new();

       
        arr.iter().enumerate()
            .find_map(|(i, x)| {
                let y = target -x;
                map.get(&y)
                    .map(|&j| vec![j as i32, i as i32])
                    .or_else(|| {map.insert(x, i);
                            None})
                
            })
            .unwrap_or_default()

    }
}