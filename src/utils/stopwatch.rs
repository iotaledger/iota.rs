use std::ops::AddAssign;
use std::ops::Sub;
use std::time::{Duration, Instant};

/// Provides a stopwatch for measuring how long things take...
/// this might seem like a weird addition to the library, but
/// it was in Jota...so I put it here. If no one uses this, I'll
/// probably remove it later.
#[derive(Copy, Clone, Debug)]
pub struct StopWatch {
    last_start_time: Instant,
    running: bool,
    duration: Duration,
}

impl Default for StopWatch {
    fn default() -> Self {
        Self::new()
    }
}

impl StopWatch {
    /// Creates a new stopwatch
    pub fn new() -> StopWatch {
        StopWatch {
            last_start_time: Instant::now(),
            running: true,
            duration: Duration::new(0, 0),
        }
    }
    /// Restarts the stopwatch
    pub fn restart(&mut self) {
        self.last_start_time = Instant::now();
        self.running = true;
        self.duration = Duration::new(0, 0);
    }
    /// Stops the stopwatch and returns the duration
    pub fn stop(mut self) -> Duration {
        self.running = false;
        self.duration
            .add_assign(Instant::now().sub(self.last_start_time));
        self.duration
    }
    /// Pauses the stopwatch
    pub fn pause(&mut self) {
        self.running = false;
        self.duration
            .add_assign(Instant::now().sub(self.last_start_time))
    }
    /// Resumes the stopwatch
    pub fn resume(&mut self) {
        self.running = true;
        self.last_start_time = Instant::now();
    }
    /// Returns the elapsed time so far
    pub fn elapsed_time(&self) -> Duration {
        self.duration
    }
    /// Checks whether the stopwatch is running
    pub fn is_running(&self) -> bool {
        self.running
    }
}
