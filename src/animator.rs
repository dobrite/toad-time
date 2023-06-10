pub struct Animator {
    resolution: u32,
    rate: u32,
    count: u32,
}

impl Animator {
    pub fn new(resolution: u32, rate: u32) -> Self {
        Self {
            resolution,
            rate,
            count: 0,
        }
    }

    pub fn update(&mut self) {
        self.count += 1;
        self.count %= (self.resolution as f32 / self.rate as f32) as u32
    }

    pub fn next_frame(&self) -> bool {
        self.count % ((self.resolution as f32 / self.rate as f32) as u32) == 0
    }
}
