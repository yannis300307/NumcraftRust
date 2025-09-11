use crate::eadk;

pub struct TimingManager {
    last_timer: u64,
    delta_time: f32,
}

impl TimingManager {
    pub fn new() -> Self {
        TimingManager {
            last_timer: eadk::timing::millis(),
            delta_time: 0.1,
        }
    }

    pub fn update(&mut self) {
        let current = eadk::timing::millis();
        self.delta_time = (current - self.last_timer) as f32 / 1000.0;
        self.last_timer = current;
    }

    pub fn get_delta_time(&self) -> f32 {
        self.delta_time
    }

    pub fn get_fps(&self) -> f32 {
        return 1. / self.delta_time;
    }
    pub fn reset(&mut self) {
        self.last_timer = eadk::timing::millis();
        self.delta_time = 0.1;
    }
}
