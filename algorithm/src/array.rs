use rand::Rng;

pub fn random_acces(arr: &[i32]) -> i32 {
    let index = rand::rng().random_range(0..arr.len());
    arr[index]
}

/* 在数组的索引 index 处插入元素 num */
pub fn insert(arr: &mut [i32], target: i32, index: usize) {
    if index >= arr.len() {
        return;
    }
    for i in (index + 1..arr.len()).rev() {
        arr[i] = arr[i - 1];
    }
    arr[index] = target;
}

pub fn remove(arr: &mut [i32], index: usize) {
    if index >= arr.len() {
        return;
    }
    for i in index..arr.len() - 1 {
        arr[i] = arr[i + 1];
    }
}

pub fn traverse(arr: &[i32]) {
    for i in arr {
        println!("{}", i);
    }
}

pub fn find(arr: &[i32], target: i32) -> Option<usize> {
    for (i, &v) in arr.iter().enumerate() {
        if v == target {
            return Some(i);
        }
    }
    None
}

pub fn extend(arr: &[i32], resize: usize) -> Vec<i32> {
    let mut res = vec![0; arr.len() + resize];
    res[..arr.len()].copy_from_slice(arr);

    res
}
