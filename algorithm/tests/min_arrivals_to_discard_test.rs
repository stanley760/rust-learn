
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
}