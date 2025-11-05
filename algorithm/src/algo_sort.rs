pub fn selection_sort(arr: &mut [i32]) {
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

pub fn bubble_sort(arr: &mut [i32]) {
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

pub fn insertion_sort(arr: &mut [i32]) {
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

pub fn quick_sort(arr: &mut [i32]) {
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

pub fn merge_sort(arr: &mut [i32]) {
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

pub fn heap_sort(arr: &mut [i32]) {
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

pub fn heapify(arr: &mut [i32], heap_size: usize, root_index: usize) {
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

pub fn bucket_sort(arr: &mut [f64]) {
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

pub fn bucket_sort_normal_distribution(arr: &mut [f64]) {
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

pub fn counting_sort(arr: &mut [i32]) {
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

pub fn radix_sort(arr: &mut [i32]) {
    let max_value = *arr.iter().max().unwrap_or(&0);
    let mut exp = 1; // 当前位的权重（1, 10, 100, ...）

    while max_value / exp > 0 {
        counting_sort_by_digit(arr, exp);
        exp *= 10;
    }
}

pub fn counting_sort_by_digit(arr: &mut [i32], exp: i32) {
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

