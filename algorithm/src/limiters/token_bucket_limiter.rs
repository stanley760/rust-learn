use std::{sync::Mutex, time::Instant};

struct TokenBucketLimiterState {
    current_token_num: i32,
    last_time: Instant,
}

pub struct TokenBucketLimiter {
    pub rate: f64,
    pub capacity: i32,
    state: Mutex<TokenBucketLimiterState>,
}

impl TokenBucketLimiter {
    fn new(rate: f64, capacity: i32) -> Self {
        Self {
            rate,
            capacity,
            state: Mutex::new(TokenBucketLimiterState {
                current_token_num: 0,
                last_time: Instant::now(),
            }),
        }
    }

    fn allow_request(&self) -> bool {
        let mut stat = self.state.lock().unwrap();
        let last_time_sec = stat.last_time.elapsed().as_secs_f64();
        let token_count = (last_time_sec * self.rate) as i32;

        if token_count > 0 {
            stat.current_token_num += token_count;
            stat.last_time = Instant::now();
        }

        if stat.current_token_num > self.capacity {
            stat.current_token_num = self.capacity;
        }

        if stat.current_token_num > 0 {
            stat.current_token_num -= 1;
            return true;
        }

        false
    }
}
#[allow(unused)]
mod tests {
    use std::{thread, time::Duration};

    use crate::limiters::token_bucket_limiter::TokenBucketLimiter;

    #[test]
    pub fn test_token_bucket_limiter() {
        println!("-------------------- 令牌桶算法 -------------------------");
        let limiter = TokenBucketLimiter::new(4f64, 5);
        let total_req = 10;

        for i in 0..total_req {
            thread::sleep(Duration::from_millis(50));
            if limiter.allow_request() {
                println!("第{}个请求通过", i + 1);
            } else {
                println!("第{}个请求被限流", i + 1);
            }
        }
        println!("-----------------------------------------------");
    }
}
