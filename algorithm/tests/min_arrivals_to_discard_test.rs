
#[cfg(test)]
mod tests {
    use algorithm::min_arrivals_to_discard::Solution;

    #[test]
    pub fn test_min_arrivals_to_discard() {
        let arrivals = vec![1,2,1,3,1];
        let w = 4;
        let m = 2;
        assert_eq!(Solution::min_arrivals_to_discard(arrivals, w, m), 0);
    }

    #[test]
    pub fn test_min_arrivals_to_discard_2() {
        let arrivals = vec![1,2,3,3,3,4];
        let w = 3;
        let m = 2;
        assert_eq!(Solution::min_arrivals_to_discard(arrivals, w, m), 1);
    }

    #[test]
    pub fn test_min_arrivals_to_discard_3() {
        let arrivals = vec![7,3,9,9,7,3,5,9,7,2,6,10,9,7,9,1,3,6,2,4,6,2,6,8,4,8,2,7,5,6];
        let w = 10;
        let m = 1;
        assert_eq!(Solution::min_arrivals_to_discard(arrivals, w, m), 13);
    }
}