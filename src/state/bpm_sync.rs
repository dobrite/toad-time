use embassy_time::Instant;
use heapless::Vec;

use super::MAX_BPM;

const MICRO_SECONDS_IN_A_SECOND: u64 = 1_000_000;
const SECONDS_IN_A_MINUTE: u64 = 60;
const TOO_LONG_IN_MIRCO_SECONDS: u64 = 5_000_000; // 5 seconds

#[derive(Clone)]
pub struct BpmSync {
    instants: Vec<Instant, 8>,
}

impl Default for BpmSync {
    fn default() -> Self {
        Self::new()
    }
}

impl BpmSync {
    pub fn new() -> Self {
        let now = Instant::now();
        let mut instants = Vec::new();
        instants.push(now).ok();

        Self { instants }
    }

    pub fn pulse(&mut self) -> Option<u32> {
        let now = Instant::now();

        if self.been_too_long(now) {
            self.instants.clear();
            self.instants.push(now).ok();

            return Option::None
        }

        if self.instants.is_full() {
            self.instants.rotate_left(1);
            self.instants.pop();
        }

        self.instants.push(now).ok();

        Option::Some(self.calculate_bpm())
    }

    fn been_too_long(&self, later: Instant) -> bool {
        let earlier = *self.instants.last().unwrap();
        (later - earlier).as_micros() >= TOO_LONG_IN_MIRCO_SECONDS
    }

    fn calculate_bpm(&self) -> u32 {
        let sum = self.sum();
        let len = self.instants.len() as u64;
        let num_windows = len - 1;
        let avg_micros = (sum / num_windows) as f32;
        let beats_per_second = MICRO_SECONDS_IN_A_SECOND as f32 / avg_micros;
        let bpm = beats_per_second * SECONDS_IN_A_MINUTE as f32;

        [bpm as u32, MAX_BPM].into_iter().min().unwrap()
    }

    fn sum(&self) -> u64 {
        self.instants
            .windows(2)
            .map(|window| {
                let earlier = window[0];
                let later = window[1];

                (later - earlier).as_micros()
            })
            .sum()
    }
}
