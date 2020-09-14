use crate::esc_res::delta_time::DeltaTime;

pub struct VXLClock {
    timer: sdl2::TimerSubsystem,
    last_time: u64,
    now_time: u64,
}

impl VXLClock {
    pub fn new(timer: sdl2::TimerSubsystem) -> VXLClock {
        VXLClock {
            timer,
            last_time: 0,
            now_time: 0,
        }
    }

    pub fn get_delta(&mut self) -> DeltaTime {
        self.last_time = self.now_time;
        self.now_time = self.timer.performance_counter();

        let delta = ((self.now_time - self.last_time) * 1000) as f64
            / self.timer.performance_frequency() as f64;
        DeltaTime { delta }
    }
}
