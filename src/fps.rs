use std::time::{Instant, Duration};
use std::collections::VecDeque;

pub struct FpsCounter {
    frame_times: VecDeque<Instant>,
    window_size: usize,
}

impl FpsCounter {
    pub fn new(window_size: usize) -> Self {
        FpsCounter {
            frame_times: VecDeque::with_capacity(window_size),
            window_size,
        }
    }

    pub fn tick(&mut self) {
        let now = Instant::now();
        self.frame_times.push_back(now);

        while self.frame_times.len() > self.window_size {
            self.frame_times.pop_front();
        }
    }

    pub fn fps(&self) -> f64 {
        if self.frame_times.len() < 2 {
            return 0.0;
        }

        let elapsed = self.frame_times.back().unwrap()
            .duration_since(*self.frame_times.front().unwrap());
        let samples = (self.frame_times.len() - 1) as f64;

        if elapsed.as_secs_f64() > 0.0 {
            samples / elapsed.as_secs_f64()
        } else {
            0.0
        }
    }
}

pub struct FrameTimer {
    last_frame_time: Instant,
    target_frame_duration: Option<Duration>,
}

impl FrameTimer {
    pub fn new() -> Self {
        FrameTimer {
            last_frame_time: Instant::now(),
            target_frame_duration: None,
        }
    }

    pub fn set_fps_limit(&mut self, fps: u32) {
        if fps > 0 {
            let duration_micros = 1_000_000 / fps as u64;
            self.target_frame_duration = Some(Duration::from_micros(duration_micros));
        } else {
            self.target_frame_duration = None;
        }
    }

    pub fn wait_frame(&mut self) {
        if let Some(target_duration) = self.target_frame_duration {
            let elapsed = self.last_frame_time.elapsed();
            if elapsed < target_duration {
                let sleep_duration = target_duration - elapsed;
                std::thread::sleep(sleep_duration);
            }
        }
        self.last_frame_time = Instant::now();
    }

    pub fn frame_time(&self) -> Duration {
        self.last_frame_time.elapsed()
    }
}
