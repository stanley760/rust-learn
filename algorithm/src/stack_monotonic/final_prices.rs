#[allow(dead_code)]
struct Solution;
#[allow(dead_code)]
impl Solution {
    pub fn final_prices(prices: Vec<i32>) -> Vec<i32> {
        let n = prices.len();
        let mut res = prices.clone();
        let mut stack: Vec<usize> = Vec::new();

        for i in 0..n {
            while let Some(&top) = stack.last() {
                if prices[i] <= prices[top] {
                    res[top] = prices[top] - prices[i];
                    stack.pop();
                } else {
                    break;
                }
            }
            stack.push(i);
        }

        res
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_final_prices() {
        let prices = vec![8, 4, 6, 2, 3];
        assert_eq!(Solution::final_prices(prices), vec![4, 2, 4, 2, 3]);
    }

    #[test]
    fn test_final_prices_2() {
        let prices = vec![1, 2, 3, 4, 5];
        assert_eq!(Solution::final_prices(prices), vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_final_prices_3() {
        let prices = vec![10, 1, 1, 6];
        assert_eq!(Solution::final_prices(prices), vec![9, 0, 1, 6]);
    }
}