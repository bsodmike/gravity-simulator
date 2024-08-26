pub struct TimeConfig {
    ns_per_frame: u64,
    frame_count: u64,
}

impl TimeConfig {
    pub fn new(ns_per_frame: u64) -> TimeConfig {
        TimeConfig {
            ns_per_frame,
            frame_count: 0,
        }
    }

    pub fn advance_frame(&mut self) {
        self.frame_count += 1;
    }

    pub fn get_time(&self) -> u64 {
        self.frame_count * self.ns_per_frame
    }
}
