use std::time::{Duration, Instant};

pub struct AttemptRateLimiter {
    failed_attempts: u32,
    lockout_until: Option<Instant>,
}

impl AttemptRateLimiter {
    pub fn new() -> Self {
        Self {
            failed_attempts: 0,
            lockout_until: None,
        }
    }

    pub fn record_failure(&mut self) -> bool {
        self.failed_attempts += 1;
        if self.failed_attempts >= 5 {
            self.lockout_until = Some(Instant::now() + Duration::from_secs(30));
            true
        } else {
            false
        }
    }

    pub fn is_locked(&self) -> bool {
        if let Some(until) = self.lockout_until {
            Instant::now() < until
        } else {
            false
        }
    }
}
