#[cfg(test)]
mod tests {
    use algorithm::max_score::Solution;

    #[test]
    pub fn test_max_score() {
        let card_points = vec![1, 2, 3, 4, 5, 6, 1];
        let k = 3;
        assert_eq!(Solution::max_score(card_points, k), 12);
    }

    #[test]
    pub fn test_max_score_2() {
        let card_points = vec![2, 2, 2];
        let k = 2;
        assert_eq!(Solution::max_score(card_points, k), 4);
    }

    #[test]
    pub fn test_max_score_3() {
        let card_points = vec![9, 7, 7, 9, 7, 7, 9];
        let k = 7;
        assert_eq!(Solution::max_score(card_points, k), 55);
    }

    #[test]
    pub fn test_max_score_4() {
        let card_points = vec![1, 1000, 1];
        let k = 1;
        assert_eq!(Solution::max_score(card_points, k), 1);
    }

    #[test]
    pub fn test_max_score_5() {
        let card_points = vec![1, 79, 80, 1, 1, 1, 200, 1];
        let k = 3;
        assert_eq!(Solution::max_score(card_points, k), 202);
    }
}