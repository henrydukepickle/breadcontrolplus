use std::time::Instant;

pub struct Timer {
    started: Instant,
    total: f64,
}

impl Timer {
    pub fn new(time: f64) -> Self {
        Self {
            started: Instant::now(),
            total: time,
        }
    }

    pub fn start(&mut self) {
        self.started = Instant::now();
    }

    pub fn set_time(&mut self, time: f64) {
        self.total = time;
    }

    pub fn done(&self) -> bool {
        self.started.elapsed().as_secs_f64() >= self.total
    }
}
