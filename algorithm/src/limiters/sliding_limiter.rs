use std::{
    sync::Mutex,
    time::{Duration, Instant},
};

struct SlidingLimiterState {
    start_index: usize,
    requests_time_list: Vec<Instant>,
}

pub struct SlidingLimiter {
    pub window_size: Duration,
    pub max_request: usize,

    state: Mutex<SlidingLimiterState>,
}

impl SlidingLimiter {
    pub fn new(window_size: Duration, max_request: usize) -> Self {
        Self {
            window_size,
            max_request,
            state: Mutex::new(SlidingLimiterState {
                requests_time_list: Vec::with_capacity(max_request),
                start_index: 0,
            }),
        }
    }

    pub fn allow_request(&self) -> bool {
        let mut state = self.state.lock().unwrap();
        let current = Instant::now();

        while state.start_index < state.requests_time_list.len() {
            if current.duration_since(state.requests_time_list[state.start_index])
                > self.window_size
            {
                state.start_index += 1;
            } else {
                break;
            }
        }

        let current_requests = state.requests_time_list.len() - state.start_index;

        if current_requests >= self.max_request {
            return false;
        }

        let start_index = state.start_index;
        if start_index > 0 {
            state.requests_time_list.drain(..start_index);
            state.start_index = 0;
        }

        state.requests_time_list.push(current);
        true
    }
}

#[allow(unused)]
mod tests {
    use std::{thread, time::Duration};

    use crate::limiters::sliding_limiter::SlidingLimiter;

    #[test]
    pub fn test_sliding_limiter() {
        let mut limiter = SlidingLimiter::new(Duration::from_secs(1), 3);

        for i in 0..20 {
            if limiter.allow_request() {
                println!("Request-{} allowed", i);
            } else {
                println!("Request-{} denied", i);
            }
            if i % 5 == 0  && i != 0 {
                thread::sleep(Duration::from_secs(1));
            }
         }
    }
}
