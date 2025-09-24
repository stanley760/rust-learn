#[cfg(test)]
mod tests {
    use algorithm::*;

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
