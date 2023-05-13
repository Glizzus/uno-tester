use std::time::Duration;

#[derive(Copy, Clone)]
pub struct Options {
    pub num_threads: usize,
    pub slow: bool,
    pub verbose: bool,
}

impl Options {
    pub fn new(num_threads: usize, slow: bool, verbose: bool) -> Self {
        Self {
            num_threads,
            slow,
            verbose,
        }
    }

    pub fn default() -> Self {
        Self::new(4, false, false)
    }
}

pub struct GameOptions {
    pub verbose: bool,
    pub turn_pause: Duration,
}
