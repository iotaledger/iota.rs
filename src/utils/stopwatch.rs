use std::time::{Instant, Duration};
use std::ops::AddAssign;
use std::ops::Sub;

pub struct StopWatch {
    last_start_time: Instant,
    running: bool,
    duration: Duration,
}

impl StopWatch {
    pub fn new() -> StopWatch {
        StopWatch {
            last_start_time: Instant::now(),
            running: true,
            duration: Duration::new(0, 0),
        }
    }

    pub fn restart(&mut self) {
        self.last_start_time = Instant::now();
        self.running = true;
        self.duration = Duration::new(0, 0);
    }

    pub fn stop(mut self) -> Duration {
        self.running = false;
        self.duration.add_assign(Instant::now().sub(self.last_start_time));
        self.duration
    }

    pub fn pause(&mut self) {
        self.running = false;
        self.duration.add_assign(Instant::now().sub(self.last_start_time))
    }

    pub fn resume(&mut self) {
        self.running = true;
        self.last_start_time = Instant::now();
    }

    pub fn elapsed_time(&self) -> Duration {
        self.duration
    }

    pub fn is_running(&self) -> bool {
        self.running
    }
}