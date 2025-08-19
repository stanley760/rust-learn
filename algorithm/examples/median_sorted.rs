// 给定两个大小分别为 m 和 n 的正序（从小到大）数组 nums1 和 nums2。请你找出并返回这两个正序数组的 中位数 。
// 算法的时间复杂度应该为 O(log (m+n)) 。

// 示例 1：

// 输入：nums1 = [1,3], nums2 = [2]
// 输出：2.00000
// 解释：合并数组 = [1,2,3] ，中位数 2
// 示例 2：

// 输入：nums1 = [1,2], nums2 = [3,4]
// 输出：2.50000
// 解释：合并数组 = [1,2,3,4] ，中位数 (2 + 3) / 2 = 2.5

fn main() {
    let nums1 = vec![1, 3];
    let nums2 = vec![2];
    let res = find_median_sorted_arrays(nums1, nums2);
    println!("result 1: {:.5?}", res);
    let nums1 = vec![1, 2, 3];
    let nums2 = vec![3, 4, 5];
    let res = find_median_sorted_arrays(nums1, nums2);
    println!("result 2: {:.5?}", res);
}

/// 给定两个大小分别为 m 和 n 的正序（从小到大）数组 nums1 和 nums2。请你找出并返回这两个正序数组的 中位数 。
/// 算法的时间复杂度应该为 O(log (m+n)) 。
/// 思路：分治,将两个有序数组合并后成一个有序数组，数组1中各个元素顺序位置不会变，数组2也一样。
/// 所以新数组的前半部分一定是数组1前面连续的一部分（可能是空）和数组2前面连续的一部分组成的，
/// 后半部分同理。所以其实只需要知道数组1在新数组前后两部分的切分位置，数组2同理。
fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
    let (n1, n2) = (nums1.len(), nums2.len());
    if n1 > n2 {
        return find_median_sorted_arrays(nums2, nums1);
    }

    let (mut left, mut right) = (0, n1);
    let median = (n1 + n2 + 1) / 2;

    while left <= right {
        let i = (left + right) / 2;
        let j = median - i;

        let nums1_left = if i == 0 { i32::MIN } else { nums1[i - 1] };
        let nums1_right = if i == n1 { i32::MAX } else { nums1[i] };
        let nums2_left = if j == 0 { i32::MIN } else { nums2[j - 1] };
        let nums2_right = if j == n2 { i32::MAX } else { nums2[j] };

        if nums1_left <= nums2_right && nums2_left <= nums1_right {
            if (n1 + n2) % 2 == 0 {
                return (nums1_left.max(nums2_left) + nums1_right.min(nums2_right)) as f64 / 2.0;
            } else {
                return nums1_left.max(nums2_left) as f64;
            }
        } else if nums1_left > nums2_right {
            right = i - 1;
        } else {
            left = i + 1;
        }
    }
    unreachable!()
}
