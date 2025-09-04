fn selection_sort(arr: &mut [i32]) {
    let n = arr.len();
    for i in 0..n - 1 {
        let mut min_index = i;
        for j in (i + 1)..n {
            if arr[j] < arr[min_index] {
                min_index = j; // 找到更小的元素索引
            }
        }
        arr.swap(i, min_index);
        println!("arr: {:?}", arr);
    }
}

fn bubble_sort(arr: &mut [i32]) {
    let n = arr.len();
    for i in (1..n).rev() {
        for j in 0..i {
            if arr[j] > arr[j + 1] {
                arr.swap(j, j + 1);
                println!("arr:{:?}", arr);
            }
        }
    }
}

fn insertion_sort(arr: &mut [i32]) {
    let n = arr.len();
    for i in 1..n {
        let (base, mut j) = (arr[i], (i - 1) as i32);
        while j >= 0 && arr[j as usize] > base {
            // 当前值大于base值时，向右移一位
            // arr.swap((j+ 1) as usize, j as usize);
            arr[(j + 1) as usize] = arr[j as usize];
            j -= 1;
            println!("arr:{:?}", arr);
        }
        // j = -1 , arr[0] = base
        arr[(j + 1) as usize] = base;
    }
}

fn quick_sort(arr: &mut [i32]) {
    let size = arr.len();
    if size <= 1 {
        return;
    }
    let pivot = arr[size / 2];
    let (mut left, mut right) = (0, size - 1);
    while left <= right {
        while arr[left] < pivot {
            left += 1;
        }
        while arr[right] > pivot {
            right -= 1;
        }
        if left <= right {
            arr.swap(left, right);
            left += 1;
            if right == 0 {
                break;
            } // 防止 usize 溢出
            right -= 1;
        }
    }
    quick_sort(&mut arr[0..=right]);
    quick_sort(&mut arr[left..size]);
}

fn merge_sort(arr: &mut [i32]) {
    let n = arr.len();
    if n <= 1 {
        return;
    }
    let mid = n / 2;
    merge_sort(&mut arr[..mid]);
    merge_sort(&mut arr[mid..]);
    let mut merged = Vec::with_capacity(n);
    let (mut left, mut right) = (0, mid);
    while left < mid && right < n {
        if arr[left] <= arr[right] {
            merged.push(arr[left]);
            left += 1;
        } else {
            merged.push(arr[right]);
            right += 1;
        }
    }
    merged.extend_from_slice(&arr[left..mid]);
    merged.extend_from_slice(&arr[right..n]);
    arr.copy_from_slice(&merged);
}

fn heap_sort(arr: &mut [i32]) {
    let n = arr.len();

    // 构建最大堆
    for i in (0..n / 2).rev() {
        heapify(arr, n, i);
    }

    // 一个个从堆中取出元素
    for i in (1..n).rev() {
        arr.swap(0, i); // 将当前最大的放到数组末尾
        heapify(arr, i, 0); // 重新调整堆
    }
}

fn heapify(arr: &mut [i32], heap_size: usize, root_index: usize) {
    let mut largest = root_index;
    let left_child = 2 * root_index + 1;
    let right_child = 2 * root_index + 2;

    if left_child < heap_size && arr[left_child] > arr[largest] {
        largest = left_child;
    }
    if right_child < heap_size && arr[right_child] > arr[largest] {
        largest = right_child;
    }
    if largest != root_index {
        arr.swap(root_index, largest);
        heapify(arr, heap_size, largest);
    }
}

fn bucket_sort(arr: &mut [f64]) {
    let k = arr.len() / 2;

    let mut buckets  = vec![Vec::new(); k];
    for &num in arr.iter() {
        let index = (num * k as f64) as usize;
        buckets[index].push(num);
    }

    for bucket in buckets.iter_mut() {
        bucket.sort_by(|a, b| a.partial_cmp(b).unwrap());
    }

    let mut idx = 0;
    for bucket in buckets {
        for &num in &bucket {
            arr[idx] = num;
            idx += 1;   
        }
    }

}

fn bucket_sort_normal_distribution(arr: &mut [f64]) {
    if arr.is_empty() {
        return;
    }
    
    // 2. 根据数据分布特征确定桶的数量
    // 对于正态分布，我们在较小值区域使用更多的桶
    let mean = arr.iter().sum::<f64>() / arr.len() as f64;
    let variance = arr.iter()
        .map(|&x| (x - mean).powi(2))
        .sum::<f64>() / arr.len() as f64;
    let std_dev = variance.sqrt();

        // 根据正态分布特性确定桶的数量
    let k = (arr.len() as f64).sqrt().ceil() as usize;
    
    // 3. 创建桶并分配元素
    let mut buckets = vec![Vec::new(); k];

    // 3. 使用正态分布的累积分布函数(CDF)来分配桶
    for &num in arr.iter() {
        // 计算z-score
        let z_score = (num - mean) / std_dev;
        // 使用误差函数计算累积概率
        let cdf = 0.5 * (1.0 + erf(z_score / 2.0_f64.sqrt()));
        // 根据概率分配到对应的桶
        let bucket_index = ((cdf * (k - 1) as f64).floor() as usize).min(k - 1);
        buckets[bucket_index].push(num);
    }
    
    // 4. 对每个桶进行排序
    for bucket in buckets.iter_mut() {
        bucket.sort_by(|a, b| a.partial_cmp(b).unwrap());
    }
    
    // 5. 合并结果
    let mut idx = 0;
    for bucket in buckets {
        for &num in &bucket {
            arr[idx] = num;
            idx += 1;
        }
    }
}

