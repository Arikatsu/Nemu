pub(super) struct FpsTracker {
    pub(super) fps: f32,
    frame_count: u32,
    last_instant: std::time::Instant,
}

impl FpsTracker {
    pub fn new() -> Self {
        Self {
            frame_count: 0,
            last_instant: std::time::Instant::now(),
            fps: 0.0,
        }
    }

    pub fn update(&mut self) {
        self.frame_count += 1;
        let now = std::time::Instant::now();
        let duration = now.duration_since(self.last_instant);
        if duration.as_secs_f32() >= 1.0 {
            self.fps = self.frame_count as f32 / duration.as_secs_f32();
            self.frame_count = 0;
            self.last_instant = now;
        }
    }
    
    pub fn reset(&mut self) {
        self.frame_count = 0;
        self.last_instant = std::time::Instant::now();
        self.fps = 0.0;
    }
}