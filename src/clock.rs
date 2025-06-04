use std::time::{Duration, Instant};
use tokio::time::sleep;

/// A monotonic clock implementation for testing timeouts and time-based operations
#[derive(Debug, Clone)]
pub struct AbstractMonotonicClock {
    /// The current time in milliseconds
    current_time: u64,
    /// The last time the clock was cranked
    last_crank: Instant,
}

impl AbstractMonotonicClock {
    /// Creates a new monotonic clock starting at time 0
    pub fn new() -> Self {
        Self {
            current_time: 0,
            last_crank: Instant::now(),
        }
    }

    /// Advances the clock by the specified number of milliseconds
    pub fn crank(&mut self, time_ms: u64) {
        self.current_time += time_ms;
    }

    /// Gets the current time in milliseconds
    pub fn get_time(&self) -> u64 {
        self.current_time
    }

    /// Returns a future that completes after the specified number of milliseconds
    pub async fn timeout(&self, time_ms: u64) {
        let target_time = self.current_time + time_ms;
        while self.current_time < target_time {
            sleep(Duration::from_millis(1)).await;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;

    #[test]
    fn test_clock_creation() {
        let clock = AbstractMonotonicClock::new();
        assert_eq!(clock.get_time(), 0);
    }

    #[test]
    fn test_clock_crank() {
        let mut clock = AbstractMonotonicClock::new();
        clock.crank(100);
        assert_eq!(clock.get_time(), 100);
        clock.crank(50);
        assert_eq!(clock.get_time(), 150);
    }

    #[tokio::test]
    async fn test_clock_timeout() {
        let mut clock = AbstractMonotonicClock::new();
        let start = clock.get_time();
        
        // Set a timeout for 100ms
        let timeout_future = clock.timeout(100);
        
        // Crank the clock to trigger the timeout
        clock.crank(100);
        
        // Wait for the timeout
        timeout_future.await;
        
        assert_eq!(clock.get_time(), 100);
    }
} 