// 误差函数的近似实现
fn erf(x: f64) -> f64 {
    // 使用泰勒级数展开的近似
    let a1 = 0.254829592;
    let a2 = -0.284496736;
    let a3 = 1.421413741;
    let a4 = -1.453152027;
    let a5 = 1.061405429;
    let p = 0.3275911;

    let sign = if x < 0.0 { -1.0 } else { 1.0 };
    let x = x.abs();
    
    let t = 1.0 / (1.0 + p * x);
    let y = 1.0 - (((((a5 * t + a4) * t) + a3) * t + a2) * t + a1) * t * (-x * x).exp();
    
    sign * y
}

fn counting_sort(arr: &mut [i32]) {
    let max_value = *arr.iter().max().unwrap_or(&0);
    let mut count = vec![0; (max_value + 1) as usize];

    // 统计每个元素的频率
    for &num in arr.iter() {
        count[num as usize] += 1;
    }

    for i in 0..max_value {
        count[i as usize + 1] += count[i as usize];
    }

    let n  =  arr.len();
    let mut result = vec![0; n];

    for i in (0..n).rev() {
        let num = arr[i];
        result[count[num as usize] - 1] = num;
        count[num as usize] -= 1;
    }
    arr.copy_from_slice(&result);
}

fn radix_sort(arr: &mut [i32]) {
    let max_value = *arr.iter().max().unwrap_or(&0);
    let mut exp = 1; // 当前位的权重（1, 10, 100, ...）

    while max_value / exp > 0 {
        counting_sort_by_digit(arr, exp);
        exp *= 10;
    }
}

fn counting_sort_by_digit(arr: &mut [i32], exp: i32) {
    let n = arr.len();
    let mut count = vec![0; 10]; // 基数为10（十进制）

    // 统计每个数字在当前位上的频率
    for &num in arr.iter() {
        let digit = (num / exp) % 10;
        count[digit as usize] += 1;
    }

    // 累加频率
    for i in 1..10 {
        count[i] += count[i - 1];
    }

    let mut output = vec![0; n];
    for i in (0..n).rev() {
        let num = arr[i];
        let digit = (num / exp) % 10;
        output[count[digit as usize] - 1] = num;
        count[digit as usize] -= 1;
    }

    arr.copy_from_slice(&output);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_selection_sort() {
        let mut arr = [64, 25, 12, 22, 11];
        selection_sort(&mut arr);
        assert_eq!(arr, [11, 12, 22, 25, 64]);
    }

    #[test]
    fn test_bubble_sort() {
        let mut arr = [64, 34, 25, 12, 22, 11, 90];
        bubble_sort(&mut arr);
        assert_eq!(arr, [11, 12, 22, 25, 34, 64, 90]);
    }

    #[test]
    fn test_insertion_sort() {
        let mut arr = [12, 11, 13, 5, 6];
        insertion_sort(&mut arr);
        assert_eq!(arr, [5, 6, 11, 12, 13]);
    }

    #[test]
    fn test_quick_sort() {
        let mut arr = [3, 7, 1, 5, 2];
        quick_sort(&mut arr);
        assert_eq!(arr, [1, 2, 3, 5, 7]);
    }

    #[test]
    fn test_merge_sort() {
        let mut arr = [38, 27, 43, 3, 9, 82, 10];
        merge_sort(&mut arr);
        assert_eq!(arr, [3, 9, 10, 27, 38, 43, 82]);
    }

    #[test]
    fn test_heap_sort() {
        let mut arr = [12, 11, 13, 5, 6, 7];
        heap_sort(&mut arr);
        assert_eq!(arr, [5, 6, 7, 11, 12, 13]);
    }

    #[test]
    fn test_bucket_sort() {
        let mut arr = [0.42, 0.32, 0.23, 0.52, 0.25, 0.47, 0.51];
        bucket_sort(&mut arr);
        assert_eq!(arr, [0.23, 0.25, 0.32, 0.42, 0.47, 0.51, 0.52]);
    }
    
    #[test]
    fn test_bucket_sort_normal_distribution() {
        let mut arr = [0.1, 0.4, 0.35, 0.8, 0.65, 0.2, 0.5, 0.9, 0.3, 0.55];
        bucket_sort_normal_distribution(&mut arr);
        assert_eq!(arr, [0.1, 0.2, 0.3, 0.35, 0.4, 0.5, 0.55, 0.65, 0.8, 0.9]);
    }

    #[test]
    fn test_counting_sort() {
        let mut arr = [4, 2, 2, 8, 3, 3, 1];
        counting_sort(&mut arr);
        assert_eq!(arr, [1, 2, 2, 3, 3, 4, 8]);
    }

    #[test]
    fn test_radix_sort() {
        let mut arr = [1701102, 4576301, 7510231, 9012345, 8027134, 2457901, 2709811, 6611234];
        radix_sort(&mut arr);
        assert_eq!(arr, [1701102, 2457901, 2709811, 4576301, 6611234, 7510231, 8027134, 9012345]);
    }
}
