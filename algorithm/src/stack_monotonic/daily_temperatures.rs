#[allow(dead_code)]
struct Solution;
#[allow(dead_code)]
impl Solution {
    // from left to right
    pub fn daily_temperatures(temperatures: Vec<i32>) -> Vec<i32> {
        let n = temperatures.len();
        let mut res = vec![0; n];
        let mut stack: Vec<usize> = Vec::new();

        for i in 0..n {
            while let Some(&top) = stack.last() {
                if temperatures[i] > temperatures[top] {
                    res[top] = (i - top) as i32;
                    stack.pop();
                } else {
                    break;
                }
            }
            stack.push(i);
        }

        res
    }

    // from right to left
    pub fn daily_temperatures_reverse(temperatures: Vec<i32>) -> Vec<i32> {
        let n = temperatures.len();
        let mut res = vec![0; n];
        let mut stack: Vec<usize> = Vec::new();

        for i in (0..n).rev() {
            while let Some(&top) = stack.last() {
                if temperatures[i] >= temperatures[top] {
                    stack.pop();
                } else {
                    break;
                }
            }
            if let Some(&top) = stack.last() {
                res[i] = (top - i) as i32;
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
    fn test_daily_temperatures() {
        let temperatures = vec![73, 74, 75, 71, 69, 72, 76, 73];
        assert_eq!(
            Solution::daily_temperatures(temperatures.clone()),
            vec![1, 1, 4, 2, 1, 1, 0, 0]
        );
        assert_eq!(
            Solution::daily_temperatures_reverse(temperatures.clone()),
            vec![1, 1, 4, 2, 1, 1, 0, 0]
        );
    }
}
