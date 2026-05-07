use std::sync::Mutex;
use std::time::{Duration, Instant};

struct FixedLimiterState {
    requests: u32,
    last_reset: Instant,
}

pub struct FixedLimiter {
    pub window_size: Duration,
    pub max_request: u32,
    state: Mutex<FixedLimiterState>,
}

impl FixedLimiter {
    pub fn new(window_size: Duration, max_request: u32) -> Self {
        Self {
            window_size,
            max_request,
            state: Mutex::new(FixedLimiterState {
                requests: 0,
                last_reset: Instant::now(),
            }),
        }
    }

    pub fn allow(&self) -> bool {
        let mut state = self.state.lock().unwrap();
        let now = Instant::now();

        if now.duration_since(state.last_reset) >= self.window_size {
            state.requests = 0;
            state.last_reset = now;
        }

        if state.requests < self.max_request {
            state.requests += 1;
            true
        } else {
            false
        }
    }

    pub fn reset(&self) {
        let mut state = self.state.lock().unwrap();
        state.requests = 0;
        state.last_reset = Instant::now();
    }
}

#[allow(unused)]
mod tests {
    use std::{thread, time::{Duration, SystemTime}};
    use crate::limiters::fixed_limiter::FixedLimiter;

    

    #[test]
    fn test_fixed_limiter() {
        let limiter = FixedLimiter::new(Duration::from_secs(1),  3);

        for _ in 0..20 {
            let allowed = limiter.allow();
            let now = SystemTime::now()
                .duration_since(SystemTime::UNIX_EPOCH)
                .unwrap()
                .as_millis();

            if allowed {
                println!("{} 请求通过", now);
            } else {
                println!("{} 请求被限流了", now);
            }
        }
        thread::sleep(Duration::from_millis(100));
    }
}
