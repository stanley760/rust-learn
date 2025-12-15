#[allow(dead_code)]
struct Solution;
#[allow(dead_code)]
impl Solution {
    pub fn car_fleet(target: i32, position: Vec<i32>, speed: Vec<i32>) -> i32 {
        let mut ps = position.iter().zip(speed.iter()).collect::<Vec<_>>();
        ps.sort_unstable_by(|a, b|b.0.cmp(a.0));

        let mut st = vec![(target - ps[0].0) as f32 / *ps[0].1 as f32];

        for p in ps {
            let t = (target - p.0) as f32 / *p.1 as f32;
            if t > *st.last().unwrap() {
                st.push(t);
            }
        }
        st.len() as i32
    }
}

#[cfg(test)]
mod tests {
    use crate::stack_monotonic::car_fleet::Solution;


    #[test]
    fn test_normal_case() {
        let target = 12;
        let position = vec![10,8,0,5,3];
        let speed = vec![2,4,1,1,3];

        let x = Solution::car_fleet(target, position, speed);
        assert_eq!(x, 3);

        let target = 10;
        let position = vec![3];
        let speed = vec![3];

        let x = Solution::car_fleet(target, position, speed);
        assert_eq!(x, 1);

        let target = 100;
        let position = vec![0,2,4];
        let speed = vec![4,2,1];

        let x = Solution::car_fleet(target, position, speed);
        assert_eq!(x, 1);
    }


}