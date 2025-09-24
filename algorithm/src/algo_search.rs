
pub fn binary_search<T>(item: &T, array: &[T]) -> Option<usize>
where
    T: Ord,
{
    let mut left = 0;
    let mut right = array.len();

    while left < right {
        let mid = left + (right - left) / 2;
        match item.cmp(&array[mid]) {
            std::cmp::Ordering::Less => {
                right = mid;
            }
            std::cmp::Ordering::Greater => {
                left = mid + 1;
            }
            std::cmp::Ordering::Equal => {
                return Some(mid);
            }
        }
    }
    None
}
