use std::{sync::Mutex, time::Instant};

struct LeakyBucketLimiterState {
    current_req_num: i32,
    last_time: Instant,
}

pub struct LeakyBucketLimiter {
    pub rate: f64,
    pub capacity: i32,
    state: Mutex<LeakyBucketLimiterState>,
}

impl LeakyBucketLimiter {
    pub fn new(rate: f64, capacity: i32) -> Self {
        Self { 
            rate, 
            capacity, 
            state: Mutex::new(LeakyBucketLimiterState { 
                current_req_num: 0, 
                last_time: Instant::now() }) 
        }
    }

    pub fn allow_request(&self) -> bool {
        let mut stat = self.state.lock().unwrap();
        
        let elapsed_secs = stat.last_time.elapsed().as_secs_f64();
        let leaky_req_count = (elapsed_secs * self.rate) as i32;
    
        if leaky_req_count > 0 {
            stat.current_req_num -= leaky_req_count;
            stat.last_time = Instant::now();
        }
        if stat.current_req_num < 0 {
            stat.current_req_num = 0;
        }

        if stat.current_req_num < self.capacity {
            stat.current_req_num += 1;
            return true;
        }

        false
    }

}

#[allow(unused)]
mod tests {
    use std::{thread, time};

    use crate::limiters::leaky_bucket_limiter::LeakyBucketLimiter;

    #[test]
    pub fn test_leaky_bucket_limiter() {

        println!("================== leaky_bucket_limiter =====================");
        let limiter = LeakyBucketLimiter::new(4f64, 5);
        let total_req = 10;
        for i in 0..total_req  {
            thread::sleep(time::Duration::from_millis(50));
            if limiter.allow_request() {
                println!("第{}个请求通过", i + 1);
            } else {
                println!("第{}个请求被限流", i + 1);
            }
        }
        println!("=================================");
    }
}