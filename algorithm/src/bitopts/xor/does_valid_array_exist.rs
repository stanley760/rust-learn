#[allow(dead_code)]
struct Solution;
#[allow(dead_code)]
impl Solution {
    // $$
    //   original[i] = \begin{cases}0,\quad i\eq 0 \\ original[i-1] ⊕ derived[i-1],\quad i\gte 1\end{cases}
    //   original[i] = \begin{cases}1,\quad i\eq 0 \\ original[i-1] ⊕ derived[i-1],\quad i\gte 1\end{cases}
    // $$
    pub fn does_valid_array_exist(derived: Vec<i32>) -> bool {
        derived.into_iter().reduce(|xor, x| xor ^ x).unwrap_or(0) == 0
    }

}
#[cfg(test)]
mod tests {
    use crate::xor::does_valid_array_exist::Solution;

    #[test]
    pub fn test_normal_case() {
        let derived = vec![1, 1, 0];
        assert_eq!(Solution::does_valid_array_exist(derived), true);
        let derived = vec![1, 1];
        assert_eq!(Solution::does_valid_array_exist(derived), true);
        let derived = vec![1, 0];
        assert_eq!(Solution::does_valid_array_exist(derived), false);
    }
}