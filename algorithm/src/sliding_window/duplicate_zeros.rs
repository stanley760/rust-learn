#[allow(dead_code)]
pub struct Solution;

#[allow(unused)]
impl Solution {
    pub fn duplicate_zeros(arr: &mut Vec<i32>) {
        let n = arr.len();
        let (mut i, mut j) = (0, 0);
        while j < n {
            if arr[i] == 0 {
                j += 1;
            }
            i += 1;
            j += 1;
        }
        i -= 1;
        j -= 1;
        while i < j {
            if arr[i] == 0 {
                if j < n {
                    arr[j] = 0;
                }
                j -= 1;
            }
            if j < n {
                arr[j] = arr[i];
            }
            i -= 1;
            j -= 1;
        }
    }
}


#[test]
fn test_normal_case() {
    let mut arr = vec![1,0,2,3,0,4,5,0];
    
    Solution::duplicate_zeros(&mut arr);
    let expect = vec![1,0,0,2,3,0,0,4];

    assert_eq!(arr, expect);

    let mut arr = vec![1,2,3];
    Solution::duplicate_zeros(&mut arr);

    let expect = vec![1,2,3];

    assert_eq!(arr, expect);
}