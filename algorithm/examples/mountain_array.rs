struct Solution;

impl Solution {
    pub fn peak_index_in_mountain_array(arr: Vec<i32>) -> i32 {
        let mut left = 0 ;

        let mut right = arr.len() - 2;

        while left + 1 < right {
            let mid = left + (right - left) / 2;
            if arr[mid] < arr[mid + 1] {
                left = mid;
            } else {
                right = mid;
            }
        }
        right as i32
    }
}

 fn main() {
     let arr = vec![0,1,0];
     println!("{:?}", Solution::peak_index_in_mountain_array(arr));
 }