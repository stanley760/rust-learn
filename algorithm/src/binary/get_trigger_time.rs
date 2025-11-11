struct Solution;

impl Solution {
     pub fn get_trigger_time(increase: Vec<Vec<i32>>, requirements: Vec<Vec<i32>>) -> Vec<i32> {
        let mut prev_sum = vec![[0; 3]; increase.len() + 1];
        increase.iter().enumerate().for_each(|(i, item)| {
            prev_sum[i + 1][0] = prev_sum[i][0] + item[0];
            prev_sum[i + 1][1] = prev_sum[i][1] + item[1];
            prev_sum[i + 1][2] = prev_sum[i][2] + item[2];
        });
        
        let mut res = vec![-1; requirements.len()];
        for (i, req) in requirements.iter().enumerate() {
            res[i] = prev_sum.partition_point(|&p| p[0] < req[0] || p[1] < req[1] || p[2] < req[2]) as i32;
            if res[i] == prev_sum.len() as i32 {
                res[i] = -1;
            }
        }
        res

     }
}

// cargo test --package algorithm --lib -- binary::get_trigger_time::tests --nocapture
#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_get_trigger_time() {
        let increase = vec![vec![2,8,4],vec![2,5,0],vec![10,9,8]];
        let requirements = vec![vec![2,11,3],vec![15,10,7],vec![9,17,12],vec![8,1,14]];
        assert_eq!(Solution::get_trigger_time(increase.clone(), requirements.clone()), vec![2,-1,3,-1]); 
    }
}