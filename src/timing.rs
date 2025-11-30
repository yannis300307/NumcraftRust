use crate::eadk;

pub struct TimingManager {
    last_timer: u64,
    delta_time: f32,
    frame_time: u64,
}

impl TimingManager {
    pub fn new() -> Self {
        TimingManager {
            last_timer: eadk::time::get_current_time_millis(),
            delta_time: 0.1,
            frame_time: 1,
        }
    }

    pub fn update(&mut self) {
        let current = eadk::time::get_current_time_millis();
        self.frame_time = current - self.last_timer;
        self.delta_time = self.frame_time as f32 / 1000.0;
        self.last_timer = current;
    }

    pub fn get_delta_time(&self) -> f32 {
        self.delta_time
    }

    pub fn get_frame_time(&self) -> u64 {
        self.frame_time
    }
    pub fn get_fps(&self) -> f32 {
        return 1. / self.delta_time;
    }
    pub fn reset(&mut self) {
        self.last_timer = eadk::time::get_current_time_millis();
        self.frame_time = 1;
        self.delta_time = 0.1;
    }
}
