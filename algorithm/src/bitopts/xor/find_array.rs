#[allow(dead_code)]
struct Solution;
#[allow(dead_code)]
impl Solution {
    pub fn find_array(pref: Vec<i32>) -> Vec<i32> {
        let n = pref.len();
        let mut ans = vec![pref[0]; n];
        for x in 1..n {
            let cur = pref[x] ^ pref[x-1];
            ans[x] = cur;
        }
        ans
    }

    pub fn find_array_v1(pref: Vec<i32>) -> Vec<i32> {
        std::iter::once(pref[0])
            .chain(pref.windows(2).map(|w| w[0] ^ w[1]))
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;
    #[test]
    pub fn test_normal_case() {
        let pref = vec![5,2,0,3,1];
        let expected = vec![5,7,2,3,2];
        assert_eq!(Solution::find_array(pref), expected);

        let pref = vec![13];
        let expected = vec![13];
        assert_eq!(Solution::find_array(pref), expected);
    }

    #[test]
    pub fn test_case_1() {
        let pref = vec![5,2,0,3,1];
        let expected = vec![5,7,2,3,2];
        assert_eq!(Solution::find_array_v1(pref), expected);

        let pref = vec![13];
        let expected = vec![13];
        assert_eq!(Solution::find_array_v1(pref), expected);
    }
}
